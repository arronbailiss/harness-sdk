mod basic_usage {
    // --8<-- [start:basic_usage]
    use strands::models::openai::OpenAIModel;
    use strands::Agent;

    fn main() -> strands::Result<()> {
        let model = OpenAIModel::builder()
            .model_id("gpt-4o")
            .api_key("<KEY>")
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

mod client_config {
    // --8<-- [start:client_config]
    use std::time::Duration;

    use strands::models::openai::{OpenAIClientConfig, OpenAIModel};

    fn main() -> strands::Result<()> {
        let client = OpenAIClientConfig::builder()
            .base_url("https://api.openai.com/v1")
            .max_retries(2)
            .timeout(Duration::from_secs(600))
            .build();
        let model = OpenAIModel::builder()
            .model_id("gpt-4o")
            .api_key("<KEY>")
            .client_config(client)
            .build()?;
        let _ = model;
        Ok(())
    }
    // --8<-- [end:client_config]
}

mod web_search {
    // --8<-- [start:web_search]
    use serde_json::json;
    use strands::models::openai::OpenAIModel;
    use strands::Agent;

    fn main() -> strands::Result<()> {
        let model = OpenAIModel::builder()
            .model_id("gpt-4o")
            .api_key("<KEY>")
            .params(json!({"tools": [{"type": "web_search"}]}))
            .build()?;
        let agent = Agent::builder()
            .model(model)
            .printer(false)
            .build()?;
        let _ = agent;
        Ok(())
    }
    // --8<-- [end:web_search]
}

mod file_search {
    // --8<-- [start:file_search]
    use serde_json::json;
    use strands::models::openai::OpenAIModel;
    use strands::Agent;

    fn main() -> strands::Result<()> {
        let model = OpenAIModel::builder()
            .model_id("gpt-4o")
            .api_key("<KEY>")
            .params(json!({
                "tools": [{
                    "type": "file_search",
                    "vector_store_ids": ["vs_abc123"]
                }]
            }))
            .build()?;
        let agent = Agent::builder()
            .model(model)
            .printer(false)
            .build()?;
        let _ = agent;
        Ok(())
    }
    // --8<-- [end:file_search]
}

mod code_interpreter {
    // --8<-- [start:code_interpreter]
    use serde_json::json;
    use strands::models::openai::OpenAIModel;
    use strands::Agent;

    fn main() -> strands::Result<()> {
        let model = OpenAIModel::builder()
            .model_id("gpt-4o")
            .api_key("<KEY>")
            .params(json!({
                "tools": [{
                    "type": "code_interpreter",
                    "container": {"type": "auto"}
                }]
            }))
            .build()?;
        let agent = Agent::builder()
            .model(model)
            .printer(false)
            .build()?;
        let _ = agent;
        Ok(())
    }
    // --8<-- [end:code_interpreter]
}

mod remote_mcp {
    // --8<-- [start:remote_mcp]
    use serde_json::json;
    use strands::models::openai::OpenAIModel;
    use strands::Agent;

    fn main() -> strands::Result<()> {
        let model = OpenAIModel::builder()
            .model_id("gpt-4o")
            .api_key("<KEY>")
            .params(json!({
                "tools": [{
                    "type": "mcp",
                    "server_label": "deepwiki",
                    "server_url": "https://mcp.deepwiki.com/mcp",
                    "require_approval": "never"
                }]
            }))
            .build()?;
        let agent = Agent::builder()
            .model(model)
            .printer(false)
            .build()?;
        let _ = agent;
        Ok(())
    }
    // --8<-- [end:remote_mcp]
}

mod shell {
    // --8<-- [start:shell]
    use serde_json::json;
    use strands::models::openai::OpenAIModel;
    use strands::Agent;

    fn main() -> strands::Result<()> {
        let model = OpenAIModel::builder()
            .model_id("gpt-4o")
            .api_key("<KEY>")
            .params(json!({
                "tools": [{
                    "type": "shell",
                    "environment": {"type": "container_auto"}
                }]
            }))
            .build()?;
        let agent = Agent::builder()
            .model(model)
            .printer(false)
            .build()?;
        let _ = agent;
        Ok(())
    }
    // --8<-- [end:shell]
}

mod stateful {
    // --8<-- [start:stateful]
    use strands::models::openai::OpenAIModel;
    use strands::Agent;

    fn main() -> strands::Result<()> {
        let model = OpenAIModel::builder()
            .model_id("gpt-4o")
            .api_key("<KEY>")
            .stateful(true)
            .build()?;
        let agent = Agent::builder()
            .model(model)
            .printer(false)
            .build()?;
        let _ = agent;
        Ok(())
    }
    // --8<-- [end:stateful]
}

mod prompt_caching {
    // --8<-- [start:prompt_caching]
    use strands::models::openai::OpenAIModel;
    use strands::models::CacheConfig;

    fn main() -> strands::Result<()> {
        let cache = CacheConfig::builder()
            .messages_ttl("12h")
            .build();
        let model = OpenAIModel::builder()
            .model_id("gpt-6-sol")
            .api_key("<KEY>")
            .cache_config(cache)
            .build()?;
        let _ = model;
        Ok(())
    }
    // --8<-- [end:prompt_caching]
}

mod prompt_caching_alias {
    // --8<-- [start:prompt_caching_alias]
    use strands::models::openai::OpenAIModel;
    use strands::models::{
        CacheConfig,
        ModelCapabilities,
        ModelInfo,
        PromptCaching,
    };

    fn main() -> strands::Result<()> {
        let info = ModelInfo {
            capabilities: ModelCapabilities {
                prompt_caching: Some(PromptCaching::CachePoints),
                ..ModelCapabilities::default()
            },
            ..ModelInfo::default()
        };
        let model = OpenAIModel::builder()
            .model_id("my-proxy-alias")
            .api_key("<KEY>")
            .cache_config(CacheConfig::default())
            .model_info(info)
            .build()?;
        let _ = model;
        Ok(())
    }
    // --8<-- [end:prompt_caching_alias]
}
