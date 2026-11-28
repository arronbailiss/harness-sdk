mod system_prompt_caching {
    // --8<-- [start:system_prompt_caching]
    use strands::{
        Agent,
        CachePointBlock,
        SystemContentBlock,
    };

    fn main() -> strands::Result<()> {
        let system_prompt = vec![
            SystemContentBlock::text(
                "You are a helpful assistant...".repeat(1600),
            ),
            CachePointBlock::new().into(),
        ];
        let agent = Agent::builder()
            .system_prompt(system_prompt)
            .build()?;
        let _ = agent;
        Ok(())
    }
    // --8<-- [end:system_prompt_caching]
}

mod tool_caching {
    // --8<-- [start:tool_caching]
    use strands::models::bedrock::BedrockModel;
    use strands::models::CacheConfig;

    fn main() {
        let cache = CacheConfig::builder()
            .tools_ttl(true)
            .build();
        let model = BedrockModel::builder()
            .model_id("global.anthropic.claude-sonnet-5")
            .cache_config(cache)
            .build();
        let _ = model;
    }
    // --8<-- [end:tool_caching]
}

mod automatic_cache_strategy {
    // --8<-- [start:automatic_cache_strategy]
    use strands::models::bedrock::BedrockModel;
    use strands::models::{CacheConfig, CacheStrategy};
    use strands::Agent;

    fn main() -> strands::Result<()> {
        let cache = CacheConfig::builder()
            .strategy(CacheStrategy::Auto)
            .build();
        let model = BedrockModel::builder()
            .model_id("global.anthropic.claude-sonnet-5")
            .cache_config(cache)
            .build();
        let agent = Agent::builder()
            .model(model)
            .build()?;
        let _ = agent;
        Ok(())
    }
    // --8<-- [end:automatic_cache_strategy]
}

mod messages_caching {
    // --8<-- [start:messages_caching]
    use strands::{
        Agent,
        CachePointBlock,
        ContentBlock,
        Message,
        Role,
    };

    fn main() -> strands::Result<()> {
        let messages = vec![Message::new(
            Role::User,
            vec![
                ContentBlock::text(
                    "Here is a long technical document...",
                ),
                CachePointBlock::new().into(),
            ],
        )];
        let agent = Agent::builder()
            .messages(messages)
            .build()?;
        let _ = agent;
        Ok(())
    }
    // --8<-- [end:messages_caching]
}

mod cache_metrics {
    // --8<-- [start:cache_metrics]
    use strands::AgentResult;

    fn print_cache_metrics(result: &AgentResult) {
        let usage = result
            .metrics()
            .map(|metrics| &metrics.accumulated_usage);
        let cache_write = usage
            .and_then(|usage| usage.cache_write_input_tokens)
            .unwrap_or_default();
        let cache_read = usage
            .and_then(|usage| usage.cache_read_input_tokens)
            .unwrap_or_default();
        println!("Cache write tokens: {cache_write}");
        println!("Cache read tokens: {cache_read}");
    }
    // --8<-- [end:cache_metrics]
}
