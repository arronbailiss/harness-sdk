mod getting_started {
    // --8<-- [start:getting_started]
    use std::sync::Arc;

    use strands::memory::{MemoryManagerConfig, MemoryStore};
    use strands::vended_memory_stores::test_memory_store::{
        TestMemoryStore,
        TestMemoryStoreConfig,
    };
    use strands::Agent;

    fn main() -> strands::Result<()> {
        let store = TestMemoryStore::new(
            TestMemoryStoreConfig::new("notes"),
        )?;
        let memory = MemoryManagerConfig {
            stores: vec![Arc::new(store) as Arc<dyn MemoryStore>],
            ..MemoryManagerConfig::default()
        };
        let agent = Agent::builder()
            .memory_manager(memory)
            .build()?;
        let _ = agent;
        Ok(())
    }
    // --8<-- [end:getting_started]
}
