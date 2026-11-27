#![allow(dead_code)]

async fn harness_model() -> strands::Result<()> {
    // --8<-- [start:harness_model]
    use strands_harness::{HarnessAgentOptions, create_harness};

    let _agent = create_harness(
        HarnessAgentOptions::builder()
            .model("openai/gpt-5.6-sol")
            .build(),
    )
    .await?;
    // --8<-- [end:harness_model]
    Ok(())
}

fn sdk_model() -> strands::Result<()> {
    // --8<-- [start:sdk_model]
    use strands::Agent;

    let _agent = Agent::builder()
        .model("global.anthropic.claude-sonnet-4-6")
        .build()?;
    // --8<-- [end:sdk_model]
    Ok(())
}

async fn model_instance() -> strands::Result<()> {
    // --8<-- [start:model_instance]
    use strands::models::bedrock::BedrockModel;
    use strands_harness::{HarnessAgentOptions, create_harness};

    let model = BedrockModel::builder()
        .model_id("global.anthropic.claude-opus-4-6-v1")
        .region("us-west-2")
        .build();
    let _agent = create_harness(
        HarnessAgentOptions::builder().model(model).build(),
    )
    .await?;
    // --8<-- [end:model_instance]
    Ok(())
}

async fn harness_effort() -> strands::Result<()> {
    // --8<-- [start:harness_effort]
    use strands_harness::{
        Effort, HarnessAgentOptions, create_harness,
    };

    let _agent = create_harness(
        HarnessAgentOptions::builder()
            .model("anthropic/claude-opus-5")
            .effort(Effort::High)
            .build(),
    )
    .await?;
    // --8<-- [end:harness_effort]
    Ok(())
}

fn main() {}
