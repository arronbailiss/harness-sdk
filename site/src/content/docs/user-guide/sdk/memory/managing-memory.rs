mod turn_on_writes {
    // --8<-- [start:turn_on_writes]
    use std::sync::Arc;

    use strands::memory::{MemoryManagerConfig, MemoryStore};
    use strands::vended_memory_stores::test_memory_store::{
        TestMemoryStore,
        TestMemoryStoreConfig,
    };
    use strands::Agent;

    fn main() -> strands::Result<()> {
        let store = TestMemoryStore::new(
            TestMemoryStoreConfig::builder("preferences")
                .persist(false)
                .extraction(true.into())
                .build(),
        )?;
        let memory = MemoryManagerConfig {
            stores: vec![Arc::new(store) as Arc<dyn MemoryStore>],
            add_tool_config: Some(true.into()),
            ..MemoryManagerConfig::default()
        };
        let agent = Agent::builder()
            .memory_manager(memory)
            .build()?;
        let _ = agent;
        Ok(())
    }
    // --8<-- [end:turn_on_writes]
}

mod multi_store {
    // --8<-- [start:multi_store]
    use std::sync::Arc;

    use strands::memory::{MemoryManagerConfig, MemoryStore};
    use strands::vended_memory_stores::test_memory_store::{
        TestMemoryStore,
        TestMemoryStoreConfig,
    };
    use strands::Agent;

    fn main() -> strands::Result<()> {
        let personal = TestMemoryStore::new(
            TestMemoryStoreConfig::builder("personal")
                .persist(false)
                .build(),
        )?;
        let team = TestMemoryStore::new(
            TestMemoryStoreConfig::builder("team")
                .persist(false)
                .build(),
        )?;
        let stores: Vec<Arc<dyn MemoryStore>> = vec![
            Arc::new(personal),
            Arc::new(team),
        ];
        let agent = Agent::builder()
            .memory_manager(MemoryManagerConfig {
                stores,
                ..MemoryManagerConfig::default()
            })
            .build()?;
        let _ = agent;
        Ok(())
    }
    // --8<-- [end:multi_store]
}

mod search_tool_config {
    // --8<-- [start:search_tool_config]
    use std::sync::Arc;

    use strands::memory::{
        MemoryAddToolConfig,
        MemoryManagerConfig,
        MemoryStore,
        MemoryToolConfig,
    };
    use strands::vended_memory_stores::test_memory_store::{
        TestMemoryStore,
        TestMemoryStoreConfig,
    };
    use strands::Agent;

    fn main() -> strands::Result<()> {
        let store = TestMemoryStore::new(
            TestMemoryStoreConfig::builder("preferences")
                .persist(false)
                .build(),
        )?;
        let memory = MemoryManagerConfig {
            stores: vec![Arc::new(store) as Arc<dyn MemoryStore>],
            search_tool_config: Some(
                MemoryToolConfig::builder()
                    .name("recall")
                    .description(
                        "Look up what you remember about the user.",
                    )
                    .build()
                    .into(),
            ),
            add_tool_config: Some(
                MemoryAddToolConfig::builder()
                    .wait_for_writes(false)
                    .build()
                    .into(),
            ),
            ..MemoryManagerConfig::default()
        };
        let agent = Agent::builder()
            .memory_manager(memory)
            .build()?;
        let _ = agent;
        Ok(())
    }
    // --8<-- [end:search_tool_config]
}

mod injection_custom {
    // --8<-- [start:injection_custom]
    use std::sync::Arc;

    use strands::injection::InjectionTrigger;
    use strands::memory::{
        MemoryFormat,
        MemoryInjectionConfig,
        MemoryManagerConfig,
        MemoryStore,
    };
    use strands::vended_memory_stores::test_memory_store::{
        TestMemoryStore,
        TestMemoryStoreConfig,
    };
    use strands::Agent;

    fn main() -> strands::Result<()> {
        let store = TestMemoryStore::new(
            TestMemoryStoreConfig::builder("preferences")
                .persist(false)
                .build(),
        )?;
        let format = MemoryFormat::new(|entries| {
            Ok(entries
                .iter()
                .map(|entry| format!("- {}", entry.content))
                .collect::<Vec<_>>()
                .join("\n"))
        });
        let injection = MemoryInjectionConfig {
            trigger: Some(InjectionTrigger::EveryTurn),
            max_entries: Some(3),
            format: Some(format),
            ..MemoryInjectionConfig::default()
        };
        let agent = Agent::builder()
            .memory_manager(MemoryManagerConfig {
                stores: vec![Arc::new(store) as Arc<dyn MemoryStore>],
                injection: Some(injection.into()),
                ..MemoryManagerConfig::default()
            })
            .build()?;
        let _ = agent;
        Ok(())
    }
    // --8<-- [end:injection_custom]
}

mod extraction_defaults {
    // --8<-- [start:extraction_defaults]
    use strands::vended_memory_stores::test_memory_store::{
        TestMemoryStore,
        TestMemoryStoreConfig,
    };

    fn main() -> strands::Result<()> {
        let store = TestMemoryStore::new(
            TestMemoryStoreConfig::builder("preferences")
                .persist(false)
                .extraction(true.into())
                .build(),
        )?;
        let _ = store;
        Ok(())
    }
    // --8<-- [end:extraction_defaults]
}

mod extraction_custom {
    // --8<-- [start:extraction_custom]
    use std::sync::Arc;

    use strands::memory::{
        ExtractionConfig,
        InvocationTrigger,
        ModelExtractor,
        ModelExtractorOptions,
    };
    use strands::models::bedrock::BedrockModel;
    use strands::models::Model;
    use strands::vended_memory_stores::test_memory_store::{
        TestMemoryStore,
        TestMemoryStoreConfig,
    };

    fn main() -> strands::Result<()> {
        let model: Arc<dyn Model> = Arc::new(
            BedrockModel::builder()
                .model_id(
                    "us.anthropic.claude-haiku-4-5-20251001-v1:0",
                )
                .build(),
        );
        let extractor = ModelExtractor::new(
            ModelExtractorOptions::builder()
                .model(model)
                .system_prompt(
                    "Extract durable user preferences as discrete facts.",
                )
                .build(),
        );
        let extraction = ExtractionConfig {
            trigger: Some(vec![Arc::new(InvocationTrigger::new())]),
            extractor: Some(Arc::new(extractor)),
            ..ExtractionConfig::default()
        };
        let store = TestMemoryStore::new(
            TestMemoryStoreConfig::builder("preferences")
                .persist(false)
                .extraction(extraction.into())
                .build(),
        )?;
        let _ = store;
        Ok(())
    }
    // --8<-- [end:extraction_custom]
}

mod custom_trigger {
    // --8<-- [start:custom_trigger]
    use std::sync::Arc;

    use serde_json::json;
    use strands::hooks::AfterInvocationEvent;
    use strands::memory::{
        ExtractionConfig,
        ExtractionTrigger,
        ExtractionTriggerContext,
    };
    use strands::vended_memory_stores::test_memory_store::{
        TestMemoryStore,
        TestMemoryStoreConfig,
    };

    struct CustomTrigger;

    impl ExtractionTrigger for CustomTrigger {
        fn name(&self) -> &str {
            "custom-trigger"
        }

        fn attach(&self, context: ExtractionTriggerContext<'_>) {
            let state = context.agent.app_state().clone();
            let fire = context.fire;
            context.agent.add_hook(
                move |event: &mut AfterInvocationEvent<'_>| {
                    if state.get_value("extract") == Some(json!(true)) {
                        fire.fire(event.invocation_context().cloned());
                    }
                },
            );
        }
    }

    fn main() -> strands::Result<()> {
        let extraction = ExtractionConfig {
            trigger: Some(vec![Arc::new(CustomTrigger)]),
            ..ExtractionConfig::default()
        };
        let store = TestMemoryStore::new(
            TestMemoryStoreConfig::builder("preferences")
                .persist(false)
                .extraction(extraction.into())
                .build(),
        )?;
        let _ = store;
        Ok(())
    }
    // --8<-- [end:custom_trigger]
}

mod flush {
    // --8<-- [start:flush]
    use strands::Agent;

    async fn shutdown(mut agent: Agent) -> strands::Result<()> {
        agent.shutdown().await
    }
    // --8<-- [end:flush]
}

mod programmatic {
    // --8<-- [start:programmatic]
    use serde_json::json;
    use strands::memory::{
        MemoryAddOptions,
        MemoryManager,
        MemorySearchOptions,
    };

    async fn use_memory(
        memory: &MemoryManager,
    ) -> strands::Result<()> {
        let all_results = memory.search("travel plans").await?;
        let scoped = memory
            .search_with(
                "travel plans",
                MemorySearchOptions::builder()
                    .stores(vec!["personal".into()])
                    .max_search_results(5)
                    .build(),
            )
            .await?;
        let metadata = json!({"category": "travel"})
            .as_object()
            .cloned();
        memory
            .add_with(
                "Prefers aisle seats",
                MemoryAddOptions::builder()
                    .stores(vec!["personal".into()])
                    .metadata(metadata.unwrap_or_default())
                    .build(),
            )
            .await?;
        let _ = (all_results, scoped);
        Ok(())
    }
    // --8<-- [end:programmatic]
}

mod custom_store {
    // --8<-- [start:custom_store]
    use std::sync::Mutex;

    use strands::memory::{
        MemoryEntry,
        MemoryStore,
        SearchOptions,
    };
    use strands::{JsonObject, JsonValue};

    #[derive(Default)]
    struct InMemoryStore {
        entries: Mutex<Vec<String>>,
    }

    #[strands::async_trait]
    impl MemoryStore for InMemoryStore {
        fn name(&self) -> &str {
            "preferences"
        }

        fn description(&self) -> Option<&str> {
            Some("User preferences and stable facts.")
        }

        fn writable(&self) -> bool {
            true
        }

        async fn search(
            &self,
            query: &str,
            options: SearchOptions,
        ) -> strands::Result<Vec<MemoryEntry>> {
            let limit = options.max_search_results.unwrap_or(3);
            let entries = self.entries.lock().unwrap();
            Ok(entries
                .iter()
                .filter(|content| content.contains(query))
                .take(limit)
                .cloned()
                .map(MemoryEntry::new)
                .collect())
        }

        fn implements_add(&self) -> bool {
            true
        }

        async fn add(
            &self,
            content: &str,
            _metadata: Option<JsonObject>,
        ) -> strands::Result<JsonValue> {
            self.entries.lock().unwrap().push(content.into());
            Ok(JsonValue::Null)
        }
    }
    // --8<-- [end:custom_store]
}

mod server_side_store {
    // --8<-- [start:server_side_store]
    use strands::memory::{
        AddMessagesContext,
        ExtractionConfig,
        MemoryEntry,
        MemoryStore,
        SearchOptions,
    };
    use strands::{JsonValue, Message};

    struct ServerSideStore;

    #[strands::async_trait]
    impl MemoryStore for ServerSideStore {
        fn name(&self) -> &str {
            "preferences"
        }

        fn writable(&self) -> bool {
            true
        }

        fn extraction(&self) -> Option<ExtractionConfig> {
            Some(ExtractionConfig::default())
        }

        async fn search(
            &self,
            _query: &str,
            _options: SearchOptions,
        ) -> strands::Result<Vec<MemoryEntry>> {
            Ok(Vec::new())
        }

        fn implements_add_messages(&self) -> bool {
            true
        }

        async fn add_messages(
            &self,
            messages: &[Message],
            _context: AddMessagesContext,
        ) -> strands::Result<JsonValue> {
            Ok(serde_json::json!({"accepted": messages.len()}))
        }
    }
    // --8<-- [end:server_side_store]
}
