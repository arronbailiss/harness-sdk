#![allow(dead_code)]

// --8<-- [start:example]
use strands::Agent;
use strands::vended_tools::shell::{
    make_shell,
    MakeShellOptions,
};

fn agent() -> strands::Result<Agent> {
    let shell = make_shell(
        MakeShellOptions::builder()
            .default_timeout(30.0)
            .build(),
    )?;
    Agent::builder().tools([shell]).printer(false).build()
}
// --8<-- [end:example]
