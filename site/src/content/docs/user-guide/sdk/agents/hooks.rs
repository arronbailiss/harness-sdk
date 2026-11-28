#![allow(dead_code)]
#![allow(clippy::print_stdout)]

mod individual_callback {
    // --8<-- [start:individual_callback]
    use strands::hooks::{BeforeInvocationEvent, BeforeToolCallEvent};
    use strands::Agent;

    fn agent() -> strands::Result<Agent> {
        let agent = Agent::builder().printer(false).build()?;
        agent.add_hook(|_: &mut BeforeInvocationEvent<'_>| {
            println!("Custom callback triggered");
        });
        agent.add_hook(|event: &mut BeforeToolCallEvent| {
            println!("Tool called: {}", event.tool_use().name);
        });
        Ok(agent)
    }
    // --8<-- [end:individual_callback]
}

mod logging_plugin {
    // --8<-- [start:logging_plugin]
    use strands::agent::LocalAgent;
    use strands::hooks::{AfterToolCallEvent, BeforeToolCallEvent};
    use strands::plugins::Plugin;
    use strands::Agent;

    struct LoggingPlugin;

    #[strands::async_trait]
    impl Plugin for LoggingPlugin {
        fn name(&self) -> &str {
            "logging"
        }

        async fn init_agent(&self, agent: &mut LocalAgent) -> strands::Result<()> {
            agent.add_hook(|event: &mut BeforeToolCallEvent| {
                println!("Calling: {}", event.tool_use().name);
            });
            agent.add_hook(|event: &mut AfterToolCallEvent| {
                println!("Completed: {}", event.tool_use().name);
            });
            Ok(())
        }
    }

    fn agent() -> strands::Result<Agent> {
        Agent::builder()
            .plugins([LoggingPlugin])
            .printer(false)
            .build()
    }
    // --8<-- [end:logging_plugin]
}

mod hook_ordering {
    // --8<-- [start:hook_ordering]
    use strands::hooks::{
        BeforeModelCallEvent,
        HookCallbackOptions,
        HookOrder,
    };
    use strands::Agent;

    fn agent() -> strands::Result<Agent> {
        let agent = Agent::builder().printer(false).build()?;
        agent.add_hook_with(
            |_: &mut BeforeModelCallEvent<'_>| println!("I run first"),
            HookCallbackOptions {
                order: HookOrder::SDK_FIRST,
            },
        );
        agent.add_hook_with(
            |_: &mut BeforeModelCallEvent<'_>| println!("I run last"),
            HookCallbackOptions {
                order: HookOrder::SDK_LAST,
            },
        );
        Ok(agent)
    }
    // --8<-- [end:hook_ordering]
}

mod invocation_state {
    // --8<-- [start:invocation_state]
    use strands::hooks::BeforeToolCallEvent;
    use strands::{Agent, InvocationState, InvokeOptions};

    #[derive(Clone)]
    struct RequestContext {
        user_id: String,
        session_id: String,
    }

    async fn run() -> strands::Result<()> {
        let mut agent = Agent::builder().printer(false).build()?;
        agent.add_hook(|event: &mut BeforeToolCallEvent| {
            if let Some(context) =
                event.invocation_state().get::<RequestContext>()
            {
                println!(
                    "user={} session={} tool={}",
                    context.user_id,
                    context.session_id,
                    event.tool_use().name,
                );
            }
        });

        let state = InvocationState::new();
        state.insert(RequestContext {
            user_id: "user123".into(),
            session_id: "sess456".into(),
        });
        agent
            .invoke_with(
                "Process the data",
                InvokeOptions {
                    invocation_state: Some(state),
                    ..InvokeOptions::default()
                },
            )
            .await?;
        agent.shutdown().await
    }
    // --8<-- [end:invocation_state]
}

mod tool_interception {
    // --8<-- [start:tool_interception]
    use std::sync::Arc;

    use strands::hooks::BeforeToolCallEvent;
    use strands::{Agent, Tool, ToolError, tool};

    #[tool(description = "Handle sensitive work with restricted behavior.")]
    async fn safe_tool(input: String) -> Result<String, ToolError> {
        Ok(format!("Safely handled {input}"))
    }

    fn agent() -> strands::Result<Agent> {
        let agent = Agent::builder().printer(false).build()?;
        let replacement: Arc<dyn Tool> = Arc::new(SafeTool);
        agent.add_hook(move |event: &mut BeforeToolCallEvent| {
            if event.tool_use().name == "sensitive_tool" {
                event.set_selected_tool(Arc::clone(&replacement));
                event.tool_use_mut().name = "safe_tool".into();
            }
        });
        Ok(agent)
    }
    // --8<-- [end:tool_interception]
}

mod invocation_message_modification {
    // --8<-- [start:invocation_message_modification]
    use strands::hooks::BeforeInvocationEvent;
    use strands::{Agent, Message};

    fn agent() -> strands::Result<Agent> {
        let agent = Agent::builder().printer(false).build()?;
        agent.add_hook(|event: &mut BeforeInvocationEvent<'_>| {
            event.set_messages(vec![Message::user("[REDACTED]")]);
        });
        Ok(agent)
    }
    // --8<-- [end:invocation_message_modification]
}

mod result_modification {
    // --8<-- [start:result_modification]
    use strands::hooks::AfterToolCallEvent;
    use strands::{Agent, ToolResultBlock, ToolResultContent};

    fn agent() -> strands::Result<Agent> {
        let agent = Agent::builder().printer(false).build()?;
        agent.add_hook(|event: &mut AfterToolCallEvent| {
            if event.tool_use().name == "calculator" {
                event.set_result(ToolResultBlock::success(
                    event.tool_use().tool_use_id.clone(),
                    vec![ToolResultContent::text("Result: [formatted]")],
                ));
            }
        });
        Ok(agent)
    }
    // --8<-- [end:result_modification]
}

mod composability {
    // --8<-- [start:composability]
    use strands::hooks::{
        AfterInvocationEvent,
        BeforeInvocationEvent,
        BeforeToolCallEvent,
    };
    use strands::Agent;

    struct RequestLoggingHooks;

    impl RequestLoggingHooks {
        fn register(&self, agent: &Agent) {
            agent.add_hook(|_: &mut BeforeInvocationEvent<'_>| {
                println!("request started");
            });
            agent.add_hook(|_: &mut AfterInvocationEvent<'_>| {
                println!("request completed");
            });
            agent.add_hook(|event: &mut BeforeToolCallEvent| {
                println!("tool={}", event.tool_use().name);
            });
        }
    }
    // --8<-- [end:composability]
}

mod fixed_tool_arguments {
    // --8<-- [start:fixed_tool_arguments]
    use strands::hooks::BeforeToolCallEvent;
    use strands::{Agent, JsonObject};

    struct ConstantToolArguments {
        tool_name: String,
        values: JsonObject,
    }

    impl ConstantToolArguments {
        fn register(self, agent: &Agent) {
            agent.add_hook(move |event: &mut BeforeToolCallEvent| {
                if event.tool_use().name != self.tool_name {
                    return;
                }
                let Some(input) = event.tool_use_mut().input.as_object_mut()
                else {
                    return;
                };
                input.extend(self.values.clone());
            });
        }
    }
    // --8<-- [end:fixed_tool_arguments]

    // --8<-- [start:fixed_tool_arguments_usage]
    fn agent() -> strands::Result<Agent> {
        use strands::{Agent, JsonObject, JsonValue};

        let agent = Agent::builder().printer(false).build()?;
        let mut values = JsonObject::new();
        values.insert("precision".into(), JsonValue::from(1));
        ConstantToolArguments {
            tool_name: "calculator".into(),
            values,
        }
        .register(&agent);
        Ok(agent)
    }
    // --8<-- [end:fixed_tool_arguments_usage]
}

mod limit_tool_counts {
    // --8<-- [start:limit_tool_counts]
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex, PoisonError};

    use strands::hooks::{BeforeInvocationEvent, BeforeToolCallEvent};
    use strands::Agent;

    struct LimitToolCounts {
        maximums: HashMap<String, usize>,
        counts: Arc<Mutex<HashMap<String, usize>>>,
    }

    impl LimitToolCounts {
        fn register(&self, agent: &Agent) {
            let counts = Arc::clone(&self.counts);
            agent.add_hook(move |_: &mut BeforeInvocationEvent<'_>| {
                counts
                    .lock()
                    .unwrap_or_else(PoisonError::into_inner)
                    .clear();
            });

            let counts = Arc::clone(&self.counts);
            let maximums = self.maximums.clone();
            agent.add_hook(move |event: &mut BeforeToolCallEvent| {
                let name = event.tool_use().name.clone();
                let Some(maximum) = maximums.get(&name) else {
                    return;
                };
                let mut counts =
                    counts.lock().unwrap_or_else(PoisonError::into_inner);
                let count = counts.entry(name.clone()).or_default();
                *count += 1;
                let exceeded = *count > *maximum;
                drop(counts);
                if exceeded {
                    event.set_cancel(format!(
                        "Tool '{name}' has been invoked too many times and is \
                         now being throttled. DO NOT CALL THIS TOOL ANYMORE",
                    ));
                }
            });
        }
    }
    // --8<-- [end:limit_tool_counts]

    // --8<-- [start:limit_tool_counts_usage]
    fn agent() -> strands::Result<Agent> {
        use std::collections::HashMap;
        use std::sync::Arc;

        use strands::Agent;

        let agent = Agent::builder().printer(false).build()?;
        LimitToolCounts {
            maximums: HashMap::from([("sleep".into(), 3)]),
            counts: Arc::default(),
        }
        .register(&agent);
        Ok(agent)
    }
    // --8<-- [end:limit_tool_counts_usage]
}

mod model_call_retry {
    // --8<-- [start:model_call_retry]
    use strands::hooks::AfterModelCallEvent;
    use strands::Agent;

    struct RetryOnModelError {
        max_retries: u32,
    }

    impl RetryOnModelError {
        fn register(self, agent: &Agent) {
            agent.add_hook(move |event: &mut AfterModelCallEvent<'_>| {
                let can_retry =
                    event.error().is_some()
                        && event.attempt_count() <= self.max_retries;
                event.set_retry(can_retry);
            });
        }
    }
    // --8<-- [end:model_call_retry]

    // --8<-- [start:model_call_retry_usage]
    fn agent() -> strands::Result<Agent> {
        use strands::Agent;

        let agent = Agent::builder().printer(false).build()?;
        RetryOnModelError { max_retries: 3 }.register(&agent);
        Ok(agent)
    }
    // --8<-- [end:model_call_retry_usage]
}

mod exception_handling {
    // --8<-- [start:exception_handling]
    use strands::hooks::AfterToolCallEvent;
    use strands::{Agent, BoxError, Error};

    fn agent() -> strands::Result<Agent> {
        let agent = Agent::builder().printer(false).build()?;
        agent.add_hook(
            |event: &mut AfterToolCallEvent| -> Result<(), BoxError> {
                let Some(error) = event.error() else {
                    return Ok(());
                };
                if error.to_string().contains("validation") {
                    return Ok(());
                }
                Err(Error::other(format!(
                    "unexpected tool error: {error}",
                ))
                .into())
            },
        );
        Ok(agent)
    }
    // --8<-- [end:exception_handling]
}

mod tool_call_retry {
    // --8<-- [start:tool_call_retry]
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex, PoisonError};

    use strands::hooks::AfterToolCallEvent;
    use strands::{Agent, ToolResultStatus};

    struct RetryOnToolError {
        max_retries: usize,
        attempts: Arc<Mutex<HashMap<String, usize>>>,
    }

    impl RetryOnToolError {
        fn register(&self, agent: &Agent) {
            let attempts = Arc::clone(&self.attempts);
            let maximum = self.max_retries;
            agent.add_hook(move |event: &mut AfterToolCallEvent| {
                let id = event.tool_use().tool_use_id.clone();
                let failed =
                    event.result().status == ToolResultStatus::Error;
                let mut attempts =
                    attempts.lock().unwrap_or_else(PoisonError::into_inner);
                if !failed {
                    attempts.remove(&id);
                    return;
                }
                let attempt = attempts.entry(id).or_default();
                *attempt += 1;
                event.set_retry(*attempt <= maximum);
            });
        }
    }
    // --8<-- [end:tool_call_retry]

    // --8<-- [start:tool_call_retry_usage]
    fn agent() -> strands::Result<Agent> {
        use std::sync::Arc;

        use strands::Agent;

        let agent = Agent::builder().printer(false).build()?;
        RetryOnToolError {
            max_retries: 1,
            attempts: Arc::default(),
        }
        .register(&agent);
        Ok(agent)
    }
    // --8<-- [end:tool_call_retry_usage]
}

mod resume_once {
    // --8<-- [start:resume_once]
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    use strands::hooks::AfterInvocationEvent;
    use strands::Agent;

    fn agent() -> strands::Result<Agent> {
        let agent = Agent::builder().printer(false).build()?;
        let invocations = Arc::new(AtomicUsize::new(0));
        agent.add_hook(move |event: &mut AfterInvocationEvent<'_>| {
            if invocations.fetch_add(1, Ordering::Relaxed) == 0 {
                event.set_resume(
                    "Now summarize what you just did in one sentence.",
                );
            }
        });
        Ok(agent)
    }
    // --8<-- [end:resume_once]
}

mod iterative_resume {
    // --8<-- [start:iterative_resume]
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    use strands::hooks::AfterInvocationEvent;
    use strands::Agent;

    fn agent() -> strands::Result<Agent> {
        const MAX_ITERATIONS: usize = 3;

        let agent = Agent::builder().printer(false).build()?;
        let iteration = Arc::new(AtomicUsize::new(0));
        agent.add_hook(move |event: &mut AfterInvocationEvent<'_>| {
            let current = iteration.fetch_add(1, Ordering::Relaxed);
            if current < MAX_ITERATIONS {
                event.set_resume(format!(
                    "Improve the response. Iteration {} of {MAX_ITERATIONS}.",
                    current + 1,
                ));
            }
        });
        Ok(agent)
    }
    // --8<-- [end:iterative_resume]
}

mod resume_interrupts {
    // --8<-- [start:resume_interrupts]
    use std::sync::{Arc, Mutex, PoisonError};

    use strands::hooks::{AfterInvocationEvent, InterruptEvent};
    use strands::{Agent, InterruptResponse};

    fn agent() -> strands::Result<Agent> {
        let agent = Agent::builder().printer(false).build()?;
        let pending: Arc<Mutex<Vec<String>>> = Arc::default();

        let observed = Arc::clone(&pending);
        agent.add_hook(move |event: &mut InterruptEvent<'_>| {
            observed
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .push(event.interrupt().id.clone());
        });

        agent.add_hook(move |event: &mut AfterInvocationEvent<'_>| {
            let ids = std::mem::take(
                &mut *pending
                    .lock()
                    .unwrap_or_else(PoisonError::into_inner),
            );
            if !ids.is_empty() {
                let responses = ids
                    .into_iter()
                    .map(|id| InterruptResponse::new(id, "approved"))
                    .collect::<Vec<_>>();
                event.set_resume(responses);
            }
        });
        Ok(agent)
    }
    // --8<-- [end:resume_interrupts]
}

mod reusable_hook {
    // --8<-- [start:reusable_hook]
    use strands::hooks::{
        AfterInvocationEvent,
        BeforeInvocationEvent,
        Hook,
    };
    use strands::{Agent, BoxError};

    #[derive(Clone)]
    struct RequestLogger;

    #[strands::async_trait]
    impl<'a> Hook<BeforeInvocationEvent<'a>> for RequestLogger {
        async fn handle(
            &self,
            _event: &mut BeforeInvocationEvent<'a>,
        ) -> Result<(), BoxError> {
            println!("request started");
            Ok(())
        }
    }

    #[strands::async_trait]
    impl<'a> Hook<AfterInvocationEvent<'a>> for RequestLogger {
        async fn handle(
            &self,
            _event: &mut AfterInvocationEvent<'a>,
        ) -> Result<(), BoxError> {
            println!("request completed");
            Ok(())
        }
    }

    fn agent() -> strands::Result<Agent> {
        let agent = Agent::builder().printer(false).build()?;
        agent.add_hook_impl::<BeforeInvocationEvent<'_>>(
            RequestLogger,
        );
        agent.add_hook_impl::<AfterInvocationEvent<'_>>(
            RequestLogger,
        );
        Ok(agent)
    }
    // --8<-- [end:reusable_hook]
}
