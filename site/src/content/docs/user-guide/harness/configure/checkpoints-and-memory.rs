#![allow(dead_code)]

async fn session() -> strands::Result<()> {
    // --8<-- [start:session]
    use strands_harness::{
        HarnessAgentOptions, SessionConfig, create_harness,
    };

    let session = SessionConfig::builder().id("refactor-parser").build();
    let mut agent = create_harness(
        HarnessAgentOptions::builder().session(session).build(),
    )
    .await?;
    agent.invoke("Where should we start?").await?;
    // --8<-- [end:session]
    Ok(())
}

fn main() {}
