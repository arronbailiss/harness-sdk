#![allow(dead_code)]

mod cancel_timeout {
    // --8<-- [start:cancel_timeout]
    use std::time::Duration;

    use strands::{Agent, CancelOptions, StopReason};

    async fn run() -> strands::Result<()> {
        let mut agent = Agent::builder().build()?;
        let cancel = agent.cancel_handle();
        let watchdog = tokio::spawn(async move {
            tokio::time::sleep(Duration::from_secs(30)).await;
            cancel.cancel(CancelOptions::default());
        });
        let result = agent.invoke("Analyze this large dataset.").await?;
        watchdog.abort();

        if result.stop_reason() == StopReason::Cancelled {
            println!("Agent was cancelled due to timeout");
        }
        Ok(())
    }
    // --8<-- [end:cancel_timeout]
}

mod cancel_external_signal {
    // --8<-- [start:cancel_external_signal]
    use std::time::Duration;

    use strands::{
        Agent,
        CancelHandle,
        CancelOptions,
        InvokeOptions,
        StopReason,
    };

    async fn external(agent: &mut Agent) -> strands::Result<()> {
        let cancel = CancelHandle::new();
        let signal = cancel.signal();
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_secs(30)).await;
            cancel.cancel(CancelOptions::default());
        });

        let result = agent
            .invoke_with(
                "Analyze this large dataset.",
                InvokeOptions {
                    cancel_signal: Some(signal),
                    ..InvokeOptions::default()
                },
            )
            .await?;
        if result.stop_reason() == StopReason::Cancelled {
            println!("Agent was cancelled due to timeout");
        }
        Ok(())
    }
    // --8<-- [end:cancel_external_signal]
}

mod tool_cancellation {
    // --8<-- [start:cancel_signal]
    use strands::{ToolContext, tool};

    #[tool(description = "Process a batch of work.")]
    async fn long_job(
        #[tool(context)] context: &ToolContext,
        chunks: Vec<String>,
    ) -> String {
        for (processed, _chunk) in chunks.into_iter().enumerate() {
            if context.cancel_signal().is_cancelled() {
                return format!("cancelled after {processed} chunks");
            }
        }
        "done".into()
    }
    // --8<-- [end:cancel_signal]
}

mod limits {
    // --8<-- [start:limits_basic]
    use strands::{
        Agent,
        InvokeLimits,
        InvokeOptions,
        StopReason,
    };

    async fn run() -> strands::Result<()> {
        let mut agent = Agent::builder().build()?;
        let result = agent
            .invoke_with(
                "Summarize this document.",
                InvokeOptions {
                    limits: Some(InvokeLimits {
                        turns: Some(5),
                        output_tokens: Some(2_000),
                        total_tokens: Some(10_000),
                        ..InvokeLimits::default()
                    }),
                    ..InvokeOptions::default()
                },
            )
            .await?;

        match result.stop_reason() {
            StopReason::LimitTurns => println!("Hit turn budget"),
            StopReason::LimitTotalTokens => {
                println!("Hit token budget");
            }
            _ => {}
        }
        Ok(())
    }
    // --8<-- [end:limits_basic]
}
