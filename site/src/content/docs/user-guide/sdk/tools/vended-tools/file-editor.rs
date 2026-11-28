#![allow(dead_code, deprecated)]

// --8<-- [start:example]
use strands::Agent;
use strands::vended_tools::file_editor::file_editor;

fn agent() -> strands::Result<Agent> {
    Agent::builder()
        .tools([file_editor()])
        .printer(false)
        .build()
}
// --8<-- [end:example]
