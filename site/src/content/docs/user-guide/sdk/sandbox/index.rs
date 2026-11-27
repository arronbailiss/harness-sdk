#![allow(dead_code)]

mod basic_usage {
    // --8<-- [start:basic_usage]
    use strands::sandbox::NotASandboxLocalEnvironment;
    use strands::Agent;

    fn agent() -> strands::Result<Agent> {
        Agent::builder()
            .sandbox(NotASandboxLocalEnvironment::new())
            .printer(false)
            .build()
    }
    // --8<-- [end:basic_usage]
}

mod tool_override {
    // --8<-- [start:tool_override]
    use strands::{tool, Agent, ToolError};

    #[tool(description = "Run an approved workspace command.")]
    async fn sandbox_shell(command: String) -> Result<String, ToolError> {
        Ok(format!("Approved command: {command}"))
    }

    fn agent() -> strands::Result<Agent> {
        Agent::builder()
            .tools([SandboxShell])
            .printer(false)
            .build()
    }
    // --8<-- [end:tool_override]
}

mod custom_tool {
    // --8<-- [start:custom_tool]
    use strands::{tool, Agent, ToolContext, ToolError};

    #[tool(description = "Run a command in the agent's sandbox.")]
    async fn run_command(
        command: String,
        #[tool(context)] context: &ToolContext,
    ) -> Result<String, ToolError> {
        let result = context
            .agent()
            .sandbox()
            .execute(&command, Default::default())
            .await
            .map_err(ToolError::new)?;
        Ok(result.stdout)
    }

    fn agent() -> strands::Result<Agent> {
        Agent::builder()
            .tools([RunCommand])
            .printer(false)
            .build()
    }
    // --8<-- [end:custom_tool]
}
