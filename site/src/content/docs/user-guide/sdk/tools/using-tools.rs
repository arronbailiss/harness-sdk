#![allow(dead_code)]

async fn adding_tools() -> strands::Result<()> {
    // --8<-- [start:adding_tools]
    #[strands::tool(description = "Create and read named notes.")]
    async fn notebook(mode: String, name: String) -> Result<String, strands::ToolError> {
        Ok(format!("{mode}: {name}"))
    }

    #[strands::tool(description = "Fetch a URL.")]
    async fn http_request(url: String) -> Result<String, strands::ToolError> {
        Ok(format!("Fetched {url}"))
    }

    let mut agent = strands::Agent::builder()
        .tools(strands::tool_list![Notebook, HttpRequest])
        .build()?;
    agent
        .invoke("Create an ideas notebook with three project ideas.")
        .await?;
    agent
        .invoke("Fetch https://example.com and summarize the status.")
        .await?;
    // --8<-- [end:adding_tools]
    Ok(())
}

fn accessing_tools() -> strands::Result<()> {
    // --8<-- [start:accessing_tools]
    #[strands::tool(description = "Create and read named notes.")]
    async fn notebook(mode: String, name: String) -> Result<String, strands::ToolError> {
        Ok(format!("{mode}: {name}"))
    }

    let agent = strands::Agent::builder().tools([Notebook]).build()?;
    let names: Vec<String> = agent
        .tools()
        .iter()
        .map(|tool| tool.name().to_owned())
        .collect();
    println!("{names:?}");
    // --8<-- [end:accessing_tools]
    Ok(())
}

async fn natural_language() -> strands::Result<()> {
    // --8<-- [start:natural_language]
    #[strands::tool(description = "Create and read named notes.")]
    async fn notebook(mode: String, name: String) -> Result<String, strands::ToolError> {
        Ok(format!("{mode}: {name}"))
    }

    let mut agent = strands::Agent::builder().tools([Notebook]).build()?;
    agent.invoke("Read the ideas notebook.").await?;
    // --8<-- [end:natural_language]
    Ok(())
}

async fn direct_call() -> strands::Result<()> {
    // --8<-- [start:direct_call]
    #[strands::tool(description = "Create and read named notes.")]
    async fn notebook(mode: String, name: String) -> Result<String, strands::ToolError> {
        Ok(format!("{mode}: {name}"))
    }

    let mut agent = strands::Agent::builder().tools([Notebook]).build()?;
    let result = agent
        .tool("notebook")
        .invoke(serde_json::json!({
            "mode": "read",
            "name": "ideas"
        }))
        .await?;
    println!("{:?}", result.content);
    // --8<-- [end:direct_call]
    Ok(())
}

fn main() {}
