#![allow(dead_code)]
#![allow(clippy::print_stdout)]

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
