mod basic_example {
    // --8<-- [start:basic_example]
    use strands::vended_interventions::cedar::{
        CedarAuthorization,
        CedarAuthorizationConfig,
    };
    use strands::Agent;

    fn main() -> strands::Result<()> {
        let cedar = CedarAuthorization::new(CedarAuthorizationConfig {
            policies: r#"
                permit(
                    principal,
                    action == Action::"search",
                    resource
                );
            "#
            .into(),
            ..CedarAuthorizationConfig::default()
        })?;
        let agent = Agent::builder()
            .interventions([cedar])
            .printer(false)
            .build()?;
        let _ = agent;
        Ok(())
    }
    // --8<-- [end:basic_example]
}

mod role_based {
    // --8<-- [start:role_based]
    use strands::agent::InvocationState;
    use strands::vended_interventions::cedar::{
        CedarAuthorization,
        CedarAuthorizationConfig,
        PrincipalResolver,
        TypeAndId,
    };

    #[derive(Clone)]
    struct UserId(String);

    fn main() -> strands::Result<()> {
        let resolver = PrincipalResolver::new(
            |state: &InvocationState| {
                Ok(
                    state
                        .get::<UserId>()
                        .map(|user| TypeAndId::new("User", user.0)),
                )
            },
        );
        let cedar = CedarAuthorization::new(CedarAuthorizationConfig {
            policies: "permit(principal, action, resource);".into(),
            principal_resolver: Some(resolver),
            ..CedarAuthorizationConfig::default()
        })?;
        let _ = cedar;
        Ok(())
    }
    // --8<-- [end:role_based]
}

mod rate_limit {
    // --8<-- [start:rate_limit]
    use strands::vended_interventions::cedar::{
        CedarAuthorization,
        CedarAuthorizationConfig,
    };

    fn main() -> strands::Result<()> {
        let policy = r#"
            permit(
                principal,
                action == Action::"search",
                resource
            ) when {
                context.session.call_count <= 3
            };
        "#;
        let cedar = CedarAuthorization::new(CedarAuthorizationConfig {
            policies: policy.into(),
            ..CedarAuthorizationConfig::default()
        })?;
        let _ = cedar;
        Ok(())
    }
    // --8<-- [end:rate_limit]
}

mod schema_validation {
    // --8<-- [start:schema_validation]
    use serde_json::json;
    use strands::vended_interventions::cedar::{
        CedarAuthorization,
        CedarAuthorizationConfig,
        ToolDefinition,
    };

    fn main() -> strands::Result<()> {
        let search = ToolDefinition::new(
            "search",
            json!({
                "type": "object",
                "properties": {
                    "query": {"type": "string"}
                },
                "required": ["query"]
            }),
        );
        let cedar = CedarAuthorization::new(CedarAuthorizationConfig {
            policies: r#"
                permit(
                    principal,
                    action == Action::"search",
                    resource
                );
            "#
            .into(),
            tools: Some(vec![search]),
            ..CedarAuthorizationConfig::default()
        })?;
        let _ = cedar;
        Ok(())
    }
    // --8<-- [end:schema_validation]
}

mod namespace {
    // --8<-- [start:namespace]
    use strands::vended_interventions::cedar::{
        CedarAuthorization,
        CedarAuthorizationConfig,
    };

    fn main() -> strands::Result<()> {
        let cedar = CedarAuthorization::new(CedarAuthorizationConfig {
            policies: r#"
                permit(
                    principal,
                    action == Agent::Action::"search",
                    resource
                );
            "#
            .into(),
            namespace: Some("Agent".into()),
            ..CedarAuthorizationConfig::default()
        })?;
        let _ = cedar;
        Ok(())
    }
    // --8<-- [end:namespace]
}

mod env_gating {
    // --8<-- [start:env_gating]
    use strands::types::JsonObject;
    use strands::vended_interventions::cedar::{
        CedarAuthorization,
        CedarAuthorizationConfig,
        ContextEnricher,
        ContextEnricherInput,
    };

    fn main() -> strands::Result<()> {
        let enricher = ContextEnricher::new(
            |_input: &ContextEnricherInput<'_>| {
                let mut session = JsonObject::new();
                session.insert("environment".into(), "staging".into());
                Ok(session)
            },
        );
        let cedar = CedarAuthorization::new(CedarAuthorizationConfig {
            policies: r#"
                permit(principal, action, resource) when {
                    context.session.environment in ["dev", "staging"]
                };
            "#
            .into(),
            context_enricher: Some(enricher),
            ..CedarAuthorizationConfig::default()
        })?;
        let _ = cedar;
        Ok(())
    }
    // --8<-- [end:env_gating]
}

mod file_policies {
    // --8<-- [start:file_policies]
    use strands::vended_interventions::cedar::{
        CedarAuthorization,
        CedarAuthorizationConfig,
    };

    fn main() -> strands::Result<()> {
        let cedar = CedarAuthorization::new(CedarAuthorizationConfig {
            policies: "policies/agent.cedar".into(),
            schema: Some("policies/agent.cedarschema".into()),
            ..CedarAuthorizationConfig::default()
        })?;
        let _ = cedar;
        Ok(())
    }
    // --8<-- [end:file_policies]
}

mod hot_reload {
    // --8<-- [start:hot_reload]
    use strands::vended_interventions::cedar::{
        CedarAuthorization,
        CedarAuthorizationConfig,
    };

    fn reload() -> strands::Result<()> {
        let cedar = CedarAuthorization::new(CedarAuthorizationConfig {
            policies: "policies/agent.cedar".into(),
            ..CedarAuthorizationConfig::default()
        })?;
        cedar.reload()?;
        Ok(())
    }
    // --8<-- [end:hot_reload]
}
