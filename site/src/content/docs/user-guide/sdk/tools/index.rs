#![allow(dead_code)]

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
