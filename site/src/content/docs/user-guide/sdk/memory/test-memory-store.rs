mod basic {
    // --8<-- [start:basic]
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
    // --8<-- [end:basic]
}

mod persistence {
    // --8<-- [start:persistence]
    use strands::vended_memory_stores::test_memory_store::{
        TestMemoryStore,
        TestMemoryStoreConfig,
    };

    fn main() -> strands::Result<()> {
        let scratch = TestMemoryStore::new(
            TestMemoryStoreConfig::builder("notes")
                .persist(false)
                .build(),
        )?;
        let project = TestMemoryStore::new(
            TestMemoryStoreConfig::builder("notes")
                .path("./notes.json")
                .build(),
        )?;
        let _ = (scratch, project);
        Ok(())
    }
    // --8<-- [end:persistence]
}

mod search_and_add {
    // --8<-- [start:search_and_add]
    use serde_json::json;
    use strands::memory::SearchOptions;
    use strands::vended_memory_stores::test_memory_store::{
        TestMemoryStore,
        TestMemoryStoreConfig,
    };

    async fn run() -> strands::Result<()> {
        let store = TestMemoryStore::new(
            TestMemoryStoreConfig::new("notes"),
        )?;
        let metadata = json!({"category": "travel"})
            .as_object()
            .cloned();
        let result = store
            .add("User prefers aisle seats", metadata)
            .await?;
        println!("{}", result.id);

        let entries = store
            .search(
                "which seats does the user prefer?",
                SearchOptions::default(),
            )
            .await?;
        for entry in entries {
            println!("{}", entry.content);
        }
        Ok(())
    }
    // --8<-- [end:search_and_add]
}

mod extraction {
    // --8<-- [start:extraction]
    use strands::vended_memory_stores::test_memory_store::{
        TestMemoryStore,
        TestMemoryStoreConfig,
    };

    fn main() -> strands::Result<()> {
        let store = TestMemoryStore::new(
            TestMemoryStoreConfig::builder("notes")
                .extraction(true.into())
                .build(),
        )?;
        let _ = store;
        Ok(())
    }
    // --8<-- [end:extraction]
}
