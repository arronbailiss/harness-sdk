#![allow(dead_code)]

async fn worker() -> strands::Result<()> {
    // --8<-- [start:worker]
    use std::path::PathBuf;
    use strands_harness::{
        BuiltinToolsConfig, HarnessAgentOptions, Isolation,
        MakeProgrammaticToolCallerOptions, create_harness,
        make_programmatic_tool_caller,
    };

    let caller = make_programmatic_tool_caller(
        MakeProgrammaticToolCallerOptions::builder()
            .isolation(Isolation::Subprocess(PathBuf::from(
                "/usr/local/bin/monty",
            )))
            .build(),
    );
    let builtins = BuiltinToolsConfig::builder()
        .programmatic_tool_caller(false)
        .build();
    let _agent = create_harness(
        HarnessAgentOptions::builder()
            .builtin_tools(builtins)
            .tools([caller])
            .build(),
    )
    .await?;
    // --8<-- [end:worker]
    Ok(())
}

fn main() {}
