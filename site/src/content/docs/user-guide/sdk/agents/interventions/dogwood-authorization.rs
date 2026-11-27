mod basic {
    // --8<-- [start:basic]
    use strands::types::ToolSpec;
    use strands::vended_interventions::dogwood::{
        DogwoodAuthorization,
        DogwoodAuthorizationConfig,
    };
    use strands::Agent;

    fn main() -> strands::Result<()> {
        let search = ToolSpec::new("search", "Search for information");
        let dogwood = DogwoodAuthorization::new(
            DogwoodAuthorizationConfig {
                policies: r#"
                    permit(
                        principal,
                        action == Action::"search",
                        resource
                    );
                "#
                .into(),
                tools: Some(vec![search]),
                ..DogwoodAuthorizationConfig::default()
            },
        )?;
        let agent = Agent::builder()
            .interventions([dogwood])
            .printer(false)
            .build()?;
        let _ = agent;
        Ok(())
    }
    // --8<-- [end:basic]
}

mod temporal {
    // --8<-- [start:temporal]
    use strands::types::ToolSpec;
    use strands::vended_interventions::dogwood::{
        DogwoodAuthorization,
        DogwoodAuthorizationConfig,
    };
    use strands::{Agent, SequentialToolExecutor};

    fn main() -> strands::Result<()> {
        let tools = vec![
            ToolSpec::new("search", "Search for information"),
            ToolSpec::new("delete_record", "Delete a record"),
        ];
        let dogwood = DogwoodAuthorization::new(
            DogwoodAuthorizationConfig {
                policies: r#"
                    permit(
                        principal,
                        action == Action::"search",
                        resource
                    );
                    permit(
                        principal,
                        action == Action::"delete_record",
                        resource
                    ) when temporal {
                        formerly within 1h
                            Action::"search"::response{}
                    };
                "#
                .into(),
                tools: Some(tools),
                ..DogwoodAuthorizationConfig::default()
            },
        )?;
        let agent = Agent::builder()
            .interventions([dogwood])
            .tool_executor(SequentialToolExecutor::new())
            .printer(false)
            .build()?;
        let _ = agent;
        Ok(())
    }
    // --8<-- [end:temporal]
}

mod identity {
    // --8<-- [start:identity]
    use strands::agent::InvocationState;
    use strands::types::{JsonObject, ToolSpec};
    use strands::vended_interventions::dogwood::{
        ContextEnricher,
        ContextEnricherInput,
        DogwoodAuthorization,
        DogwoodAuthorizationConfig,
        EntityRef,
        PrincipalResolver,
    };

    #[derive(Clone)]
    struct UserId(String);

    #[derive(Clone)]
    struct Role(String);

    fn main() -> strands::Result<()> {
        let principal = PrincipalResolver::new(
            |state: &InvocationState| {
                Ok(
                    state
                        .get::<UserId>()
                        .map(|user| EntityRef::new("User", user.0)),
                )
            },
        );
        let context = ContextEnricher::new(
            |input: &ContextEnricherInput<'_>| {
                let mut session = JsonObject::new();
                let role = input
                    .invocation_state
                    .get::<Role>()
                    .map_or_else(|| "none".into(), |role| role.0);
                session.insert("role".into(), role.into());
                Ok(session)
            },
        );
        let dogwood = DogwoodAuthorization::new(
            DogwoodAuthorizationConfig {
                policies: "permit(principal, action, resource);".into(),
                tools: Some(vec![ToolSpec::new("search", "Search")]),
                principal_resolver: Some(principal),
                context_enricher: Some(context),
                ..DogwoodAuthorizationConfig::default()
            },
        )?;
        let _ = dogwood;
        Ok(())
    }
    // --8<-- [end:identity]
}

mod file_policy {
    // --8<-- [start:file_policy]
    use strands::vended_interventions::dogwood::{
        DogwoodAuthorization,
        DogwoodAuthorizationConfig,
    };

    fn main() -> strands::Result<()> {
        let dogwood = DogwoodAuthorization::new(
            DogwoodAuthorizationConfig {
                policies: "policies/agent.dw".into(),
                action_schema: Some(
                    "policies/agent.cedarschema".into(),
                ),
                ..DogwoodAuthorizationConfig::default()
            },
        )?;
        let _ = dogwood;
        Ok(())
    }
    // --8<-- [end:file_policy]
}

mod session_scope {
    // --8<-- [start:session_scope]
    use std::sync::Arc;

    use strands::types::ToolSpec;
    use strands::vended_interventions::dogwood::{
        DogwoodAuthorization,
        DogwoodAuthorizationConfig,
        EntityRef,
    };
    use strands::Agent;

    fn main() -> strands::Result<()> {
        let dogwood = Arc::new(DogwoodAuthorization::new(
            DogwoodAuthorizationConfig {
                policies: r#"
                    permit(
                        principal,
                        action == Action::"search",
                        resource
                    );
                "#
                .into(),
                tools: Some(vec![ToolSpec::new("search", "Search")]),
                principal: Some(EntityRef::new("User", "alice")),
                ..DogwoodAuthorizationConfig::default()
            },
        )?);
        let agent = Agent::builder()
            .interventions([Arc::clone(&dogwood)])
            .printer(false)
            .build()?;
        let _ = agent;
        dogwood.reset();
        Ok(())
    }
    // --8<-- [end:session_scope]
}
