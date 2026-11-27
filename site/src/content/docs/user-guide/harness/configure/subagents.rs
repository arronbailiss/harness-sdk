#![allow(dead_code)]

async fn specialist() -> strands::Result<()> {
    // --8<-- [start:specialist]
    use strands::Agent;
    use strands_harness::{HarnessAgentOptions, create_harness};

    let researcher = Agent::builder()
        .name("researcher")
        .description("Returns a concise, sourced research summary.")
        .system_prompt("Research the topic and cite each source.")
        .build()?;
    let _agent = create_harness(
        HarnessAgentOptions::builder().tools([researcher]).build(),
    )
    .await?;
    // --8<-- [end:specialist]
    Ok(())
}

async fn disable() -> strands::Result<()> {
    // --8<-- [start:disable]
    use strands_harness::{
        BuiltinToolsConfig, HarnessAgentOptions, create_harness,
    };

    let tools = BuiltinToolsConfig::builder().subagent(false).build();
    let _agent = create_harness(
        HarnessAgentOptions::builder().builtin_tools(tools).build(),
    )
    .await?;
    // --8<-- [end:disable]
    Ok(())
}

async fn depth() -> strands::Result<()> {
    // --8<-- [start:depth]
    use strands_harness::{
        BuiltinToolsConfig, HarnessAgentOptions, SubagentConfig,
        create_harness,
    };

    let subagent = SubagentConfig::builder().max_depth(1).build();
    let tools = BuiltinToolsConfig::builder().subagent(subagent).build();
    let _agent = create_harness(
        HarnessAgentOptions::builder().builtin_tools(tools).build(),
    )
    .await?;
    // --8<-- [end:depth]
    Ok(())
}

fn main() {}
