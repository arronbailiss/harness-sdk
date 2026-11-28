#![allow(dead_code)]

// --8<-- [start:attach_plugin]
use strands::vended_plugins::context_injector::{
    ContextInjector,
    ContextInjectorConfig,
};
use strands::Agent;

fn agent() -> strands::Result<Agent> {
    let injector = ContextInjector::new(ContextInjectorConfig::new(
        |_context| async {
            Ok(Some("<environment>production</environment>".to_owned()))
        },
    ));
    Agent::builder()
        .plugins([injector])
        .printer(false)
        .build()
}
// --8<-- [end:attach_plugin]
