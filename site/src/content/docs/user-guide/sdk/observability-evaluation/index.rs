#![allow(dead_code)]

// --8<-- [start:observability_overview]
use strands::Agent;
use strands::telemetry::{TelemetryExporters, TracerConfig, setup_tracer};

#[tokio::main]
async fn main() -> strands::Result<()> {
    let tracer = setup_tracer(TracerConfig {
        exporters: TelemetryExporters {
            otlp: false,
            console: true,
        },
        ..TracerConfig::default()
    })?;
    tracer.try_init_subscriber()?;

    let mut agent = Agent::builder().build()?;
    agent.invoke("What is agent observability?").await?;
    agent.shutdown().await
}
// --8<-- [end:observability_overview]
