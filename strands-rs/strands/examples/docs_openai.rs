//! Compile-checked OpenAI snippet used by the Rust SDK overview.

#![allow(clippy::print_stdout, reason = "a docs example reports to the terminal")]

// --8<-- [start:provider_openai]
use strands::Agent;
use strands::models::openai::OpenAIModel;

#[tokio::main]
async fn main() -> strands::Result<()> {
    let model = OpenAIModel::builder().api_key("<KEY>").model_id("gpt-5.4").build()?;
    let mut agent = Agent::builder().model(model).printer(false).build()?;
    let result = agent.invoke("What can you help me build?").await?;
    println!("{result}");
    agent.shutdown().await
}
// --8<-- [end:provider_openai]
