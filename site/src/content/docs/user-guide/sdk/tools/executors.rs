#![allow(dead_code)]

fn concurrent() -> strands::Result<strands::Agent> {
    // --8<-- [start:concurrent]
    #[strands::tool(description = "Return the weather for a city.")]
    async fn weather_tool(city: String) -> Result<String, strands::ToolError> {
        Ok(format!("Sunny in {city}"))
    }

    #[strands::tool(description = "Return the local time for a city.")]
    async fn time_tool(city: String) -> Result<String, strands::ToolError> {
        Ok(format!("12:00 in {city}"))
    }

    strands::Agent::builder()
        .tool_executor(strands::tools::ToolExecutorStrategy::Concurrent)
        .tools(strands::tool_list![WeatherTool, TimeTool])
        .build()
    // --8<-- [end:concurrent]
}

fn sequential() -> strands::Result<strands::Agent> {
    // --8<-- [start:sequential]
    #[strands::tool(description = "Capture a screenshot.")]
    async fn screenshot_tool() -> String {
        "screenshot.png".to_owned()
    }

    #[strands::tool(description = "Email a file.")]
    async fn email_tool(path: String) -> String {
        format!("Sent {path}")
    }

    strands::Agent::builder()
        .tool_executor(strands::tools::ToolExecutorStrategy::Sequential)
        .tools(strands::tool_list![ScreenshotTool, EmailTool])
        .build()
    // --8<-- [end:sequential]
}

fn main() {}
