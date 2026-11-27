#![allow(dead_code)]

// --8<-- [start:deploy_ready_agent]
use serde::{Deserialize, Serialize};
use strands::Agent;

#[derive(Deserialize)]
struct InvokeRequest {
    prompt: String,
}

#[derive(Serialize)]
struct InvokeResponse {
    output: String,
}

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
}

async fn invoke(
    agent: &mut Agent,
    request: InvokeRequest,
) -> strands::Result<InvokeResponse> {
    let result = agent.invoke(request.prompt).await?;
    Ok(InvokeResponse {
        output: result.to_string(),
    })
}

fn ping() -> HealthResponse {
    HealthResponse { status: "healthy" }
}
// --8<-- [end:deploy_ready_agent]
