#![allow(dead_code)]

async fn harness() -> strands::Result<()> {
    // --8<-- [start:harness]
    use strands_harness::{HarnessAgentOptions, create_harness};

    let _agent = create_harness(
        HarnessAgentOptions::builder()
            .mcp_servers("./mcp.json")
            .build(),
    )
    .await?;
    // --8<-- [end:harness]
    Ok(())
}

async fn sdk() -> strands::Result<()> {
    // --8<-- [start:sdk]
    use strands::mcp::{
        McpClient, McpClientOptions, McpLoadServersOptions,
    };
    use strands::Agent;

    let clients = McpClient::load_servers(
        "./mcp.json",
        McpClientOptions::default(),
        McpLoadServersOptions::default(),
    )
    .await?;
    let _agent = Agent::builder().tools(clients).build()?;
    // --8<-- [end:sdk]
    Ok(())
}

fn main() {}
