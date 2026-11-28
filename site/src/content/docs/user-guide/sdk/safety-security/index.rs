#![allow(dead_code)]

fn guardrail_agent() -> strands::Result<strands::Agent> {
    // --8<-- [start:guardrail_agent]
    use strands::models::bedrock::{
        BedrockGuardrailConfig,
        BedrockModel,
    };

    let guardrail = BedrockGuardrailConfig {
        guardrail_identifier: "your-guardrail-id".into(),
        guardrail_version: "1".into(),
        ..BedrockGuardrailConfig::default()
    };
    let model = BedrockModel::builder()
        .guardrail_config(guardrail)
        .build();

    strands::Agent::builder().model(model).build()
    // --8<-- [end:guardrail_agent]
}
