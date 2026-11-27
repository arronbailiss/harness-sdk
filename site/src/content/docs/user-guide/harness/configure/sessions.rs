#![allow(dead_code)]

async fn harness_resume() -> strands::Result<()> {
    // --8<-- [start:harness_resume]
    use strands_harness::{
        HarnessAgentOptions, SessionConfig, create_harness,
    };

    let session = SessionConfig::builder().id("refactor-parser").build();
    let mut agent = create_harness(
        HarnessAgentOptions::builder().session(session.clone()).build(),
    )
    .await?;
    agent.invoke("Where should we start?").await?;
    agent.shutdown().await?;

    let mut resumed = create_harness(
        HarnessAgentOptions::builder().session(session).build(),
    )
    .await?;
    resumed.invoke("Where did we leave off?").await?;
    // --8<-- [end:harness_resume]
    Ok(())
}

async fn sdk_resume() -> strands::Result<()> {
    // --8<-- [start:sdk_resume]
    use std::sync::Arc;
    use strands::storage::LocalFileStorage;
    use strands::{Agent, SessionManager};

    let storage = Arc::new(LocalFileStorage::new("./sessions"));
    let session = SessionManager::builder()
        .session_id("refactor-parser")
        .storage(storage.clone())
        .build()?;
    let mut agent = Agent::builder().session_manager(session).build()?;
    agent.invoke("Where should we start?").await?;
    agent.shutdown().await?;

    let session = SessionManager::builder()
        .session_id("refactor-parser")
        .storage(storage)
        .build()?;
    let mut resumed = Agent::builder().session_manager(session).build()?;
    resumed.invoke("Where did we leave off?").await?;
    // --8<-- [end:sdk_resume]
    Ok(())
}

async fn harness_dir() -> strands::Result<()> {
    // --8<-- [start:harness_dir]
    use strands_harness::{
        HarnessAgentOptions, SessionConfig, create_harness,
    };

    let session = SessionConfig::builder()
        .id("refactor-parser")
        .dir("/var/lib/agent/sessions")
        .build();
    let _agent = create_harness(
        HarnessAgentOptions::builder().session(session).build(),
    )
    .await?;
    // --8<-- [end:harness_dir]
    Ok(())
}

fn sdk_dir() -> strands::Result<()> {
    // --8<-- [start:sdk_dir]
    use strands::storage::LocalFileStorage;
    use strands::{Agent, SessionManager};

    let session = SessionManager::builder()
        .session_id("refactor-parser")
        .storage(LocalFileStorage::new("/var/lib/agent/sessions"))
        .build()?;
    let _agent = Agent::builder().session_manager(session).build()?;
    // --8<-- [end:sdk_dir]
    Ok(())
}

fn main() {}
