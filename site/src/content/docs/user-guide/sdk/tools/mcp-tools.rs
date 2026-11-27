fn documentation_server() -> strands::mcp::McpTransport {
    strands::mcp::McpTransport::stdio(strands::mcp::StdioServerParameters::new(
        "uvx",
        ["awslabs.aws-documentation-mcp-server@latest"],
    ))
}

mod quickstart {
    // --8<-- [start:quickstart]
    use strands::mcp::{McpClient, McpTransport, StdioServerParameters};
    use strands::Agent;

    #[tokio::main]
    async fn main() -> strands::Result<()> {
        let transport = McpTransport::stdio(StdioServerParameters::new(
            "uvx",
            ["awslabs.aws-documentation-mcp-server@latest"],
        ));
        let client = McpClient::with_transport(transport);
        let mut agent = Agent::builder().tools([client]).build()?;
        agent.invoke("What is AWS Lambda?").await?;
        agent.shutdown().await
    }
    // --8<-- [end:quickstart]
}

mod integration_approaches {
    // --8<-- [start:integration_approaches]
    use strands::mcp::McpClient;
    use strands::Agent;

    async fn managed(client: McpClient) -> strands::Result<Agent> {
        Agent::builder().tools([client]).build()
    }

    async fn explicit(client: McpClient) -> strands::Result<Agent> {
        let tools = client.list_tools().await?;
        Agent::builder().tools(tools).build()
    }
    // --8<-- [end:integration_approaches]
}

mod multiple_servers {
    // --8<-- [start:multiple_servers]
    use strands::mcp::{McpClient, McpClientConfig};
    use strands::Agent;

    fn agent() -> strands::Result<Agent> {
        let documentation = McpClient::new(McpClientConfig {
            url: Some("https://docs.example.com/mcp".to_owned()),
            ..McpClientConfig::default()
        })?;
        let issues = McpClient::new(McpClientConfig {
            url: Some("https://issues.example.com/mcp".to_owned()),
            ..McpClientConfig::default()
        })?;
        Agent::builder().tools([documentation, issues]).build()
    }
    // --8<-- [end:multiple_servers]
}

mod client_configuration {
    // --8<-- [start:client_configuration]
    use strands::mcp::{
        McpClient, McpClientConfig, McpClientOptions, McpToolFilters, McpTransport,
        StdioServerParameters,
    };

    fn client() -> strands::Result<McpClient> {
        let transport = McpTransport::stdio(StdioServerParameters::new(
            "uvx",
            ["awslabs.aws-documentation-mcp-server@latest"],
        ));
        McpClient::new(McpClientConfig {
            transport: Some(transport),
            options: McpClientOptions {
                prefix: Some("aws_docs".to_owned()),
                tool_filters: Some(McpToolFilters {
                    allowed: Some(vec![
                        "search_documentation".into(),
                        "read_documentation".into(),
                    ]),
                    ..McpToolFilters::default()
                }),
                ..McpClientOptions::default()
            },
            ..McpClientConfig::default()
        })
    }
    // --8<-- [end:client_configuration]
}

mod direct_invocation {
    // --8<-- [start:direct_invocation]
    use serde_json::json;
    use strands::mcp::{McpCallToolOptions, McpClient};
    use strands::Tool;

    async fn calculate(client: &McpClient) -> strands::Result<serde_json::Value> {
        let tools = client.list_tools().await?;
        let calculator = tools
            .iter()
            .find(|tool| tool.name() == "calculator")
            .ok_or_else(|| strands::Error::other("calculator tool not found"))?;
        client
            .call_tool(
                calculator,
                json!({"x": 10, "y": 20}),
                McpCallToolOptions::default(),
            )
            .await
    }
    // --8<-- [end:direct_invocation]
}

mod elicitation {
    // --8<-- [start:elicitation]
    use std::sync::Arc;

    use strands::mcp::rmcp::model::{ElicitResult, ElicitationAction};
    use strands::mcp::{
        ElicitationCallback, McpClient, McpClientConfig, McpClientOptions,
    };

    fn client() -> strands::Result<McpClient> {
        let callback: Arc<dyn ElicitationCallback> =
            Arc::new(|_context, _request| async {
                Ok(ElicitResult::new(ElicitationAction::Accept)
                    .with_content(serde_json::json!({"username": "myname"})))
            });
        McpClient::new(McpClientConfig {
            transport: Some(super::documentation_server()),
            options: McpClientOptions {
                elicitation_callback: Some(callback),
                ..McpClientOptions::default()
            },
            ..McpClientConfig::default()
        })
    }
    // --8<-- [end:elicitation]
}

mod progress {
    // --8<-- [start:progress]
    use strands::mcp::{
        McpClient, McpClientConfig, McpClientOptions, McpProgress,
        McpProgressCallback,
    };

    fn client() -> strands::Result<McpClient> {
        let callback = McpProgressCallback::from(|update: McpProgress| {
            println!(
                "{} / {}: {}",
                update.progress,
                update.total.unwrap_or_default(),
                update.message.unwrap_or_default(),
            );
        });
        McpClient::new(McpClientConfig {
            transport: Some(super::documentation_server()),
            options: McpClientOptions {
                progress_callback: Some(callback),
                ..McpClientOptions::default()
            },
            ..McpClientConfig::default()
        })
    }
    // --8<-- [end:progress]
}
