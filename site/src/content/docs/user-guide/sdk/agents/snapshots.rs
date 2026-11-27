#![allow(dead_code)]

mod basic {
    // --8<-- [start:take_snapshot]
    use strands::{Agent, TakeSnapshotOptions};

    async fn run() -> strands::Result<()> {
        let mut agent = Agent::builder()
            .system_prompt("You are a helpful assistant.")
            .build()?;
        agent.invoke("Hello!").await?;
        agent.app_state().set("user_id", "user-123")?;

        let snapshot =
            agent.take_snapshot(TakeSnapshotOptions::session())?;
        println!("{}", snapshot.schema_version);
        println!("{}", snapshot.created_at);
        println!("{:?}", snapshot.data.keys());
        Ok(())
    }
    // --8<-- [end:take_snapshot]
}

mod restore {
    // --8<-- [start:load_snapshot]
    use strands::{Agent, TakeSnapshotOptions};

    async fn restore() -> strands::Result<()> {
        let mut agent = Agent::builder()
            .system_prompt("You are a helpful assistant.")
            .build()?;
        agent.invoke("Hello!").await?;
        let snapshot =
            agent.take_snapshot(TakeSnapshotOptions::session())?;

        agent.invoke("Tell me a joke.").await?;
        agent.invoke("Tell me another one.").await?;
        agent.load_snapshot(&snapshot)?;

        println!("{}", agent.messages().len());
        Ok(())
    }
    // --8<-- [end:load_snapshot]
}

mod fields {
    // --8<-- [start:field_selection]
    use strands::{
        Agent,
        SnapshotField,
        SnapshotPreset,
        TakeSnapshotOptions,
    };

    fn snapshots(agent: &Agent) -> strands::Result<()> {
        let selected = agent.take_snapshot(TakeSnapshotOptions {
            include: vec![SnapshotField::Messages, SnapshotField::State],
            ..TakeSnapshotOptions::default()
        })?;
        let with_prompt = agent.take_snapshot(TakeSnapshotOptions {
            preset: Some(SnapshotPreset::Session),
            include: vec![SnapshotField::SystemPrompt],
            ..TakeSnapshotOptions::default()
        })?;
        let without_interrupts =
            agent.take_snapshot(TakeSnapshotOptions {
                preset: Some(SnapshotPreset::Session),
                exclude: vec![SnapshotField::Interrupts],
                ..TakeSnapshotOptions::default()
            })?;
        let _ = (selected, with_prompt, without_interrupts);
        Ok(())
    }
    // --8<-- [end:field_selection]
}

mod app_data {
    // --8<-- [start:app_data]
    use strands::{Agent, JsonObject, TakeSnapshotOptions};

    fn snapshot(agent: &Agent) -> strands::Result<()> {
        let app_data: JsonObject = serde_json::from_value(
            serde_json::json!({
                "snapshot_label": "After onboarding",
                "workflow_step": 3,
                "user_display_name": "Alice"
            }),
        )?;
        let snapshot = agent.take_snapshot(TakeSnapshotOptions {
            app_data: Some(app_data),
            ..TakeSnapshotOptions::session()
        })?;

        println!("{}", snapshot.app_data["snapshot_label"]);
        println!("{}", snapshot.app_data["user_display_name"]);
        Ok(())
    }
    // --8<-- [end:app_data]
}

mod serialization {
    // --8<-- [start:serialization]
    use strands::{Agent, Snapshot, TakeSnapshotOptions};

    fn save(agent: &Agent) -> strands::Result<()> {
        let snapshot =
            agent.take_snapshot(TakeSnapshotOptions::session())?;
        let json = serde_json::to_string_pretty(&snapshot)?;
        std::fs::write("snapshot.json", json)?;

        let json = std::fs::read_to_string("snapshot.json")?;
        let restored: Snapshot = serde_json::from_str(&json)?;
        let mut new_agent = Agent::builder().build()?;
        new_agent.load_snapshot(&restored)?;
        Ok(())
    }
    // --8<-- [end:serialization]
}

mod checkpoint {
    // --8<-- [start:checkpointing]
    use strands::{Agent, TakeSnapshotOptions};

    async fn run() -> strands::Result<()> {
        let mut agent = Agent::builder()
            .system_prompt("You are a research assistant.")
            .build()?;
        agent
            .invoke("Research the latest trends in AI agents.")
            .await?;
        let checkpoint =
            agent.take_snapshot(TakeSnapshotOptions::session())?;

        agent.invoke("Analyze the key themes and summarize.").await?;
        agent.load_snapshot(&checkpoint)?;
        agent
            .invoke("Focus on multi-agent systems and summarize.")
            .await?;
        Ok(())
    }
    // --8<-- [end:checkpointing]
}

mod branching {
    // --8<-- [start:branching]
    use strands::{Agent, TakeSnapshotOptions};

    async fn run() -> strands::Result<()> {
        let mut agent = Agent::builder()
            .system_prompt("You are a creative writer.")
            .build()?;
        agent
            .invoke("Write the opening paragraph of a mystery novel.")
            .await?;
        let branch_point =
            agent.take_snapshot(TakeSnapshotOptions::session())?;

        agent
            .invoke("Continue in a formal, academic tone.")
            .await?;
        let formal =
            agent.take_snapshot(TakeSnapshotOptions::session())?;

        agent.load_snapshot(&branch_point)?;
        agent
            .invoke("Continue in a casual, conversational tone.")
            .await?;
        let casual =
            agent.take_snapshot(TakeSnapshotOptions::session())?;
        let _ = (formal, casual);
        Ok(())
    }
    // --8<-- [end:branching]
}
