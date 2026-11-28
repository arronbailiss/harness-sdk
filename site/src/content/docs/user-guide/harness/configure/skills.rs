#![allow(dead_code)]

async fn harness_paths() -> strands::Result<()> {
    // --8<-- [start:harness_paths]
    use strands_harness::{HarnessAgentOptions, create_harness};

    let _agent = create_harness(
        HarnessAgentOptions::builder()
            .skills(["./team-skills", "./project-skills"])
            .build(),
    )
    .await?;
    // --8<-- [end:harness_paths]
    Ok(())
}

fn sdk_plugin() -> strands::Result<()> {
    // --8<-- [start:sdk_plugin]
    use strands::Agent;
    use strands::vended_plugins::skills::{
        AgentSkills, AgentSkillsConfig,
    };

    let skills = AgentSkills::new(
        AgentSkillsConfig::builder()
            .skills(["./team-skills", "./project-skills"])
            .build(),
    );
    let _agent = Agent::builder().plugins([skills]).build()?;
    // --8<-- [end:sdk_plugin]
    Ok(())
}

async fn harness_off() -> strands::Result<()> {
    // --8<-- [start:harness_off]
    use strands_harness::{HarnessAgentOptions, create_harness};

    let _agent = create_harness(
        HarnessAgentOptions::builder().skills(false).build(),
    )
    .await?;
    // --8<-- [end:harness_off]
    Ok(())
}

fn main() {}
