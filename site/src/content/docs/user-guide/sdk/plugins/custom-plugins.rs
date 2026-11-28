#![allow(dead_code)]
#![allow(clippy::print_stdout)]

mod using_plugins {
    // --8<-- [start:using_plugins]
    use strands::vended_plugins::todos::Todos;
    use strands::Agent;

    fn agent() -> strands::Result<Agent> {
        Agent::builder()
            .plugins([Todos::default()])
            .printer(false)
            .build()
    }
    // --8<-- [end:using_plugins]
}

mod basic_plugin {
    // --8<-- [start:basic_plugin]
    use strands::agent::LocalAgent;
    use strands::hooks::{AfterToolCallEvent, BeforeToolCallEvent};
    use strands::plugins::Plugin;
    use strands::{tool, Agent, ToolError, ToolSource};

    #[tool(description = "Return a debug message.")]
    async fn debug_print(message: String) -> Result<String, ToolError> {
        Ok(format!("Debug: {message}"))
    }

    struct LoggingPlugin;

    #[strands::async_trait]
    impl Plugin for LoggingPlugin {
        fn name(&self) -> &str {
            "logging"
        }

        async fn init_agent(
            &self,
            agent: &mut LocalAgent,
        ) -> strands::Result<()> {
            agent.add_hook(|event: &mut BeforeToolCallEvent| {
                println!("Calling {}", event.tool_use().name);
            });
            agent.add_hook(|event: &mut AfterToolCallEvent| {
                println!("Completed {}", event.tool_use().name);
            });
            Ok(())
        }

        fn get_tools(&self) -> Vec<ToolSource> {
            vec![DebugPrint.into()]
        }
    }

    fn agent() -> strands::Result<Agent> {
        Agent::builder()
            .plugins([LoggingPlugin])
            .printer(false)
            .build()
    }
    // --8<-- [end:basic_plugin]
}

mod hooks {
    // --8<-- [start:hooks]
    use strands::agent::LocalAgent;
    use strands::hooks::{AfterModelCallEvent, BeforeModelCallEvent};
    use strands::plugins::Plugin;

    struct ModelMonitor;

    #[strands::async_trait]
    impl Plugin for ModelMonitor {
        fn name(&self) -> &str {
            "model-monitor"
        }

        async fn init_agent(
            &self,
            agent: &mut LocalAgent,
        ) -> strands::Result<()> {
            agent.add_hook(|_: &mut BeforeModelCallEvent<'_>| {
                println!("Model call starting");
            });
            agent.add_hook(|_: &mut AfterModelCallEvent<'_>| {
                println!("Model call completed");
            });
            Ok(())
        }
    }
    // --8<-- [end:hooks]
}

mod manual {
    // --8<-- [start:manual]
    use strands::agent::LocalAgent;
    use strands::hooks::BeforeToolCallEvent;
    use strands::plugins::Plugin;

    struct ManualPlugin {
        verbose: bool,
    }

    #[strands::async_trait]
    impl Plugin for ManualPlugin {
        fn name(&self) -> &str {
            "manual"
        }

        async fn init_agent(
            &self,
            agent: &mut LocalAgent,
        ) -> strands::Result<()> {
            if self.verbose {
                agent.add_hook(|event: &mut BeforeToolCallEvent| {
                    println!("Tool input: {}", event.tool_use().input);
                });
            }
            Ok(())
        }
    }
    // --8<-- [end:manual]
}

mod state {
    // --8<-- [start:state]
    use strands::agent::LocalAgent;
    use strands::hooks::BeforeToolCallEvent;
    use strands::plugins::Plugin;

    struct MetricsPlugin;

    #[strands::async_trait]
    impl Plugin for MetricsPlugin {
        fn name(&self) -> &str {
            "metrics"
        }

        async fn init_agent(
            &self,
            agent: &mut LocalAgent,
        ) -> strands::Result<()> {
            agent.app_state().set("metrics_call_count", 0_u64)?;
            agent.add_hook(|event: &mut BeforeToolCallEvent| {
                event
                    .agent()
                    .app_state()
                    .update("metrics_call_count", |count: Option<u64>| {
                        count.unwrap_or(0) + 1
                    })
                    .expect("the counter is a number");
            });
            Ok(())
        }
    }
    // --8<-- [end:state]
}

mod async_init {
    // --8<-- [start:async_init]
    use strands::agent::LocalAgent;
    use strands::plugins::Plugin;

    struct AsyncConfigPlugin;

    #[strands::async_trait]
    impl Plugin for AsyncConfigPlugin {
        fn name(&self) -> &str {
            "async-config"
        }

        async fn init_agent(
            &self,
            agent: &mut LocalAgent,
        ) -> strands::Result<()> {
            let config = tokio::fs::read_to_string("plugin.toml").await?;
            agent.app_state().set("plugin_config", config)?;
            Ok(())
        }
    }
    // --8<-- [end:async_init]
}
