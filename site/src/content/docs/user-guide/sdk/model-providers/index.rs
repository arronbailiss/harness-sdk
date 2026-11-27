mod basic_usage {
    // --8<-- [start:basic_usage]
    use strands::Agent;
    use strands::models::bedrock::BedrockModel;
    use strands::models::openai::OpenAIModel;

    #[tokio::main]
    async fn main() -> strands::Result<()> {
        let bedrock = BedrockModel::builder()
            .model_id("global.anthropic.claude-sonnet-5-5")
            .build();
        let mut agent = Agent::builder().model(bedrock).printer(false).build()?;
        println!("{}", agent.invoke("What can you help me with?").await?);
        agent.shutdown().await?;

        let openai = OpenAIModel::builder()
            .api_key("<KEY>")
            .model_id("gpt-5.4")
            .build()?;
        let mut agent = Agent::builder().model(openai).printer(false).build()?;
        println!("{}", agent.invoke("What can you help me with?").await?);
        agent.shutdown().await
    }
    // --8<-- [end:basic_usage]
}

mod auxiliary_model {
    // --8<-- [start:auxiliary_model]
    use strands::Agent;
    use strands::context_manager::ContextManagerStrategy;
    use strands::models::bedrock::BedrockModel;

    fn agent() -> strands::Result<Agent> {
        let main_model = BedrockModel::builder()
            .model_id("global.anthropic.claude-opus-5-20260301-v1:0")
            .build();
        let auxiliary_model = BedrockModel::builder()
            .model_id("global.anthropic.claude-haiku-4-5-20251001-v1:0")
            .build();

        Agent::builder()
            .model(main_model)
            .aux_model(auxiliary_model)
            .context_manager(ContextManagerStrategy::Auto)
            .build()
    }
    // --8<-- [end:auxiliary_model]
}

mod model_info {
    // --8<-- [start:model_info]
    use strands::models::bedrock::BedrockModel;
    use strands::{ModelInfo, ModelLimits};

    fn model() -> BedrockModel {
        BedrockModel::builder()
            .model_id(
                concat!(
                    "arn:aws:bedrock:us-east-1:123456789012:",
                    "application-inference-profile/a1b2c3",
                ),
            )
            .model_info(ModelInfo {
                limits: ModelLimits {
                    context_window: Some(1_000_000),
                    max_output_tokens: Some(64_000),
                },
                ..ModelInfo::default()
            })
            .build()
    }
    // --8<-- [end:model_info]
}
