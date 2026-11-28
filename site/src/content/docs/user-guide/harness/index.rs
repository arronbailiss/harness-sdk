#![allow(dead_code)]

// --8<-- [start:basic]
use strands_harness::{HarnessAgentOptions, create_harness};

#[tokio::main]
async fn main() -> strands::Result<()> {
    let mut agent = create_harness(HarnessAgentOptions::default()).await?;
    agent
        .invoke(
            "Research the top three vector databases, compare pricing and limits, \
             and write it up in comparison.md",
        )
        .await?;
    agent.shutdown().await
}
// --8<-- [end:basic]
