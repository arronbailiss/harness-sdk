#![allow(dead_code)]

async fn basic_usage() -> strands::Result<()> {
    // --8<-- [start:basic_usage]
    use futures::StreamExt;
    use strands::{Agent, AgentStreamEvent};

    let mut agent = Agent::builder().printer(false).build()?;
    let mut stream = agent.stream("Tell me a short story.");

    while let Some(event) = stream.next().await {
        match event? {
            AgentStreamEvent::ModelStreamUpdate(update) => {
                if let Some(text) = update.text_delta() {
                    print!("{text}");
                }
            }
            AgentStreamEvent::AgentResult(result) => {
                println!("\nStopped: {}", result.stop_reason());
            }
            _ => {}
        }
    }
    // --8<-- [end:basic_usage]
    Ok(())
}

// --8<-- [start:server_stream]
fn response_stream<'a>(
    agent: &'a mut strands::Agent,
    prompt: &'a str,
) -> impl futures::Stream<Item = String> + 'a {
    use futures::StreamExt;

    agent.stream(prompt).filter_map(|event| async move {
        match event {
            Ok(strands::AgentStreamEvent::ModelStreamUpdate(
                update,
            )) => update.text_delta().map(str::to_owned),
            Ok(_) => None,
            Err(error) => Some(format!("Error: {error}")),
        }
    })
}
// --8<-- [end:server_stream]

async fn agentic_loop() -> strands::Result<()> {
    // --8<-- [start:agentic_loop]
    use futures::StreamExt;
    use strands::{Agent, AgentStreamEvent};

    let mut agent = Agent::builder().printer(false).build()?;
    let mut stream = agent.stream("Explain tool calling.");

    while let Some(event) = stream.next().await {
        match event? {
            AgentStreamEvent::BeforeInvocation(_) => {
                println!("Invocation started");
            }
            AgentStreamEvent::BeforeModelCall(_) => {
                println!("Calling the model");
            }
            AgentStreamEvent::BeforeToolCall(call) => {
                println!("Calling {}", call.tool_use.name);
            }
            AgentStreamEvent::AgentResult(result) => {
                println!("Stopped: {}", result.stop_reason());
            }
            _ => {}
        }
    }
    // --8<-- [end:agentic_loop]
    Ok(())
}

async fn sub_agent_stream() -> strands::Result<()> {
    // --8<-- [start:sub_agent_stream]
    use futures::StreamExt;
    use strands::agent::AgentAsToolOptions;
    use strands::{Agent, AgentStreamEvent};

    let notes_agent = Agent::builder()
        .name("notes_agent")
        .description("Organizes notes")
        .printer(false)
        .build()?;
    let notes_tool =
        notes_agent.as_tool(AgentAsToolOptions::default());
    let mut orchestrator = Agent::builder()
        .tools([notes_tool])
        .printer(false)
        .build()?;

    let mut stream = orchestrator.stream("Organize my project notes.");
    while let Some(event) = stream.next().await {
        match event? {
            AgentStreamEvent::ToolStreamUpdate(update) => {
                println!("{:?}", update.event.data);
            }
            AgentStreamEvent::AgentResult(result) => {
                println!("{}", result.stop_reason());
            }
            _ => {}
        }
    }
    // --8<-- [end:sub_agent_stream]
    Ok(())
}

fn main() {}
