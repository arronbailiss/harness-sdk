mod basic_usage {
    // --8<-- [start:basic_usage]
    use strands::models::openai::OpenAIModel;
    use strands::Agent;

    fn main() -> strands::Result<()> {
        let model = OpenAIModel::builder()
            .model_id("gpt-5.4-mini")
            .max_tokens(1000)
            .temperature(0.7)
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

mod custom_server {
    // --8<-- [start:custom_server]
    use strands::models::openai::{OpenAIClientConfig, OpenAIModel};

    fn main() -> strands::Result<()> {
        let client = OpenAIClientConfig::builder()
            .base_url("http://localhost:8000/v1")
            .build();
        let model = OpenAIModel::builder()
            .model_id("openai-compatible-model")
            .api_key("local-key")
            .client_config(client)
            .build()?;
        let _ = model;
        Ok(())
    }
    // --8<-- [end:custom_server]
}

mod structured_output {
    // --8<-- [start:structured_output]
    use schemars::JsonSchema;
    use serde::Deserialize;
    use strands::models::openai::OpenAIModel;
    use strands::{Agent, InvokeOptions};

    #[derive(Deserialize, JsonSchema)]
    struct PersonInfo {
        name: String,
        age: u8,
        occupation: String,
    }

    #[tokio::main]
    async fn main() -> strands::Result<()> {
        let model = OpenAIModel::builder()
            .model_id("gpt-5.4-mini")
            .build()?;
        let mut agent = Agent::builder()
            .model(model)
            .printer(false)
            .build()?;
        let options =
            InvokeOptions::default().structured_output_schema::<PersonInfo>();
        let result = agent
            .invoke_with(
                "John Smith is a 30-year-old software engineer.",
                options,
            )
            .await?;
        let person = result.structured_output::<PersonInfo>()?;
        println!("{} is a {}-year-old {}.", person.name, person.age, person.occupation);
        Ok(())
    }
    // --8<-- [end:structured_output]
}
