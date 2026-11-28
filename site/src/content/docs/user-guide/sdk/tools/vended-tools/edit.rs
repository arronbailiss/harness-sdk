#![allow(dead_code)]

// --8<-- [start:example]
use strands::Agent;
use strands::vended_tools::edit::edit;

fn agent() -> strands::Result<Agent> {
    Agent::builder().tools([edit()]).printer(false).build()
}
// --8<-- [end:example]
