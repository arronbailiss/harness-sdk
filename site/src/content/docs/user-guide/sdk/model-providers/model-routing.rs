mod fallback {
    // --8<-- [start:fallback]
    use strands::models::bedrock::BedrockModel;
    use strands::models::routing::{
        ModelRouter,
        ModelRouterOptions,
    };
    use strands::Agent;

    fn main() -> strands::Result<()> {
        let primary = BedrockModel::builder()
            .model_id("us.amazon.nova-pro-v1:0")
            .build();
        let backup = BedrockModel::builder()
            .model_id(
                "us.anthropic.claude-haiku-4-5-20251001-v1:0",
            )
            .build();
        let router = ModelRouter::new(
            [primary, backup],
            ModelRouterOptions {
                max_switches: Some(1),
                ..ModelRouterOptions::default()
            },
        )?;
        let agent = Agent::builder()
            .model(router)
            .build()?;
        let _ = agent;
        Ok(())
    }
    // --8<-- [end:fallback]
}

mod classify {
    // --8<-- [start:classify]
    use std::sync::Arc;

    use serde_json::json;
    use strands::models::bedrock::BedrockModel;
    use strands::models::routing::{
        ClassifierStrategy,
        ClassifierStrategyOptions,
        ModelRouter,
        ModelRouterOptions,
        RoutingCandidate,
        RoutingCandidateOptions,
    };
    use strands::Agent;

    fn main() -> strands::Result<()> {
        let classifier = BedrockModel::builder()
            .model_id(
                "us.anthropic.claude-haiku-4-5-20251001-v1:0",
            )
            .max_tokens(64)
            .temperature(0.0)
            .build();
        let routine = RoutingCandidate::new(
            RoutingCandidateOptions::new(
                BedrockModel::builder()
                    .model_id("us.amazon.nova-lite-v1:0")
                    .build(),
            )
            .with_name("routine")
            .with_description(
                "Concise factual questions and routine requests.",
            )
            .with_metadata(
                json!({"cost": "low", "latency": "low"})
                    .as_object()
                    .unwrap()
                    .clone(),
            ),
        );
        let advanced = RoutingCandidate::new(
            RoutingCandidateOptions::new(
                BedrockModel::builder()
                    .model_id("us.amazon.nova-pro-v1:0")
                    .build(),
            )
            .with_name("advanced")
            .with_description(
                "Systems design with interacting constraints.",
            )
            .with_metadata(
                json!({"cost": "high", "latency": "medium"})
                    .as_object()
                    .unwrap()
                    .clone(),
            ),
        );
        let strategy = ClassifierStrategy::new(
            classifier,
            ClassifierStrategyOptions::builder()
                .routing_policy(
                    "Choose the lowest-latency candidate that \
                     satisfies every request requirement.",
                )
                .build(),
        )?;
        let router = ModelRouter::new(
            [routine, advanced],
            ModelRouterOptions {
                strategy: Some(Arc::new(strategy)),
                ..ModelRouterOptions::default()
            },
        )?;
        let agent = Agent::builder()
            .model(router)
            .build()?;
        let _ = agent;
        Ok(())
    }
    // --8<-- [end:classify]
}

mod composed {
    // --8<-- [start:composed]
    use std::sync::Arc;

    use strands::models::bedrock::BedrockModel;
    use strands::models::routing::{
        ClassifierStrategy,
        ClassifierStrategyOptions,
        FallbackStrategy,
        ModelRouter,
        ModelRouterOptions,
        RoutingCandidate,
        RoutingContext,
        RoutingStrategy,
    };
    use strands::{Agent, BoxError};

    struct ClassifyThenFallback {
        classifier: ClassifierStrategy,
        fallback: FallbackStrategy,
    }

    #[strands::async_trait]
    impl RoutingStrategy for ClassifyThenFallback {
        async fn select(
            &self,
            context: &RoutingContext,
        ) -> Result<Option<RoutingCandidate>, BoxError> {
            if context.attempts.is_empty() {
                return self.classifier.select(context).await;
            }
            self.fallback.select(context).await
        }
    }

    fn main() -> strands::Result<()> {
        let classifier = BedrockModel::builder()
            .model_id(
                "us.anthropic.claude-haiku-4-5-20251001-v1:0",
            )
            .build();
        let strategy = ClassifyThenFallback {
            classifier: ClassifierStrategy::new(
                classifier,
                ClassifierStrategyOptions::default(),
            )?,
            fallback: FallbackStrategy::new(),
        };
        let router = ModelRouter::new(
            [
                BedrockModel::builder()
                    .model_id("us.amazon.nova-lite-v1:0")
                    .build(),
                BedrockModel::builder()
                    .model_id("us.amazon.nova-pro-v1:0")
                    .build(),
            ],
            ModelRouterOptions {
                strategy: Some(Arc::new(strategy)),
                ..ModelRouterOptions::default()
            },
        )?;
        let agent = Agent::builder()
            .model(router)
            .build()?;
        let _ = agent;
        Ok(())
    }
    // --8<-- [end:composed]
}

mod tenant {
    // --8<-- [start:tenant]
    use std::sync::Arc;

    use serde_json::json;
    use strands::models::bedrock::BedrockModel;
    use strands::models::routing::{
        FallbackStrategy,
        ModelRouter,
        ModelRouterOptions,
        RoutingCandidate,
        RoutingCandidateOptions,
        RoutingContext,
        RoutingStrategy,
    };
    use strands::{
        Agent,
        BoxError,
        InvocationState,
        InvokeOptions,
    };

    #[derive(Clone)]
    struct Tenant(String);

    struct TenantPolicy {
        inner: FallbackStrategy,
    }

    #[strands::async_trait]
    impl RoutingStrategy for TenantPolicy {
        async fn select(
            &self,
            context: &RoutingContext,
        ) -> Result<Option<RoutingCandidate>, BoxError> {
            let tenant = context
                .invocation_state
                .get::<Tenant>()
                .map(|tenant| tenant.0);
            let allowed: Vec<_> = context
                .candidates
                .iter()
                .filter(|candidate| {
                    let vendor = candidate
                        .metadata()
                        .and_then(|metadata| metadata.get("vendor"))
                        .and_then(|value| value.as_str());
                    match tenant.as_deref() {
                        Some("customer-a") => {
                            matches!(
                                vendor,
                                Some("amazon") | Some("anthropic")
                            )
                        }
                        Some("customer-b") => vendor == Some("amazon"),
                        _ => false,
                    }
                })
                .cloned()
                .collect();
            if allowed.is_empty() {
                return Err(std::io::Error::other(
                    "tenant has no allowed model",
                )
                .into());
            }
            let mut scoped = context.clone();
            scoped.candidates = allowed.clone();
            let choice = self.inner.select(&scoped).await?;
            if choice.is_none() && context.attempts.is_empty() {
                return Ok(allowed.first().cloned());
            }
            Ok(choice)
        }
    }

    fn candidate(
        model_id: &str,
        name: &str,
        vendor: &str,
    ) -> RoutingCandidate {
        RoutingCandidate::new(
            RoutingCandidateOptions::new(
                BedrockModel::builder()
                    .model_id(model_id)
                    .build(),
            )
            .with_name(name)
            .with_metadata(
                json!({"vendor": vendor})
                    .as_object()
                    .unwrap()
                    .clone(),
            ),
        )
    }

    #[tokio::main]
    async fn main() -> strands::Result<()> {
        let router = ModelRouter::new(
            [
                candidate(
                    "us.anthropic.claude-haiku-4-5-20251001-v1:0",
                    "claude",
                    "anthropic",
                ),
                candidate(
                    "us.amazon.nova-pro-v1:0",
                    "nova",
                    "amazon",
                ),
            ],
            ModelRouterOptions {
                strategy: Some(Arc::new(TenantPolicy {
                    inner: FallbackStrategy::new(),
                })),
                ..ModelRouterOptions::default()
            },
        )?;
        let mut agent = Agent::builder()
            .model(router)
            .build()?;
        let state = InvocationState::new();
        state.insert(Tenant("customer-b".into()));
        let options = InvokeOptions {
            invocation_state: Some(state),
            ..InvokeOptions::default()
        };
        agent
            .invoke_with(
                "Summarize the tradeoffs of active-active deployment.",
                options,
            )
            .await?;
        Ok(())
    }
    // --8<-- [end:tenant]
}
