mod usage {
    // --8<-- [start:usage]
    use serde_json::json;
    use strands::models::openai::{OpenAIApi, OpenAIClientConfig, OpenAIModel};
    use strands::Agent;

    #[tokio::main]
    async fn main() -> strands::Result<()> {
        let api_key =
            std::env::var("COHERE_API_KEY").expect("COHERE_API_KEY must be set");
        let client = OpenAIClientConfig::builder()
            .base_url("https://api.cohere.ai/compatibility/v1")
            .build();
        let model = OpenAIModel::builder()
            .api(OpenAIApi::Chat)
            .api_key(api_key)
            .client_config(client)
            .model_id("command-a-03-2025")
            .params(json!({"stream_options": null}))
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
