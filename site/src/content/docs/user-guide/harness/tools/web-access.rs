#![allow(dead_code)]

async fn harness_fetch() -> strands::Result<()> {
    // --8<-- [start:harness_fetch]
    use strands_harness::{
        BuiltinToolsConfig, HarnessAgentOptions, WebFetchConfig,
        create_harness,
    };

    let web_fetch = WebFetchConfig::builder()
        .model("bedrock/global.anthropic.claude-haiku-4-5-20251001-v1:0")
        .build();
    let tools = BuiltinToolsConfig::builder()
        .web_fetch(web_fetch)
        .build();
    let _agent = create_harness(
        HarnessAgentOptions::builder().builtin_tools(tools).build(),
    )
    .await?;
    // --8<-- [end:harness_fetch]
    Ok(())
}

fn main() {}
