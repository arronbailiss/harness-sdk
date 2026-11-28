#![allow(dead_code)]

// --8<-- [start:basic]
use strands::Agent;

#[tokio::main]
async fn main() -> strands::Result<()> {
    let mut agent = Agent::builder()
        .system_prompt("You are a concise assistant.")
        .build()?;

    agent
        .invoke("What is an agent harness, in one sentence?")
        .await?;
    agent.shutdown().await
}
// --8<-- [end:basic]
