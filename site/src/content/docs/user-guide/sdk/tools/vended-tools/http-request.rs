#![allow(dead_code)]

// --8<-- [start:example]
use strands::Agent;
use strands::vended_tools::http_request::http_request;

fn agent() -> strands::Result<Agent> {
    Agent::builder()
        .tools([http_request()])
        .printer(false)
        .build()
}
// --8<-- [end:example]
