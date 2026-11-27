#![allow(dead_code)]

// --8<-- [start:example]
use strands::Agent;
use strands::vended_tools::read::read;

fn agent() -> strands::Result<Agent> {
    Agent::builder().tools([read()]).printer(false).build()
}
// --8<-- [end:example]
