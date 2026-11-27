mod basic_interrupt {
    // --8<-- [start:basic_interrupt]
    use strands::vended_interventions::hitl::{
        HumanInTheLoop,
        HumanInTheLoopConfig,
    };
    use strands::{Agent, InterruptResponse};

    fn agent() -> strands::Result<Agent> {
        let hitl = HumanInTheLoop::new(HumanInTheLoopConfig::default());
        Agent::builder()
            .interventions([hitl])
            .printer(false)
            .build()
    }

    async fn resume(
        agent: &mut Agent,
        interrupt_id: String,
    ) -> strands::Result<()> {
        agent
            .invoke(vec![InterruptResponse::new(interrupt_id, "yes")])
            .await?;
        Ok(())
    }
    // --8<-- [end:basic_interrupt]
}

mod stdio_mode {
    // --8<-- [start:stdio_mode]
    use strands::vended_interventions::hitl::{
        HumanInTheLoop,
        HumanInTheLoopAsk,
        HumanInTheLoopConfig,
    };
    use strands::Agent;

    fn main() -> strands::Result<()> {
        let hitl = HumanInTheLoop::new(HumanInTheLoopConfig {
            ask: Some(HumanInTheLoopAsk::Stdio),
            ..HumanInTheLoopConfig::default()
        });
        let agent = Agent::builder()
            .interventions([hitl])
            .printer(false)
            .build()?;
        let _ = agent;
        Ok(())
    }
    // --8<-- [end:stdio_mode]
}

mod custom_ask {
    // --8<-- [start:custom_ask]
    use strands::vended_interventions::hitl::{
        HumanInTheLoop,
        HumanInTheLoopAsk,
        HumanInTheLoopConfig,
    };
    use strands::JsonValue;

    fn main() {
        let ask = HumanInTheLoopAsk::callback(|prompt| async move {
            let _ = prompt;
            Ok(Some(JsonValue::from("yes")))
        });
        let hitl = HumanInTheLoop::new(HumanInTheLoopConfig {
            ask: Some(ask),
            ..HumanInTheLoopConfig::default()
        });
        let _ = hitl;
    }
    // --8<-- [end:custom_ask]
}

mod auto_mode {
    // --8<-- [start:auto_mode]
    use strands::vended_interventions::hitl::{
        HumanInTheLoop,
        HumanInTheLoopAutoConfig,
    };

    fn main() {
        let hitl = HumanInTheLoop::auto(
            HumanInTheLoopAutoConfig::default(),
        );
        let _ = hitl;
    }
    // --8<-- [end:auto_mode]
}

mod auto_mode_options {
    // --8<-- [start:auto_mode_options]
    use strands::vended_interventions::hitl::{
        HumanInTheLoop,
        HumanInTheLoopAsk,
        HumanInTheLoopAutoConfig,
    };

    fn main() {
        let hitl = HumanInTheLoop::auto(HumanInTheLoopAutoConfig {
            allowed_tools: vec!["read_file".into()],
            ask: Some(HumanInTheLoopAsk::Stdio),
        });
        let _ = hitl;
    }
    // --8<-- [end:auto_mode_options]
}

mod classifier {
    // --8<-- [start:classifier]
    use strands::vended_interventions::hitl::{
        ClassifierConfig,
        HumanInTheLoop,
        HumanInTheLoopConfig,
    };

    fn main() {
        let hitl = HumanInTheLoop::new(HumanInTheLoopConfig {
            allowed_tools: vec!["read_file".into()],
            classifier: Some(ClassifierConfig::llm()),
            ..HumanInTheLoopConfig::default()
        });
        let _ = hitl;
    }
    // --8<-- [end:classifier]
}

mod classifier_custom {
    // --8<-- [start:classifier_custom]
    use strands::hooks::BeforeToolCallEvent;
    use strands::vended_interventions::hitl::{
        ClassifierConfig,
        ClassifierResult,
        HumanInTheLoop,
        HumanInTheLoopConfig,
    };

    fn main() {
        let classifier = |event: &BeforeToolCallEvent| {
            let requires_approval = event.tool_use().name == "deploy";
            async move {
                Ok(
                    ClassifierResult::new(requires_approval)
                        .with_reason("deployment changes production"),
                )
            }
        };
        let hitl = HumanInTheLoop::new(HumanInTheLoopConfig {
            classifier: Some(ClassifierConfig::custom(classifier)),
            ..HumanInTheLoopConfig::default()
        });
        let _ = hitl;
    }
    // --8<-- [end:classifier_custom]
}

mod allowed_tools {
    // --8<-- [start:allowed_tools]
    use strands::vended_interventions::hitl::{
        HumanInTheLoop,
        HumanInTheLoopConfig,
    };

    fn main() {
        let hitl = HumanInTheLoop::new(HumanInTheLoopConfig {
            allowed_tools: vec![
                "read_file".into(),
                "search".into(),
                "!delete_file".into(),
            ],
            ..HumanInTheLoopConfig::default()
        });
        let _ = hitl;
    }
    // --8<-- [end:allowed_tools]
}

mod trust_mode {
    // --8<-- [start:trust_mode]
    use strands::vended_interventions::hitl::{
        HumanInTheLoop,
        HumanInTheLoopAsk,
        HumanInTheLoopConfig,
    };

    fn main() {
        let hitl = HumanInTheLoop::new(HumanInTheLoopConfig {
            enable_trust: Some(true),
            ask: Some(HumanInTheLoopAsk::Stdio),
            ..HumanInTheLoopConfig::default()
        });
        let _ = hitl;
    }
    // --8<-- [end:trust_mode]
}

mod custom_evaluate {
    // --8<-- [start:custom_evaluate]
    use strands::interventions::Evaluate;
    use strands::vended_interventions::hitl::{
        HumanInTheLoop,
        HumanInTheLoopConfig,
    };

    fn main() {
        let evaluate = Evaluate::new(|response| {
            response
                .as_str()
                .is_some_and(|value| value.eq_ignore_ascii_case("confirm"))
        });
        let hitl = HumanInTheLoop::new(HumanInTheLoopConfig {
            evaluate: Some(evaluate),
            ..HumanInTheLoopConfig::default()
        });
        let _ = hitl;
    }
    // --8<-- [end:custom_evaluate]
}
