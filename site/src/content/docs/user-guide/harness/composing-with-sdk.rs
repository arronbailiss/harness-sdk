#![allow(dead_code)]

async fn compose() -> strands::Result<()> {
    // --8<-- [start:compose]
    use strands::context_manager::ContextManagerStrategy;
    use strands_harness::{HarnessAgentOptions, create_harness};

    let _agent = create_harness(
        HarnessAgentOptions::builder()
            .context_manager(ContextManagerStrategy::Agentic)
            .interventions("ask")
            .build(),
    )
    .await?;
    // --8<-- [end:compose]
    Ok(())
}

async fn modify() -> strands::Result<()> {
    // --8<-- [start:modify]
    use strands_harness::{HarnessAgentOptions, create_harness};

    let mut agent = create_harness(HarnessAgentOptions::default()).await?;
    agent.set_system_prompt(Some(
        "Always cite file paths as file:line.".into(),
    ));
    // --8<-- [end:modify]
    Ok(())
}

fn prompt() -> strands::Result<()> {
    // --8<-- [start:prompt]
    use strands::Agent;
    use strands_harness::build_system_prompt;

    let _agent = Agent::builder()
        .system_prompt(build_system_prompt(
            Some("You are a migration assistant."),
            &[],
        ))
        .build()?;
    // --8<-- [end:prompt]
    Ok(())
}

fn main() {}
