#![allow(dead_code)]

async fn configure() -> strands::Result<()> {
    // --8<-- [start:configure]
    use strands::background_tasks::BackgroundTasksConfig;
    use strands_harness::{HarnessAgentOptions, create_harness};

    let _no_background = create_harness(
        HarnessAgentOptions::builder().background_tasks(false).build(),
    )
    .await?;
    let policy = BackgroundTasksConfig {
        agentic: Some(vec!["long_running_report".into()]),
        ..BackgroundTasksConfig::default()
    };
    let _scoped = create_harness(
        HarnessAgentOptions::builder().background_tasks(policy).build(),
    )
    .await?;
    // --8<-- [end:configure]
    Ok(())
}

fn cancel(agent: &strands::Agent) {
    // --8<-- [start:cancel]
    use strands::CancelOptions;

    agent.cancel_handle().cancel(CancelOptions {
        cancel_background_tasks: true,
        ..CancelOptions::default()
    });
    // --8<-- [end:cancel]
}

fn main() {}
