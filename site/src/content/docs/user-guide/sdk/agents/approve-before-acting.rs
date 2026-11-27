#![allow(dead_code)]

mod classifier {
    // --8<-- [start:classifier]
    use strands::{Agent, tool};
    use strands::vended_interventions::hitl::{
        ClassifierConfig,
        HumanInTheLoop,
        HumanInTheLoopAsk,
        HumanInTheLoopConfig,
    };

    #[tool(description = "List the files in a directory.")]
    async fn list_files(_directory: String) -> Vec<String> {
        vec![
            "app.log".into(),
            "app.log.1".into(),
            "app.log.2".into(),
        ]
    }

    #[tool(description = "Delete the files at the given paths.")]
    async fn delete_files(paths: Vec<String>) -> String {
        format!("Deleted {} files", paths.len())
    }

    async fn run() -> strands::Result<()> {
        let approval = HumanInTheLoop::new(HumanInTheLoopConfig {
            classifier: Some(ClassifierConfig::llm()),
            ask: Some(HumanInTheLoopAsk::Stdio),
            enable_trust: Some(true),
            ..HumanInTheLoopConfig::default()
        });
        let mut agent = Agent::builder()
            .tools(strands::tool_list![ListFiles, DeleteFiles])
            .intervention(approval)
            .build()?;
        agent
            .invoke("Delete the rotated logs in /var/log/app.")
            .await?;
        Ok(())
    }
    // --8<-- [end:classifier]
}

mod cedar {
    // --8<-- [start:cedar]
    use strands::{
        Agent,
        InvocationState,
        InvokeOptions,
        tool,
    };
    use strands::vended_interventions::cedar::{
        CedarAuthorization,
        CedarAuthorizationConfig,
        PrincipalResolver,
        TypeAndId,
    };
    use strands::vended_interventions::hitl::{
        ClassifierConfig,
        HumanInTheLoop,
        HumanInTheLoopAsk,
        HumanInTheLoopConfig,
    };

    #[tool(description = "List the files in a directory.")]
    async fn list_files(_directory: String) -> Vec<String> {
        vec![
            "app.log".into(),
            "app.log.1".into(),
            "app.log.2".into(),
        ]
    }

    #[tool(description = "Delete the files at the given paths.")]
    async fn delete_files(paths: Vec<String>) -> String {
        format!("Deleted {} files", paths.len())
    }

    #[derive(Clone)]
    struct UserId(String);

    async fn invoke_as(
        agent: &mut Agent,
        user_id: &str,
    ) -> strands::Result<()> {
        let state = InvocationState::new();
        state.insert(UserId(user_id.into()));
        agent
            .invoke_with(
                "Delete the rotated logs in /var/log/app.",
                InvokeOptions {
                    invocation_state: Some(state),
                    ..InvokeOptions::default()
                },
            )
            .await?;
        Ok(())
    }

    async fn run() -> strands::Result<()> {
        let resolver = PrincipalResolver::new(
            |state: &InvocationState| {
                Ok(state
                    .get::<UserId>()
                    .map(|user| TypeAndId::new("User", user.0)))
            },
        );
        let cedar = CedarAuthorization::new(
            CedarAuthorizationConfig {
                policies: r#"
                    permit(
                        principal,
                        action == Action::"list_files",
                        resource
                    );
                    permit(
                        principal == User::"alice",
                        action == Action::"delete_files",
                        resource
                    );
                "#
                .into(),
                principal_resolver: Some(resolver),
                ..CedarAuthorizationConfig::default()
            },
        )?;
        let approval = HumanInTheLoop::new(HumanInTheLoopConfig {
            classifier: Some(ClassifierConfig::llm()),
            ask: Some(HumanInTheLoopAsk::Stdio),
            enable_trust: Some(true),
            ..HumanInTheLoopConfig::default()
        });
        let mut agent = Agent::builder()
            .tools(strands::tool_list![ListFiles, DeleteFiles])
            .intervention(cedar)
            .intervention(approval)
            .build()?;

        invoke_as(&mut agent, "alice").await?;
        invoke_as(&mut agent, "bob").await?;
        Ok(())
    }
    // --8<-- [end:cedar]
}
