mod credentials {
    // --8<-- [start:credentials]
    use strands::models::bedrock::BedrockModel;

    fn main() {
        let model = BedrockModel::builder()
            .region("us-west-2")
            .build();
        let _ = model;
    }
    // --8<-- [end:credentials]
}

mod basic_usage {
    // --8<-- [start:basic_usage]
    use strands::models::bedrock::BedrockModel;
    use strands::Agent;

    #[tokio::main]
    async fn main() -> strands::Result<()> {
        let model = BedrockModel::builder()
            .model_id("global.anthropic.claude-sonnet-5-5")
            .build();
        let mut agent = Agent::builder()
            .model(model)
            .printer(false)
            .build()?;
        let response = agent.invoke("Tell me about Amazon Bedrock.").await?;
        println!("{}", response.last_message().text());
        Ok(())
    }
    // --8<-- [end:basic_usage]
}

mod custom_configuration {
    // --8<-- [start:custom_configuration]
    use strands::models::bedrock::BedrockModel;
    use strands::Agent;

    fn main() -> strands::Result<()> {
        let model = BedrockModel::builder()
            .model_id("us.amazon.nova-premier-v1:0")
            .temperature(0.3)
            .top_p(0.8)
            .build();
        let agent = Agent::builder()
            .model(model)
            .printer(false)
            .build()?;
        let _ = agent;
        Ok(())
    }
    // --8<-- [end:custom_configuration]
}

mod configuration {
    // --8<-- [start:configuration]
    use std::time::Duration;

    use strands::models::bedrock::BedrockModel;

    fn main() {
        let model = BedrockModel::builder()
            .model_id("global.anthropic.claude-sonnet-5-5")
            .region("us-east-1")
            .max_tokens(4096)
            .stop_sequences(vec!["###".into(), "END".into()])
            .request_timeout(Duration::from_secs(300))
            .build();
        let _ = model;
    }
    // --8<-- [end:configuration]
}

mod streaming {
    // --8<-- [start:streaming]
    use strands::models::bedrock::BedrockModel;

    fn main() {
        let streaming = BedrockModel::builder()
            .model_id("global.anthropic.claude-sonnet-5-5")
            .stream(true)
            .build();
        let non_streaming = BedrockModel::builder()
            .model_id("global.anthropic.claude-sonnet-5-5")
            .stream(false)
            .build();
        let _ = (streaming, non_streaming);
    }
    // --8<-- [end:streaming]
}

mod multimodal {
    // --8<-- [start:multimodal]
    use strands::models::bedrock::BedrockModel;
    use strands::{
        Agent,
        ContentBlock,
        DocumentBlock,
        DocumentFormat,
        DocumentSource,
    };

    #[tokio::main]
    async fn main() -> strands::Result<()> {
        let model = BedrockModel::builder()
            .model_id("global.anthropic.claude-sonnet-5-5")
            .build();
        let mut agent = Agent::builder()
            .model(model)
            .printer(false)
            .build()?;
        let document = DocumentBlock::new(
            "example",
            DocumentFormat::Txt,
            DocumentSource::Text("Once upon a time...".into()),
        );
        let prompt = vec![
            ContentBlock::from(document),
            ContentBlock::text("Tell me about the document."),
        ];
        let response = agent.invoke(prompt).await?;
        println!("{}", response.last_message().text());
        Ok(())
    }
    // --8<-- [end:multimodal]
}

mod s3_location {
    // --8<-- [start:s3_location]
    use strands::models::bedrock::BedrockModel;
    use strands::{
        Agent,
        ContentBlock,
        DocumentBlock,
        DocumentFormat,
        DocumentSource,
        S3Location,
    };

    #[tokio::main]
    async fn main() -> strands::Result<()> {
        let document = DocumentBlock::new(
            "report",
            DocumentFormat::Pdf,
            DocumentSource::Location(
                S3Location::new("s3://my-bucket/documents/report.pdf")
                    .with_bucket_owner("123456789012"),
            ),
        );
        let model = BedrockModel::default();
        let mut agent = Agent::builder()
            .model(model)
            .printer(false)
            .build()?;
        let response = agent
            .invoke(vec![
                ContentBlock::from(document),
                ContentBlock::text("Summarize this document."),
            ])
            .await?;
        println!("{}", response.last_message().text());
        Ok(())
    }
    // --8<-- [end:s3_location]
}

mod guardrails {
    // --8<-- [start:guardrails]
    use strands::models::bedrock::{
        BedrockGuardrailConfig,
        BedrockGuardrailRedactionConfig,
        BedrockGuardrailStreamProcessingMode,
        BedrockGuardrailTrace,
        BedrockModel,
    };

    fn main() {
        let guardrail = BedrockGuardrailConfig {
            guardrail_identifier: "your-guardrail-id".into(),
            guardrail_version: "DRAFT".into(),
            trace: Some(BedrockGuardrailTrace::EnabledFull),
            stream_processing_mode: Some(
                BedrockGuardrailStreamProcessingMode::Sync,
            ),
            redaction: Some(BedrockGuardrailRedactionConfig {
                input: Some(true),
                input_message: Some("Blocked input".into()),
                output: Some(false),
                output_message: None,
            }),
            guard_latest_user_message: Some(true),
        };
        let model = BedrockModel::builder()
            .guardrail_config(guardrail)
            .build();
        let _ = model;
    }
    // --8<-- [end:guardrails]
}

mod update_config {
    // --8<-- [start:update_config]
    use strands::models::bedrock::{
        BedrockModel,
        BedrockModelConfig,
    };

    fn main() -> strands::Result<()> {
        let model = BedrockModel::builder()
            .model_id("global.anthropic.claude-haiku-5-5")
            .max_tokens(4096)
            .build();
        model.update_config(BedrockModelConfig {
            max_tokens: Some(8192),
            ..BedrockModelConfig::default()
        })?;
        Ok(())
    }
    // --8<-- [end:update_config]
}

mod reasoning {
    // --8<-- [start:reasoning]
    use serde_json::json;
    use strands::models::bedrock::BedrockModel;
    use strands::Agent;

    fn main() -> strands::Result<()> {
        let model = BedrockModel::builder()
            .model_id("global.anthropic.claude-opus-5-5")
            .additional_request_fields(json!({
                "thinking": {
                    "type": "adaptive",
                    "display": "summarized"
                },
                "output_config": {"effort": "high"}
            }))
            .build();
        let agent = Agent::builder()
            .model(model)
            .printer(false)
            .build()?;
        let _ = agent;
        Ok(())
    }
    // --8<-- [end:reasoning]
}

mod structured_output {
    // --8<-- [start:structured_output]
    use schemars::JsonSchema;
    use serde::Deserialize;
    use strands::models::bedrock::BedrockModel;
    use strands::{Agent, InvokeOptions};

    #[derive(Deserialize, JsonSchema)]
    struct ProductAnalysis {
        name: String,
        category: String,
        price: f64,
        features: Vec<String>,
        rating: Option<f64>,
    }

    #[tokio::main]
    async fn main() -> strands::Result<()> {
        let model = BedrockModel::default();
        let mut agent = Agent::builder()
            .model(model)
            .printer(false)
            .build()?;
        let options = InvokeOptions::default()
            .structured_output_schema::<ProductAnalysis>();
        let result = agent
            .invoke_with(
                "Analyze the UltraBook Pro, a $1,299 premium laptop.",
                options,
            )
            .await?;
        let product = result.structured_output::<ProductAnalysis>()?;
        println!("{}: ${}", product.name, product.price);
        Ok(())
    }
    // --8<-- [end:structured_output]
}

mod token_counting {
    // --8<-- [start:token_counting]
    use strands::models::bedrock::BedrockModel;

    fn main() {
        let model = BedrockModel::builder()
            .model_id("global.anthropic.claude-sonnet-5-5")
            .use_native_token_count(true)
            .build();
        let _ = model;
    }
    // --8<-- [end:token_counting]
}

mod anthropic_bedrock {
    // --8<-- [start:anthropic_bedrock]
    use strands::models::anthropic::{
        AnthropicBedrockConfig,
        AnthropicModel,
    };
    use strands::Agent;

    fn main() -> strands::Result<()> {
        let options = AnthropicModel::builder()
            .model_id("global.anthropic.claude-sonnet-4-6")
            .max_tokens(4096)
            .build_options();
        let route = AnthropicBedrockConfig::builder()
            .region("us-west-2")
            .build();
        let model = AnthropicModel::bedrock(options, route)?;
        let agent = Agent::builder()
            .model(model)
            .printer(false)
            .build()?;
        let _ = agent;
        Ok(())
    }
    // --8<-- [end:anthropic_bedrock]
}

mod openai_mantle {
    // --8<-- [start:openai_mantle]
    use strands::models::openai::{
        BedrockMantleConfig,
        OpenAIModel,
        OpenAIModelOptions,
    };
    use strands::Agent;

    fn main() -> strands::Result<()> {
        let options = OpenAIModelOptions::builder()
            .model_id("openai.gpt-oss-120b")
            .build_options();
        let route = BedrockMantleConfig::builder()
            .region("us-east-1")
            .build();
        let model = OpenAIModel::bedrock_mantle(options, route)?;
        let agent = Agent::builder()
            .model(model)
            .printer(false)
            .build()?;
        let _ = agent;
        Ok(())
    }
    // --8<-- [end:openai_mantle]
}

mod invalid_model {
    // --8<-- [start:invalid_model]
    use strands::models::bedrock::BedrockModel;
    use strands::Agent;

    fn main() -> strands::Result<()> {
        let model = BedrockModel::builder()
            .model_id("anthropic.claude-3-5-sonnet-20241022-v2:0")
            .build();
        let agent = Agent::builder()
            .model(model)
            .printer(false)
            .build()?;
        let _ = agent;
        Ok(())
    }
    // --8<-- [end:invalid_model]
}

mod arn_cache {
    // --8<-- [start:arn_cache]
    use strands::models::bedrock::BedrockModel;
    use strands::models::{CacheConfig, CacheStrategy};

    fn main() {
        let cache = CacheConfig::builder()
            .strategy(CacheStrategy::Anthropic)
            .build();
        let model = BedrockModel::builder()
            .model_id(
                concat!(
                    "arn:aws:bedrock:us-east-1:123456789012:",
                    "application-inference-profile/abc123",
                ),
            )
            .cache_config(cache)
            .build();
        let _ = model;
    }
    // --8<-- [end:arn_cache]
}
