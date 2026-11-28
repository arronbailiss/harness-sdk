#![allow(dead_code, clippy::print_stdout)]

mod keep_model {
    // --8<-- [start:keep_model]
    use strands::Agent;
    use strands::models::openai::{OpenAIApi, OpenAIModel};

    fn agent() -> strands::Result<Agent> {
        let model = OpenAIModel::builder()
            .api(OpenAIApi::Chat)
            .model_id("gpt-5.4")
            .build()?;
        Agent::builder().model(model).printer(false).build()
    }
    // --8<-- [end:keep_model]
}

mod after_loop {
    // --8<-- [start:after_loop]
    use strands::models::openai::{OpenAIApi, OpenAIModel};
    use strands::{Agent, ToolError, tool};

    #[tool(description = "Get the weather for a city.")]
    async fn get_weather(city: String) -> Result<String, ToolError> {
        Ok(format!("It's sunny in {city}."))
    }

    fn agent() -> strands::Result<Agent> {
        let model = OpenAIModel::builder()
            .api(OpenAIApi::Chat)
            .model_id("gpt-5.4")
            .build()?;
        Agent::builder()
            .model(model)
            .tools([GetWeather])
            .printer(false)
            .build()
    }

    async fn run(prompt: &str) -> strands::Result<String> {
        let mut agent = agent()?;
        let result = agent.invoke(prompt).await?.to_string();
        agent.shutdown().await?;
        Ok(result)
    }
    // --8<-- [end:after_loop]
}

mod agents_sdk {
    // --8<-- [start:agents_sdk]
    use strands::models::openai::{OpenAIApi, OpenAIModel};
    use strands::vended_interventions::hitl::{
        HumanInTheLoop,
        HumanInTheLoopAsk,
        HumanInTheLoopConfig,
    };
    use strands::{Agent, InvokeLimits, InvokeOptions, tool};

    #[tool(description = "Search the product docs.")]
    async fn search_docs(query: String) -> String {
        format!("results for {query}")
    }

    async fn run() -> strands::Result<()> {
        let support = Agent::builder()
            .name("support")
            .description("Answers product questions using the docs.")
            .system_prompt("Answer product questions.")
            .tools([SearchDocs])
            .printer(false)
            .build()?;
        let model = OpenAIModel::builder()
            .api(OpenAIApi::Chat)
            .model_id("gpt-5.4")
            .build()?;
        let approval = HumanInTheLoop::new(HumanInTheLoopConfig {
            ask: Some(HumanInTheLoopAsk::Stdio),
            ..HumanInTheLoopConfig::default()
        });
        let mut triage = Agent::builder()
            .model(model)
            .system_prompt("Route the user to the right specialist.")
            .tools([support])
            .interventions([approval])
            .printer(false)
            .build()?;
        let result = triage
            .invoke_with(
                "How do I reset my password?",
                InvokeOptions {
                    limits: Some(InvokeLimits {
                        turns: Some(8),
                        ..InvokeLimits::default()
                    }),
                    ..InvokeOptions::default()
                },
            )
            .await?;
        println!("{result}");
        triage.shutdown().await
    }
    // --8<-- [end:agents_sdk]
}
