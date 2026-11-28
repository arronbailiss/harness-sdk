#![allow(dead_code)]

mod hooks {
    // --8<-- [start:hooks]
    use serde_json::json;
    use strands::hooks::{BeforeToolCallEvent, BeforeToolsEvent};
    use strands::{
        Agent,
        ContentBlock,
        InterruptParams,
        InterruptResponse,
        StopReason,
        tool,
    };

    #[tool(description = "Delete files at the supplied paths.")]
    async fn delete_files(paths: Vec<String>) -> bool {
        !paths.is_empty()
    }

    #[tool(description = "Inspect files at the supplied paths.")]
    async fn inspect_files(paths: Vec<String>) -> Vec<String> {
        paths
    }

    fn with_per_tool_approval() -> strands::Result<Agent> {
        let agent = Agent::builder()
            .system_prompt("Delete files older than five days.")
            .tools(strands::tool_list![
                DeleteFiles,
                InspectFiles,
            ])
            .build()?;
        agent.add_hook(|event: &mut BeforeToolCallEvent| {
            if event.tool_use().name != "delete_files" {
                return;
            }
            let paths = event.tool_use().input["paths"].clone();
            let params = InterruptParams::new("myapp-approval")
                .with_reason(json!({"paths": paths}));
            if let Ok(answer) = event.interrupt(params)
                && answer != json!("y")
            {
                event.set_cancel(
                    "User denied permission to delete files",
                );
            }
        });
        Ok(agent)
    }

    fn with_batch_approval() -> strands::Result<Agent> {
        let agent = Agent::builder()
            .system_prompt("Delete files older than five days.")
            .tools([DeleteFiles])
            .build()?;
        agent.add_hook(|event: &mut BeforeToolsEvent<'_>| {
            let tools: Vec<String> = event
                .message()
                .content
                .iter()
                .filter_map(|block| match block {
                    ContentBlock::ToolUse(tool)
                        if tool.name == "delete_files" =>
                    {
                        Some(tool.name.clone())
                    }
                    _ => None,
                })
                .collect();
            if tools.is_empty() {
                return;
            }
            let params = InterruptParams::new(
                "myapp-batch-approval",
            )
            .with_reason(json!({"tools": tools}));
            if let Ok(answer) = event.interrupt(params)
                && answer != json!("y")
            {
                event.set_cancel("Batch cancelled by user");
            }
        });
        Ok(agent)
    }

    async fn run() -> strands::Result<()> {
        let mut agent = with_per_tool_approval()?;
        let mut result = agent
            .invoke("Delete a/b/c.txt and d/e/f.txt.")
            .await?;
        while result.stop_reason() == StopReason::Interrupt {
            let responses = result
                .interrupts()
                .iter()
                .map(|interrupt| {
                    InterruptResponse::new(
                        interrupt.id.clone(),
                        "y",
                    )
                })
                .collect::<Vec<_>>();
            result = agent.invoke(responses).await?;
        }
        println!("{result}");
        Ok(())
    }
    // --8<-- [end:hooks]
}

mod tools {
    // --8<-- [start:tool_interrupt]
    use serde_json::json;
    use strands::{
        Agent,
        InterruptParams,
        ToolContext,
        ToolError,
        tool,
    };

    #[tool(description = "Delete files after user approval.")]
    async fn delete_files(
        #[tool(context)] context: &ToolContext,
        paths: Vec<String>,
    ) -> Result<bool, ToolError> {
        let approval = context.interrupt(
            InterruptParams::new("myapp-approval")
                .with_reason(json!({"paths": paths})),
        )?;
        Ok(approval == json!("y"))
    }

    fn agent() -> strands::Result<Agent> {
        Agent::builder()
            .system_prompt("Delete files older than five days.")
            .tools([DeleteFiles])
            .build()
    }
    // --8<-- [end:tool_interrupt]
}

mod sessions {
    // --8<-- [start:session_management]
    use serde_json::json;
    use strands::hooks::BeforeToolCallEvent;
    use strands::storage::LocalFileStorage;
    use strands::{
        Agent,
        AgentResult,
        InterruptParams,
        InterruptResponse,
        InvokeArgs,
        SessionManager,
        StopReason,
        tool,
    };

    #[tool(description = "Delete files at the supplied paths.")]
    async fn delete_files(paths: Vec<String>) -> bool {
        !paths.is_empty()
    }

    #[tool(description = "Inspect files at the supplied paths.")]
    async fn inspect_files(paths: Vec<String>) -> Vec<String> {
        paths
    }

    fn register_approval(agent: &Agent) {
        agent.add_hook(|event: &mut BeforeToolCallEvent| {
            if event.tool_use().name != "delete_files" {
                return;
            }
            let trusted = event
                .agent()
                .app_state()
                .get::<String>("myapp-approval")
                .ok()
                .flatten();
            if trusted.as_deref() == Some("t") {
                return;
            }
            let paths = event.tool_use().input["paths"].clone();
            let params = InterruptParams::new("myapp-approval")
                .with_reason(json!({"paths": paths}));
            if let Ok(answer) = event.interrupt(params) {
                let answer =
                    answer.as_str().unwrap_or_default().to_lowercase();
                if answer != "y" && answer != "t" {
                    event.set_cancel(
                        "User denied permission to delete files",
                    );
                }
                let _ = event
                    .agent()
                    .app_state()
                    .set("myapp-approval", answer);
            }
        });
    }

    async fn server(input: InvokeArgs) -> strands::Result<AgentResult> {
        let session = SessionManager::builder()
            .session_id("myapp")
            .storage(LocalFileStorage::new("./sessions"))
            .build()?;
        let mut agent = Agent::builder()
            .session_manager(session)
            .system_prompt("Delete files older than five days.")
            .tools(strands::tool_list![
                DeleteFiles,
                InspectFiles,
            ])
            .build()?;
        register_approval(&agent);
        agent.invoke(input).await
    }

    async fn client() -> strands::Result<AgentResult> {
        let mut result = server(
            "Delete a/b/c.txt and d/e/f.txt.".into(),
        )
        .await?;
        while result.stop_reason() == StopReason::Interrupt {
            let responses = result
                .interrupts()
                .iter()
                .map(|interrupt| {
                    InterruptResponse::new(
                        interrupt.id.clone(),
                        "t",
                    )
                })
                .collect::<Vec<_>>();
            result = server(responses.into()).await?;
        }
        Ok(result)
    }
    // --8<-- [end:session_management]
}
