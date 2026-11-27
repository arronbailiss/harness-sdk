//! Compile-checked Anthropic snippet used by the Rust SDK overview.

#![allow(clippy::print_stdout, reason = "a docs example reports to the terminal")]

// --8<-- [start:provider_anthropic]
use strands::Agent;
use strands::models::anthropic::AnthropicModel;

#[tokio::main]
async fn main() -> strands::Result<()> {
    let model = AnthropicModel::builder()
        .api_key("<KEY>")
        .model_id("claude-sonnet-5")
        .build()?;
    let mut agent = Agent::builder().model(model).printer(false).build()?;
    let result = agent.invoke("What can you help me build?").await?;
    println!("{result}");
    agent.shutdown().await
}
// --8<-- [end:provider_anthropic]
