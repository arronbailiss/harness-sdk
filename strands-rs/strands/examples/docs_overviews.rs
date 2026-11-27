//! Compile-checked snippets used by the documentation overview pages.

#![allow(clippy::print_stdout, dead_code, reason = "documentation examples")]

mod agent_anatomy {
    // --8<-- [start:agent_anatomy]
    use strands::{Agent, ToolError, tool};

    #[tool(description = "Count the words in a piece of text.")]
    async fn word_count(text: String) -> Result<String, ToolError> {
        Ok(format!("{} words", text.split_whitespace().count()))
    }

    #[tokio::main]
    async fn main() -> strands::Result<()> {
        let mut agent = Agent::builder()
            .system_prompt("You are a concise writing assistant.")
            .tools([WordCount])
            .build()?;
        agent.invoke("How many words are in this sentence?").await?;
        agent.shutdown().await
    }
    // --8<-- [end:agent_anatomy]
}

mod tool_overview {
    // --8<-- [start:tool_overview]
    use strands::{Agent, ToolError, tool};

    #[tool(description = "Get the current weather for a city.")]
    async fn get_weather(city: String) -> Result<String, ToolError> {
        Ok(format!("It's sunny in {city}."))
    }

    #[tokio::main]
    async fn main() -> strands::Result<()> {
        let mut agent = Agent::builder().tools([GetWeather]).build()?;
        agent.invoke("What's the weather in Seattle?").await?;
        agent.shutdown().await
    }
    // --8<-- [end:tool_overview]
}

mod streaming_overview {
    // --8<-- [start:streaming_overview]
    use std::io::{self, Write};

    use futures::StreamExt;
    use strands::{Agent, AgentStreamEvent};

    #[tokio::main]
    async fn main() -> strands::Result<()> {
        let mut agent = Agent::builder().printer(false).build()?;

        {
            let mut stream = agent.stream("Tell me about agents in one sentence.");
            while let Some(event) = stream.next().await {
                if let AgentStreamEvent::ModelStreamUpdate(update) = event?
                    && let Some(text) = update.text_delta()
                {
                    print!("{text}");
                    io::stdout().flush()?;
                }
            }
        }

        agent.shutdown().await
    }
    // --8<-- [end:streaming_overview]
}

mod observability_overview {
    // --8<-- [start:observability_overview]
    use strands::Agent;
    use strands::telemetry::{TelemetryExporters, TracerConfig, setup_tracer};

    #[tokio::main]
    async fn main() -> strands::Result<()> {
        let tracer = setup_tracer(TracerConfig {
            exporters: TelemetryExporters {
                otlp: false,
                console: true,
            },
            ..TracerConfig::default()
        })?;
        tracer.try_init_subscriber()?;

        let mut agent = Agent::builder().build()?;
        agent.invoke("What is agent observability?").await?;
        agent.shutdown().await
    }
    // --8<-- [end:observability_overview]
}

fn main() {}
