#![allow(dead_code)]

fn result_structure() {
    // --8<-- [start:result_structure]
    let result = strands::ToolResultBlock::new(
        "tool-123",
        strands::ToolResultStatus::Success,
        vec![strands::ToolResultContent::text("Operation completed")],
    );
    // --8<-- [end:result_structure]
    let _ = result;
}

fn response_examples() {
    // --8<-- [start:response_examples]
    let success = strands::ToolResultBlock::success(
        "tool-123",
        vec![
            strands::ToolResultContent::text("Operation completed"),
            strands::ToolResultContent::json(
                serde_json::json!({"results": [1, 2, 3]}),
            ),
        ],
    );
    let error = strands::ToolResultBlock::error(
        "tool-124",
        vec![strands::ToolResultContent::text(
            "Invalid parameters",
        )],
    );
    // --8<-- [end:response_examples]
    let _ = (success, error);
}

// --8<-- [start:return_conversion]
#[strands::tool(description = "Load records from a source.")]
async fn fetch_records(
    source_id: String,
) -> Result<Vec<strands::ToolResultContent>, strands::ToolError> {
    Ok(vec![
        strands::ToolResultContent::text(format!("Loaded {source_id}")),
        strands::ToolResultContent::json(serde_json::json!({"count": 3})),
    ])
}
// --8<-- [end:return_conversion]

fn main() {}
