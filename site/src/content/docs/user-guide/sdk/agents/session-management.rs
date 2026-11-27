mod basic_usage {
    // --8<-- [start:basic_usage]
    use std::sync::Arc;

    use strands::storage::LocalFileStorage;
    use strands::{Agent, SessionManager};

    #[tokio::main]
    async fn main() -> strands::Result<()> {
        let storage = Arc::new(LocalFileStorage::new("./sessions"));
        let session = SessionManager::builder()
            .session_id("support-chat")
            .storage(storage)
            .build()?;
        let mut agent = Agent::builder()
            .session_manager(session)
            .printer(false)
            .build()?;

        agent.invoke("Remember that my order number is 4815.").await?;
        agent.shutdown().await
    }
    // --8<-- [end:basic_usage]
}

mod session_id {
    // --8<-- [start:session_id]
    use strands::storage::InMemoryStorage;
    use strands::{Agent, SessionManager};

    fn main() -> strands::Result<()> {
        let session = SessionManager::builder()
            .session_id("user-123")
            .storage(InMemoryStorage::new())
            .build()?;
        let agent = Agent::builder()
            .session_manager(session.clone())
            .printer(false)
            .build()?;

        assert_eq!(agent.session_id(), "user-123");
        assert_eq!(session.session_id(), "user-123");
        Ok(())
    }
    // --8<-- [end:session_id]
}

mod storage_backends {
    // --8<-- [start:storage_backends]
    use strands::storage::{InMemoryStorage, LocalFileStorage};
    use strands::SessionManager;

    fn main() -> strands::Result<()> {
        let local = SessionManager::builder()
            .session_id("durable-session")
            .storage(LocalFileStorage::new("./sessions"))
            .build()?;
        let temporary = SessionManager::builder()
            .session_id("test-session")
            .storage(InMemoryStorage::new())
            .build()?;

        let _ = (local, temporary);
        Ok(())
    }
    // --8<-- [end:storage_backends]
}

mod snapshot_trigger {
    // --8<-- [start:snapshot_trigger]
    use strands::session::SnapshotTriggerCallback;
    use strands::storage::LocalFileStorage;
    use strands::SessionManager;

    fn main() -> strands::Result<()> {
        let every_four_messages = SnapshotTriggerCallback::new(|params| {
            params.agent_data.messages().len() % 4 == 0
        });
        let session = SessionManager::builder()
            .session_id("support-chat")
            .storage(LocalFileStorage::new("./sessions"))
            .snapshot_trigger(every_four_messages)
            .build()?;

        let _ = session;
        Ok(())
    }
    // --8<-- [end:snapshot_trigger]
}

mod list_and_restore {
    // --8<-- [start:list_and_restore]
    use strands::storage::LocalFileStorage;
    use strands::{Agent, SessionManager};

    #[tokio::main]
    async fn main() -> strands::Result<()> {
        let session = SessionManager::builder()
            .session_id("support-chat")
            .storage(LocalFileStorage::new("./sessions"))
            .build()?;
        let mut agent = Agent::builder()
            .session_manager(session.clone())
            .printer(false)
            .build()?;
        agent.initialize().await?;

        session.save_snapshot(agent.local(), false).await?;
        let ids = session.list_snapshot_ids(agent.local(), None, None).await?;
        if let Some(snapshot_id) = ids.last() {
            session
                .restore_snapshot(agent.local_mut(), Some(snapshot_id))
                .await?;
        }
        Ok(())
    }
    // --8<-- [end:list_and_restore]
}

mod delete_session {
    // --8<-- [start:delete_session]
    use strands::storage::LocalFileStorage;
    use strands::SessionManager;

    #[tokio::main]
    async fn main() -> strands::Result<()> {
        let session = SessionManager::builder()
            .session_id("support-chat")
            .storage(LocalFileStorage::new("./sessions"))
            .build()?;
        session.delete_session().await
    }
    // --8<-- [end:delete_session]
}
