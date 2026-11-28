mod basic_usage {
    // --8<-- [start:basic_usage]
    use strands::models::anthropic::AnthropicModel;
    use strands::Agent;

    fn main() -> strands::Result<()> {
        let model = AnthropicModel::builder()
            .model_id("claude-sonnet-4-6")
            .max_tokens(1024)
            .build()?;
        let agent = Agent::builder()
            .model(model)
            .printer(false)
            .build()?;
        let _ = agent;
        Ok(())
    }
    // --8<-- [end:basic_usage]
}

mod structured_output {
    // --8<-- [start:structured_output]
    use schemars::JsonSchema;
    use serde::Deserialize;
    use strands::models::anthropic::AnthropicModel;
    use strands::{Agent, InvokeOptions};

    #[derive(Deserialize, JsonSchema)]
    struct MovieReview {
        title: String,
        rating: u8,
        genre: String,
        sentiment: String,
    }

    #[tokio::main]
    async fn main() -> strands::Result<()> {
        let model = AnthropicModel::builder()
            .model_id("claude-sonnet-4-6")
            .build()?;
        let mut agent = Agent::builder()
            .model(model)
            .printer(false)
            .build()?;
        let options =
            InvokeOptions::default().structured_output_schema::<MovieReview>();
        let result = agent
            .invoke_with("The Matrix is a 9/10 science-fiction classic.", options)
            .await?;
        let review = result.structured_output::<MovieReview>()?;
        println!(
            "{}: {}/10, {}, {}",
            review.title,
            review.rating,
            review.genre,
            review.sentiment,
        );
        Ok(())
    }
    // --8<-- [end:structured_output]
}

mod built_in_tools {
    // --8<-- [start:built_in_tools]
    use serde_json::json;
    use strands::models::anthropic::AnthropicModel;

    fn main() -> strands::Result<()> {
        let model = AnthropicModel::builder()
            .model_id("claude-sonnet-4-6")
            .anthropic_tools(vec![json!({
                "type": "web_search_20260318",
                "name": "web_search",
                "max_uses": 5
            })])
            .build()?;
        let _ = model;
        Ok(())
    }
    // --8<-- [end:built_in_tools]
}

mod prompt_caching {
    // --8<-- [start:prompt_caching]
    use strands::models::anthropic::AnthropicModel;
    use strands::models::CacheConfig;

    fn main() -> strands::Result<()> {
        let cache = CacheConfig::builder()
            .ttl("1h")
            .tools_ttl("1h")
            .build();
        let model = AnthropicModel::builder()
            .model_id("claude-sonnet-4-6")
            .cache_config(cache)
            .build()?;
        let _ = model;
        Ok(())
    }
    // --8<-- [end:prompt_caching]
}

mod server_side_summary {
    // --8<-- [start:server_side_summary]
    use strands::context_manager::{
        ContextManagerConfig,
        ContextManagerStrategy,
        StrategyPresetName,
    };
    use strands::models::anthropic::AnthropicModel;
    use strands::Agent;

    fn main() -> strands::Result<()> {
        let model = AnthropicModel::builder()
            .model_id("claude-opus-5-5")
            .build()?;
        let context = ContextManagerConfig {
            strategies: Some(vec![
                StrategyPresetName::ProactiveSummarization.into(),
            ]),
            ..ContextManagerConfig::default()
        };
        let agent = Agent::builder()
            .model(model)
            .context_manager(ContextManagerStrategy::Config(context))
            .build()?;
        let _ = agent;
        Ok(())
    }
    // --8<-- [end:server_side_summary]
}

mod token_counting {
    // --8<-- [start:token_counting]
    use strands::models::anthropic::AnthropicModel;

    fn main() -> strands::Result<()> {
        let model = AnthropicModel::builder()
            .model_id("claude-sonnet-4-6")
            .use_native_token_count(true)
            .build()?;
        let _ = model;
        Ok(())
    }
    // --8<-- [end:token_counting]
}
