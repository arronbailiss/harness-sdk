#![allow(dead_code)]

async fn harness_context() -> strands::Result<()> {
    // --8<-- [start:harness_context]
    use strands::context_manager::ContextManagerStrategy;
    use strands_harness::{HarnessAgentOptions, create_harness};

    let _agent = create_harness(
        HarnessAgentOptions::builder()
            .context_manager(ContextManagerStrategy::Agentic)
            .build(),
    )
    .await?;
    // --8<-- [end:harness_context]
    Ok(())
}

fn sdk_context() -> strands::Result<()> {
    // --8<-- [start:sdk_context]
    use strands::Agent;
    use strands::context_manager::ContextManagerStrategy;

    let _agent = Agent::builder()
        .context_manager(ContextManagerStrategy::Auto)
        .build()?;
    // --8<-- [end:sdk_context]
    Ok(())
}

async fn harness_caching() -> strands::Result<()> {
    // --8<-- [start:harness_caching]
    use strands_harness::{HarnessAgentOptions, create_harness};

    let _agent = create_harness(
        HarnessAgentOptions::builder().caching(false).build(),
    )
    .await?;
    // --8<-- [end:harness_caching]
    Ok(())
}

fn sdk_caching() -> strands::Result<()> {
    // --8<-- [start:sdk_caching]
    use strands::Agent;
    use strands::models::CacheConfig;
    use strands::models::bedrock::BedrockModel;

    let model = BedrockModel::builder()
        .model_id("global.anthropic.claude-sonnet-4-6")
        .cache_config(CacheConfig::builder().tools_ttl(true).build())
        .build();
    let _agent = Agent::builder().model(model).build()?;
    // --8<-- [end:sdk_caching]
    Ok(())
}

fn main() {}
