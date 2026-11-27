#![allow(dead_code)]

// --8<-- [start:example]
use strands::Agent;
use strands::vended_tools::handoff_to_user::handoff_to_user;

fn agent() -> strands::Result<Agent> {
    Agent::builder()
        .system_prompt(
            "Before deleting files, call handoff_to_user for confirmation.",
        )
        .tools([handoff_to_user()])
        .printer(false)
        .build()
}
// --8<-- [end:example]
