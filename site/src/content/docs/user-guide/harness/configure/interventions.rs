#![allow(dead_code)]

async fn harness_ask() -> strands::Result<()> {
    // --8<-- [start:harness_ask]
    use strands_harness::{HarnessAgentOptions, create_harness};

    let _agent = create_harness(
        HarnessAgentOptions::builder().interventions("ask").build(),
    )
    .await?;
    // --8<-- [end:harness_ask]
    Ok(())
}

fn sdk_ask() -> strands::Result<()> {
    // --8<-- [start:sdk_ask]
    use strands::Agent;
    use strands::vended_interventions::hitl::{
        HumanInTheLoop, HumanInTheLoopConfig,
    };

    let approval = HumanInTheLoop::new(HumanInTheLoopConfig::default());
    let _agent = Agent::builder().intervention(approval).build()?;
    // --8<-- [end:sdk_ask]
    Ok(())
}

async fn natural_language() -> strands::Result<()> {
    // --8<-- [start:natural_language]
    use strands_harness::{HarnessAgentOptions, create_harness};

    let _agent = create_harness(
        HarnessAgentOptions::builder()
            .interventions(
                "Ask before deleting files or making network requests.",
            )
            .build(),
    )
    .await?;
    // --8<-- [end:natural_language]
    Ok(())
}

fn sdk_policy() -> strands::Result<()> {
    // --8<-- [start:sdk_policy]
    use strands::Agent;
    use strands::vended_interventions::hitl::{
        HumanInTheLoop, HumanInTheLoopConfig, LlmClassifierConfig,
    };

    let classifier = LlmClassifierConfig::builder()
        .system_prompt(
            "Ask before deleting files or making network requests.",
        )
        .build();
    let approval = HumanInTheLoop::new(
        HumanInTheLoopConfig::builder()
            .classifier(classifier)
            .build(),
    );
    let _agent = Agent::builder().intervention(approval).build()?;
    // --8<-- [end:sdk_policy]
    Ok(())
}

async fn cedar() -> strands::Result<()> {
    // --8<-- [start:cedar]
    use strands_harness::{HarnessAgentOptions, create_harness};

    let _agent = create_harness(
        HarnessAgentOptions::builder()
            .interventions("./policies/agent.cedar")
            .build(),
    )
    .await?;
    // --8<-- [end:cedar]
    Ok(())
}

fn main() {}
