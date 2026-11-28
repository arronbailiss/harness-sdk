#![allow(dead_code)]

// --8<-- [start:example]
use strands::Agent;
use strands::vended_tools::write::write;

fn agent() -> strands::Result<Agent> {
    Agent::builder().tools([write()]).printer(false).build()
}
// --8<-- [end:example]
