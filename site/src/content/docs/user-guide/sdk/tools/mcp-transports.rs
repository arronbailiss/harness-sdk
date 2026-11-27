mod stdio {
    // --8<-- [start:stdio]
    use strands::mcp::{McpClient, McpTransport, StdioServerParameters};
    use strands::Agent;

    fn agent() -> strands::Result<Agent> {
        let transport = McpTransport::stdio(StdioServerParameters::new(
            "uvx",
            ["awslabs.aws-documentation-mcp-server@latest"],
        ));
        let client = McpClient::with_transport(transport);
        Agent::builder().tools([client]).build()
    }
    // --8<-- [end:stdio]
}

mod streamable_http {
    // --8<-- [start:streamable_http]
    use std::collections::BTreeMap;

    use strands::mcp::{McpClient, McpClientConfig, McpClientCredentials};
    use strands::Agent;

    fn agent() -> strands::Result<Agent> {
        let client = McpClient::new(McpClientConfig {
            url: Some("https://api.example.com/mcp/".to_owned()),
            auth: Some(McpClientCredentials {
                client_id: "<OAUTH_CLIENT_ID>".to_owned(),
                client_secret: "<OAUTH_CLIENT_SECRET>".to_owned(),
                scopes: Some(vec!["mcp:tools".to_owned()]),
                ..McpClientCredentials::default()
            }),
            headers: Some(BTreeMap::from([(
                "X-Client-Name".to_owned(),
                "support-agent".to_owned(),
            )])),
            ..McpClientConfig::default()
        })?;
        Agent::builder().tools([client]).build()
    }
    // --8<-- [end:streamable_http]
}

mod sse {
    // --8<-- [start:sse]
    use strands::mcp::{McpClient, McpTransport};
    use strands::Agent;

    fn agent() -> strands::Result<Agent> {
        let transport = McpTransport::sse("http://localhost:8000/sse", None)?;
        let client = McpClient::with_transport(transport);
        Agent::builder().tools([client]).build()
    }
    // --8<-- [end:sse]
}
