#![allow(dead_code)]

async fn instructions() -> strands::Result<()> {
    // --8<-- [start:instructions]
    use strands_harness::{HarnessAgentOptions, create_harness};

    let _agent = create_harness(
        HarnessAgentOptions::builder()
            .instructions(
                "You are a support assistant. Link every ticket you change.",
            )
            .build(),
    )
    .await?;
    // --8<-- [end:instructions]
    Ok(())
}

fn system_prompt() -> strands::Result<()> {
    // --8<-- [start:system_prompt]
    use strands::Agent;

    let _agent = Agent::builder()
        .system_prompt("You are a support assistant. Link every ticket.")
        .build()?;
    // --8<-- [end:system_prompt]
    Ok(())
}

// --8<-- [start:custom_tool]
use strands::{ToolError, tool};
use strands_harness::{HarnessAgentOptions, create_harness};

#[tool(description = "Fetch a support ticket by id.")]
async fn get_ticket(ticket_id: String) -> Result<String, ToolError> {
    Ok(format!("Ticket {ticket_id}: open, assigned to support."))
}

async fn support_agent() -> strands::Result<()> {
    let _agent = create_harness(
        HarnessAgentOptions::builder().tools([GetTicket]).build(),
    )
    .await?;
    Ok(())
}
// --8<-- [end:custom_tool]

async fn builtin_tools() -> strands::Result<()> {
    // --8<-- [start:builtin_tools]
    use strands_harness::{
        BuiltinToolName, HarnessAgentOptions, create_harness,
    };

    let _agent = create_harness(
        HarnessAgentOptions::builder()
            .builtin_tools([BuiltinToolName::Read, BuiltinToolName::Shell])
            .build(),
    )
    .await?;
    // --8<-- [end:builtin_tools]
    Ok(())
}

fn main() {}
