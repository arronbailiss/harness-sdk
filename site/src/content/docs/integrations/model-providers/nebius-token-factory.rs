mod usage {
    // --8<-- [start:usage]
    use strands::models::openai::{
        OpenAIApi,
        OpenAIClientConfig,
        OpenAIModel,
    };
    use strands::Agent;

    #[tokio::main]
    async fn main() -> strands::Result<()> {
        let api_key =
            std::env::var("NEBIUS_API_KEY").expect("NEBIUS_API_KEY must be set");
        let client = OpenAIClientConfig::builder()
            .base_url("https://api.tokenfactory.nebius.com/v1/")
            .build();
        let model = OpenAIModel::builder()
            .api(OpenAIApi::Chat)
            .api_key(api_key)
            .client_config(client)
            .model_id("deepseek-ai/DeepSeek-R1-0528")
            .build()?;
        let mut agent = Agent::builder()
            .model(model)
            .printer(false)
            .build()?;
        let response = agent
            .invoke("Explain tool calling in one sentence.")
            .await?;
        println!("{response}");
        agent.shutdown().await
    }
    // --8<-- [end:usage]
}
