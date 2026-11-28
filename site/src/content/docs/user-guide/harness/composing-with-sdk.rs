#![allow(dead_code)]

async fn compose() -> strands::Result<()> {
    // --8<-- [start:compose]
    use strands::context_manager::ContextManagerStrategy;
    use strands::vended_interventions::hitl::HumanInTheLoop;
    use strands::vended_plugins::todos::Todos;
    use strands_harness::{HarnessAgentOptions, create_harness};

    let _agent = create_harness(
        HarnessAgentOptions::builder()
            .context_manager(ContextManagerStrategy::Agentic)
            .plugins([Todos::default()])
            .interventions(HumanInTheLoop::default())
            .build(),
    )
    .await?;
    // --8<-- [end:compose]
    Ok(())
}

async fn modify() -> strands::Result<()> {
    // --8<-- [start:modify]
    use strands::SystemPrompt;
    use strands_harness::{HarnessAgentOptions, create_harness};

    let mut agent = create_harness(HarnessAgentOptions::default()).await?;
    let prompt = match agent.system_prompt() {
        Some(SystemPrompt::Text(text)) => text.as_str(),
        _ => "",
    };
    agent.set_system_prompt(Some(format!(
        "{prompt}\n\nAlways cite file paths as file:line."
    ).into()));
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
