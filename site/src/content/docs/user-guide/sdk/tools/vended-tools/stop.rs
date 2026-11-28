#![allow(dead_code)]

// --8<-- [start:example]
use strands::Agent;
use strands::vended_tools::stop::stop;

fn agent() -> strands::Result<Agent> {
    Agent::builder()
        .system_prompt(
            "Complete the task, then call stop with a short summary.",
        )
        .tools([stop()])
        .printer(false)
        .build()
}
// --8<-- [end:example]
