#![allow(dead_code)]

mod agent_level {
    // --8<-- [start:agent_level]
    use strands::storage::LocalFileStorage;
    use strands::Agent;

    fn agent() -> strands::Result<Agent> {
        Agent::builder()
            .storage(LocalFileStorage::new("./.strands"))
            .printer(false)
            .build()
    }
    // --8<-- [end:agent_level]
}

mod per_plugin {
    // --8<-- [start:per_plugin]
    use strands::session::SessionManager;
    use strands::storage::{InMemoryStorage, LocalFileStorage};
    use strands::Agent;

    fn agent() -> strands::Result<Agent> {
        let sessions = SessionManager::builder()
            .session_id("my-session")
            .storage(LocalFileStorage::new("./sessions"))
            .build()?;
        Agent::builder()
            .storage(InMemoryStorage::new())
            .session_manager(sessions)
            .printer(false)
            .build()
    }
    // --8<-- [end:per_plugin]
}

mod in_memory {
    // --8<-- [start:in_memory]
    use strands::storage::{InMemoryStorage, Storage};

    async fn store() -> strands::Result<()> {
        let storage = InMemoryStorage::new();
        storage.write("notes/today.md", b"Ship the feature").await?;
        let _note = storage.read("notes/today.md").await?;
        Ok(())
    }
    // --8<-- [end:in_memory]
}

mod local_file {
    // --8<-- [start:local_file]
    use strands::storage::{LocalFileStorage, Storage};

    async fn store() -> strands::Result<()> {
        let storage = LocalFileStorage::new("./.strands");
        storage.write("notes/today.md", b"Ship the feature").await?;
        let _note = storage.read("notes/today.md").await?;
        Ok(())
    }
    // --8<-- [end:local_file]
}

mod custom {
    // --8<-- [start:custom]
    use std::collections::BTreeMap;
    use std::sync::Mutex;

    use strands::storage::Storage;

    #[derive(Default)]
    struct MapStorage(Mutex<BTreeMap<String, Vec<u8>>>);

    #[strands::async_trait]
    impl Storage for MapStorage {
        async fn write(&self, key: &str, data: &[u8]) -> strands::Result<()> {
            self.0
                .lock()
                .expect("storage lock")
                .insert(key.to_owned(), data.to_vec());
            Ok(())
        }

        async fn read(&self, key: &str) -> strands::Result<Option<Vec<u8>>> {
            Ok(self
                .0
                .lock()
                .expect("storage lock")
                .get(key)
                .cloned())
        }

        async fn delete(&self, key: &str) -> strands::Result<()> {
            self.0.lock().expect("storage lock").remove(key);
            Ok(())
        }

        async fn list(&self, prefix: &str) -> strands::Result<Vec<String>> {
            Ok(self
                .0
                .lock()
                .expect("storage lock")
                .keys()
                .filter(|key| key.starts_with(prefix))
                .cloned()
                .collect())
        }
    }
    // --8<-- [end:custom]
}
