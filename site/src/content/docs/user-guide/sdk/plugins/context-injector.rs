#![allow(dead_code)]

mod getting_started {
    // --8<-- [start:getting_started]
    use std::time::SystemTime;

    use strands::vended_plugins::context_injector::{
        ContextInjector,
        ContextInjectorConfig,
    };
    use strands::Agent;

    fn agent() -> strands::Result<Agent> {
        let injector = ContextInjector::new(ContextInjectorConfig::new(
            |_context| async {
                Ok(Some(format!("<now>{:?}</now>", SystemTime::now())))
            },
        ));
        Agent::builder()
            .plugins([injector])
            .printer(false)
            .build()
    }
    // --8<-- [end:getting_started]
}

mod every_turn {
    // --8<-- [start:every_turn]
    use std::time::SystemTime;

    use strands::vended_plugins::context_injector::{
        ContextInjector,
        ContextInjectorConfig,
        InjectionTrigger,
    };

    fn clock() -> ContextInjector {
        let config = ContextInjectorConfig::new(|_context| async {
            Ok(Some(format!("<now>{:?}</now>", SystemTime::now())))
        })
        .with_name("clock")
        .with_trigger(InjectionTrigger::EveryTurn);
        ContextInjector::new(config)
    }
    // --8<-- [end:every_turn]
}

mod predicate {
    // --8<-- [start:predicate]
    use strands::vended_plugins::context_injector::{
        ContextInjector,
        ContextInjectorConfig,
        InjectionTrigger,
    };

    fn injector() -> ContextInjector {
        let trigger = InjectionTrigger::predicate(|context| {
            Ok(context
                .app_state
                .get::<bool>("recall_enabled")?
                .unwrap_or(false))
        });
        let config = ContextInjectorConfig::new(|context| async move {
            Ok(Some(format!(
                "<context>{} turns so far</context>",
                context.messages.len()
            )))
        })
        .with_trigger(trigger);
        ContextInjector::new(config)
    }
    // --8<-- [end:predicate]
}
