#![allow(dead_code)]

async fn move_files() -> strands::Result<()> {
    // --8<-- [start:move_files]
    use strands_harness::{
        HarnessAgentOptions, MemoryConfig, create_harness,
    };

    let memory = MemoryConfig::builder()
        .dir("/var/lib/agent/memory")
        .build();
    let _agent = create_harness(
        HarnessAgentOptions::builder().memory(memory).build(),
    )
    .await?;
    // --8<-- [end:move_files]
    Ok(())
}

async fn swap_store() -> strands::Result<()> {
    // --8<-- [start:swap_store]
    use std::sync::Arc;
    use strands::memory::MemoryStore;
    use strands::storage::LocalFileStorage;
    use strands::vended_memory_stores::file_memory_store::{
        FileMemoryStore, FileMemoryStoreConfig,
    };
    use strands_harness::{
        HarnessAgentOptions, MemoryConfig, create_harness,
    };

    let store = FileMemoryStore::new(
        FileMemoryStoreConfig::builder("team")
            .storage(Arc::new(LocalFileStorage::new("./team-memory")))
            .build(),
    )?;
    let memory = MemoryConfig {
        stores: vec![Arc::new(store) as Arc<dyn MemoryStore>],
        ..MemoryConfig::default()
    };
    let _agent = create_harness(
        HarnessAgentOptions::builder().memory(memory).build(),
    )
    .await?;
    // --8<-- [end:swap_store]
    Ok(())
}

async fn disable() -> strands::Result<()> {
    // --8<-- [start:disable]
    use strands_harness::{HarnessAgentOptions, create_harness};

    let _agent = create_harness(
        HarnessAgentOptions::builder().memory(false).build(),
    )
    .await?;
    // --8<-- [end:disable]
    Ok(())
}

async fn shutdown() -> strands::Result<()> {
    // --8<-- [start:shutdown]
    use strands_harness::{HarnessAgentOptions, create_harness};

    let mut agent = create_harness(HarnessAgentOptions::default()).await?;
    agent.shutdown().await?;
    // --8<-- [end:shutdown]
    Ok(())
}

fn main() {}
