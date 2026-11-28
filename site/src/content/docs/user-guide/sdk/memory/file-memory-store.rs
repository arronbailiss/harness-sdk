mod basic {
    // --8<-- [start:basic]
    use std::sync::Arc;

    use strands::memory::{MemoryManagerConfig, MemoryStore};
    use strands::vended_memory_stores::file_memory_store::{
        FileMemoryStore,
        FileMemoryStoreConfig,
    };
    use strands::Agent;

    fn main() -> strands::Result<()> {
        let store = FileMemoryStore::new(
            FileMemoryStoreConfig::new("agent-memory"),
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

mod custom_storage {
    // --8<-- [start:custom_storage]
    use std::sync::Arc;

    use strands::storage::InMemoryStorage;
    use strands::vended_memory_stores::file_memory_store::{
        FileMemoryStore,
        FileMemoryStoreConfig,
    };

    fn main() -> strands::Result<()> {
        let store = FileMemoryStore::new(
            FileMemoryStoreConfig::builder("agent-memory")
                .storage(Arc::new(InMemoryStorage::new()))
                .build(),
        )?;
        let _ = store;
        Ok(())
    }
    // --8<-- [end:custom_storage]
}

mod search_and_add {
    // --8<-- [start:search_and_add]
    use strands::memory::SearchOptions;
    use strands::vended_memory_stores::file_memory_store::{
        FileMemoryStore,
        FileMemoryStoreConfig,
    };

    async fn run() -> strands::Result<()> {
        let store = FileMemoryStore::new(
            FileMemoryStoreConfig::new("agent-memory"),
        )?;
        let key = store
            .add(
                "# Travel preferences\n- Prefers aisle seats",
                None,
            )
            .await?;
        println!("{key}");

        let entries = store
            .search("aisle seats", SearchOptions::default())
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
    use strands::vended_memory_stores::file_memory_store::{
        FileMemoryStore,
        FileMemoryStoreConfig,
    };

    fn main() -> strands::Result<()> {
        let store = FileMemoryStore::new(
            FileMemoryStoreConfig::builder("agent-memory")
                .extraction(true.into())
                .build(),
        )?;
        let _ = store;
        Ok(())
    }
    // --8<-- [end:extraction]
}

mod extraction_custom {
    // --8<-- [start:extraction_custom]
    use std::sync::Arc;

    use strands::models::bedrock::BedrockModel;
    use strands::models::Model;
    use strands::vended_memory_stores::file_memory_store::{
        FileMemoryExtractionConfig,
        FileMemoryStore,
        FileMemoryStoreConfig,
    };

    fn main() -> strands::Result<()> {
        let model: Arc<dyn Model> = Arc::new(
            BedrockModel::builder()
                .model_id(
                    "us.anthropic.claude-haiku-4-5-20251001-v1:0",
                )
                .build(),
        );
        let extraction = FileMemoryExtractionConfig::builder()
            .model(model)
            .system_prompt(
                "Extract durable user preferences. Start each entry \
                 with a markdown heading naming its topic.",
            )
            .build();
        let store = FileMemoryStore::new(
            FileMemoryStoreConfig::builder("agent-memory")
                .extraction(extraction.into())
                .build(),
        )?;
        let _ = store;
        Ok(())
    }
    // --8<-- [end:extraction_custom]
}
