#![allow(dead_code)]

// --8<-- [start:event_serialization]
fn serialize_for_client(
    event: &strands::AgentStreamEvent,
) -> serde_json::Result<Option<String>> {
    let forward = matches!(
        event,
        strands::AgentStreamEvent::ModelStreamUpdate(_)
            | strands::AgentStreamEvent::BeforeToolCall(_)
            | strands::AgentStreamEvent::ToolResult(_)
            | strands::AgentStreamEvent::AgentResult(_)
    );

    if forward {
        serde_json::to_string(event).map(Some)
    } else {
        Ok(None)
    }
}
// --8<-- [end:event_serialization]

fn main() {}
