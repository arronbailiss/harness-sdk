#![allow(dead_code)]

mod keyword {
    // --8<-- [start:keyword]
    use strands::storage::search::{
        KeywordSearchStrategy,
        SearchStrategy,
    };
    use strands::storage::{InMemoryStorage, Storage};

    async fn search() -> strands::Result<()> {
        let storage = InMemoryStorage::new();
        storage.write("notes/theme.md", b"Enable dark mode").await?;
        let results = KeywordSearchStrategy::new()
            .search(&storage, "dark mode")
            .await?;
        let _best_match = results.first();
        Ok(())
    }
    // --8<-- [end:keyword]
}

mod configured {
    // --8<-- [start:configured]
    use std::sync::Arc;

    use strands::storage::search::KeywordSearchStrategy;
    use strands::storage::{
        InMemoryStorage,
        InMemoryStorageConfig,
    };

    fn storage() -> InMemoryStorage {
        let config = InMemoryStorageConfig::builder()
            .search_strategy(Arc::new(KeywordSearchStrategy::new()))
            .build();
        InMemoryStorage::with_config(config)
    }
    // --8<-- [end:configured]
}

mod custom {
    // --8<-- [start:custom]
    use strands::storage::search::{
        SearchStrategy,
        StorageSearchResult,
    };
    use strands::storage::Storage;

    struct PhraseCount;

    #[strands::async_trait]
    impl SearchStrategy for PhraseCount {
        async fn search(
            &self,
            storage: &dyn Storage,
            query: &str,
        ) -> strands::Result<Vec<StorageSearchResult>> {
            let mut results = Vec::new();
            for key in storage.list("").await? {
                let Some(data) = storage.read(&key).await? else {
                    continue;
                };
                let text = String::from_utf8_lossy(&data);
                let score = text.matches(query).count() as f64;
                if score > 0.0 {
                    results.push(
                        StorageSearchResult::new(key, score)
                            .with_data(data),
                    );
                }
            }
            results.sort_by(|a, b| b.score.total_cmp(&a.score));
            Ok(results)
        }
    }
    // --8<-- [end:custom]
}
