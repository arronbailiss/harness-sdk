#![allow(dead_code)]

mod input {
    // --8<-- [start:register_input]
    use strands::Agent;
    use strands::middleware::InvokeModelStage;

    fn agent() -> strands::Result<Agent> {
        let agent = Agent::builder().build()?;
        agent.add_middleware(InvokeModelStage::input(
            |mut context| async move {
                context.system_prompt = Some("Be concise.".into());
                Ok(context)
            },
        ));
        Ok(agent)
    }
    // --8<-- [end:register_input]
}

mod output {
    // --8<-- [start:register_output]
    use strands::Agent;
    use strands::middleware::InvokeModelStage;

    fn agent() -> strands::Result<Agent> {
        let agent = Agent::builder().build()?;
        agent.add_middleware(InvokeModelStage::output(
            |result| async move {
                println!(
                    "model stopped with reason {}",
                    result.result.stop_reason,
                );
                Ok(result)
            },
        ));
        Ok(agent)
    }
    // --8<-- [end:register_output]
}

mod wrap {
    // --8<-- [start:register_wrap]
    use std::time::Instant;

    use strands::Agent;
    use strands::middleware::InvokeModelStage;

    fn agent() -> strands::Result<Agent> {
        let agent = Agent::builder().build()?;
        agent.add_middleware(InvokeModelStage::wrap(
            |context, next| {
                let started = Instant::now();
                next.call(context).map_result(move |result| async move {
                    println!("model call took {:?}", started.elapsed());
                    result
                })
            },
        ));
        Ok(agent)
    }
    // --8<-- [end:register_wrap]
}

mod retry {
    // --8<-- [start:retry]
    use std::time::Duration;

    use futures::StreamExt;
    use strands::{Agent, Error};
    use strands::middleware::{
        InvokeModelStage,
        MiddlewareError,
        StageItem,
        StageStream,
    };

    fn agent() -> strands::Result<Agent> {
        let agent = Agent::builder()
            .retry_strategy(Vec::new())
            .build()?;
        agent.add_middleware(InvokeModelStage::wrap(
            |context, next| {
                let stream = next.call(context.clone());
                let state = (0_u32, context, next, stream);
                StageStream::new(futures::stream::unfold(
                    state,
                    |(
                        mut attempt,
                        context,
                        next,
                        mut stream,
                    )| async move {
                        loop {
                            let item = stream.next().await?;
                            let throttled = matches!(
                                &item,
                                StageItem::Done(Err(
                                    MiddlewareError::Error(
                                        Error::ModelThrottled(_),
                                    ),
                                ))
                            );
                            if throttled && attempt < 2 {
                                let delay =
                                    Duration::from_secs(2_u64.pow(attempt));
                                tokio::time::sleep(delay).await;
                                attempt += 1;
                                stream = next.call(context.clone());
                                continue;
                            }
                            let state =
                                (attempt, context, next, stream);
                            return Some((item, state));
                        }
                    },
                ))
            },
        ));
        Ok(agent)
    }
    // --8<-- [end:retry]
}

mod cache {
    // --8<-- [start:cache]
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex, PoisonError};

    use strands::Agent;
    use strands::middleware::{
        InvokeModelResult,
        InvokeModelStage,
        StageStream,
    };

    fn agent() -> strands::Result<Agent> {
        let cache: Arc<Mutex<HashMap<String, InvokeModelResult>>> =
            Arc::default();
        let agent = Agent::builder().build()?;
        agent.add_middleware(InvokeModelStage::wrap(
            move |context, next| {
                let key = serde_json::to_string(&context.messages)
                    .unwrap_or_default();
                if let Some(result) = cache
                    .lock()
                    .unwrap_or_else(PoisonError::into_inner)
                    .get(&key)
                    .cloned()
                {
                    return StageStream::from_result(Ok(result));
                }
                let cache = Arc::clone(&cache);
                next.call(context).map_result(move |result| async move {
                    if let Ok(value) = &result {
                        cache
                            .lock()
                            .unwrap_or_else(PoisonError::into_inner)
                            .insert(key, value.clone());
                    }
                    result
                })
            },
        ));
        Ok(agent)
    }
    // --8<-- [end:cache]
}

mod routing {
    // --8<-- [start:model_routing]
    use std::sync::Arc;

    use strands::models::Model;
    use strands::models::bedrock::BedrockModel;
    use strands::Agent;
    use strands::middleware::InvokeModelStage;

    fn agent() -> strands::Result<Agent> {
        let large: Arc<dyn Model> = Arc::new(
            BedrockModel::builder()
                .model_id("global.anthropic.claude-sonnet-4-6")
                .build(),
        );
        let agent = Agent::builder().build()?;
        agent.add_middleware(InvokeModelStage::input(
            move |mut context| {
                let large = Arc::clone(&large);
                async move {
                    if context.projected_input_tokens.unwrap_or_default()
                        > 50_000
                    {
                        context.model = large;
                    }
                    Ok(context)
                }
            },
        ));
        Ok(agent)
    }
    // --8<-- [end:model_routing]
}

mod gate {
    // --8<-- [start:tool_gate]
    use serde_json::json;
    use strands::Agent;
    use strands::middleware::{
        ExecuteToolResult,
        ExecuteToolStage,
        StageStream,
    };
    use strands::{
        InterruptParams,
        ToolResultBlock,
        ToolResultContent,
    };

    fn agent() -> strands::Result<Agent> {
        let agent = Agent::builder().build()?;
        agent.add_middleware(ExecuteToolStage::wrap(
            |context, next| {
                let reason =
                    format!("Run {}?", context.tool_use.name);
                let approval = context.interrupt(
                    InterruptParams::new("approve_tool")
                        .with_reason(reason),
                );
                match approval {
                    Ok(answer)
                        if answer.response == json!("approved") =>
                    {
                        next.call(context)
                    }
                    Ok(_) => {
                        let result = ToolResultBlock::error(
                            context.tool_use.tool_use_id,
                            vec![ToolResultContent::text(
                                "Denied by reviewer",
                            )],
                        );
                        StageStream::from_result(Ok(
                            ExecuteToolResult::new(result),
                        ))
                    }
                    Err(error) => {
                        StageStream::from_result(Err(error.into()))
                    }
                }
            },
        ));
        Ok(agent)
    }
    // --8<-- [end:tool_gate]
}
