#![allow(dead_code)]

fn todos() -> strands::Result<()> {
    // --8<-- [start:todos]
    use strands::Agent;
    use strands::vended_plugins::todos::Todos;

    let _agent = Agent::builder().plugins([Todos::default()]).build()?;
    // --8<-- [end:todos]
    Ok(())
}

async fn environment() -> strands::Result<()> {
    // --8<-- [start:environment]
    use strands_harness::{HarnessAgentOptions, create_harness};

    let _agent = create_harness(
        HarnessAgentOptions::builder()
            .builtin_plugins([])
            .build(),
    )
    .await?;
    // --8<-- [end:environment]
    Ok(())
}

fn main() {}
