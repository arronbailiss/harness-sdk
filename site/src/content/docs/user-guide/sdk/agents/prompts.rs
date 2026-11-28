#![allow(dead_code)]

// --8<-- [start:system_prompt]
fn system_prompt() -> strands::Result<strands::Agent> {
    strands::Agent::builder()
        .system_prompt(
            "You are a financial advisor specialized in retirement planning. \
             Use tools to gather information and provide personalized advice. \
             Always explain your reasoning and cite sources when possible.",
        )
        .build()
}
// --8<-- [end:system_prompt]

// --8<-- [start:text_prompt]
async fn text_prompt(agent: &mut strands::Agent) -> strands::Result<()> {
    let response = agent.invoke("What is the time in Seattle?").await?;
    println!("{response}");
    Ok(())
}
// --8<-- [end:text_prompt]

// --8<-- [start:multimodal_prompt]
async fn multimodal_prompt(
    agent: &mut strands::Agent,
) -> strands::Result<()> {
    use strands::{ContentBlock, ImageBlock, ImageFormat, ImageSource};

    let image_bytes = std::fs::read("path/to/image.png")?;
    let prompt = vec![
        ContentBlock::text("What can you see in this image?"),
        ImageBlock::new(
            ImageFormat::Png,
            ImageSource::Bytes(image_bytes),
        )
        .into(),
    ];
    agent.invoke(prompt).await?;
    Ok(())
}
// --8<-- [end:multimodal_prompt]

// --8<-- [start:direct_tool_call]
#[strands::tool(description = "Return the current time in a time zone.")]
async fn current_time(timezone: String) -> String {
    format!("The current time in {timezone} is 10:30")
}

async fn direct_tool_call() -> strands::Result<()> {
    let mut agent = strands::Agent::builder()
        .tools([CurrentTime])
        .build()?;
    let result = agent
        .tool("current_time")
        .invoke(serde_json::json!({"timezone": "US/Pacific"}))
        .await?;
    println!("{result:?}");
    Ok(())
}
// --8<-- [end:direct_tool_call]
