#![allow(dead_code)]
#![allow(clippy::print_stdout)]

mod request_handler {
    // --8<-- [start:request_handler]
    use strands::Agent;

    async fn chat(prompt: &str) -> strands::Result<String> {
        let mut agent = Agent::builder().printer(false).build()?;
        let result = agent.invoke(prompt).await?;
        let reply = result.to_string();
        agent.shutdown().await?;
        Ok(reply)
    }
    // --8<-- [end:request_handler]
}

mod provider_bedrock {
    // --8<-- [start:provider_bedrock]
    use strands::Agent;

    #[tokio::main]
    async fn main() -> strands::Result<()> {
        let mut agent = Agent::builder().printer(false).build()?;
        let result = agent.invoke("What can you help me build?").await?;
        println!("{result}");
        agent.shutdown().await
    }
    // --8<-- [end:provider_bedrock]
}

mod provider_anthropic {
    // --8<-- [start:provider_anthropic]
    use strands::Agent;
    use strands::models::anthropic::AnthropicModel;

    #[tokio::main]
    async fn main() -> strands::Result<()> {
        let model = AnthropicModel::builder()
            .api_key("<KEY>")
            .model_id("claude-sonnet-5")
            .build()?;
        let mut agent = Agent::builder().model(model).printer(false).build()?;
        let result = agent.invoke("What can you help me build?").await?;
        println!("{result}");
        agent.shutdown().await
    }
    // --8<-- [end:provider_anthropic]
}

mod provider_openai {
    // --8<-- [start:provider_openai]
    use strands::Agent;
    use strands::models::openai::OpenAIModel;

    #[tokio::main]
    async fn main() -> strands::Result<()> {
        let model = OpenAIModel::builder()
            .api_key("<KEY>")
            .model_id("gpt-5.4")
            .build()?;
        let mut agent = Agent::builder().model(model).printer(false).build()?;
        let result = agent.invoke("What can you help me build?").await?;
        println!("{result}");
        agent.shutdown().await
    }
    // --8<-- [end:provider_openai]
}
