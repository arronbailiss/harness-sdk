#![allow(dead_code)]

// --8<-- [start:example]
use std::collections::BTreeMap;

use strands::mcp::McpServerConfig;
use strands::Agent;
use strands::vended_tools::mcp_router::{
    make_mcp_router,
    MakeMcpRouterOptions,
};

fn agent() -> strands::Result<Agent> {
    let servers = BTreeMap::from([(
        "docs".to_owned(),
        McpServerConfig::builder()
            .url("https://mcp.example.com/mcp")
            .build(),
    )]);
    let router = make_mcp_router(
        MakeMcpRouterOptions::builder()
            .servers(servers)
            .max_connections(5)
            .build(),
    )?;
    Agent::builder().tools([router]).printer(false).build()
}
// --8<-- [end:example]
