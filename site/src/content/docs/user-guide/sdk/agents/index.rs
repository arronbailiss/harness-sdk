#![allow(dead_code)]

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
    agent
        .invoke("How many words are in this sentence?")
        .await?;
    agent.shutdown().await
}
// --8<-- [end:agent_anatomy]
