mod explicit_cache_config {
    // --8<-- [start:explicit_cache_config]
    use strands::models::bedrock::BedrockModel;
    use strands::models::{CacheConfig, CacheStrategy};
    use strands::Agent;

    fn main() -> strands::Result<()> {
        let cache = CacheConfig::builder()
            .strategy(CacheStrategy::Auto)
            .build();
        let model = BedrockModel::builder()
            .cache_config(cache)
            .build();
        let agent = Agent::builder()
            .model(model)
            .build()?;
        let _ = agent;
        Ok(())
    }
    // --8<-- [end:explicit_cache_config]
}

mod anthropic_cache_strategy {
    // --8<-- [start:anthropic_cache_strategy]
    use strands::models::bedrock::BedrockModel;
    use strands::models::{CacheConfig, CacheStrategy};

    fn main() {
        let cache = CacheConfig::builder()
            .strategy(CacheStrategy::Anthropic)
            .build();
        let model = BedrockModel::builder()
            .model_id("custom-application-inference-profile")
            .cache_config(cache)
            .build();
        let _ = model;
    }
    // --8<-- [end:anthropic_cache_strategy]
}

mod converse_implicit_caching_off {
    // --8<-- [start:converse_implicit_caching_off]
    use serde_json::json;
    use strands::models::bedrock::BedrockModel;
    use strands::Agent;

    fn main() -> strands::Result<()> {
        let model = BedrockModel::builder()
            .model_id("us.openai.gpt-6-luna")
            .additional_request_fields(json!({
                "prompt_cache_options": {"mode": "explicit"}
            }))
            .build();
        let agent = Agent::builder()
            .model(model)
            .build()?;
        let _ = agent;
        Ok(())
    }
    // --8<-- [end:converse_implicit_caching_off]
}
