//! Compile-checked snippets used by the Rust SDK quickstart pages.

#![allow(dead_code)]
#![allow(clippy::print_stdout, reason = "docs examples report results to the terminal")]

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

mod basic {
    // --8<-- [start:basic]
    use strands::Agent;

    #[tokio::main]
    async fn main() -> strands::Result<()> {
        let mut agent = Agent::builder().system_prompt("You are a concise assistant.").build()?;

        agent.invoke("What is an agent harness, in one sentence?").await?;
        agent.shutdown().await
    }
    // --8<-- [end:basic]
}

mod custom_tool {
    // --8<-- [start:custom_tool]
    use strands::{Agent, ToolError, tool};

    /// Count occurrences of a letter in a word.
    #[tool]
    async fn letter_counter(
        /// Word to search.
        word: String,
        /// Single letter to count.
        letter: char,
    ) -> Result<usize, ToolError> {
        Ok(word
            .chars()
            .filter(|candidate| candidate.eq_ignore_ascii_case(&letter))
            .count())
    }

    #[tokio::main]
    async fn main() -> strands::Result<()> {
        let mut agent = Agent::builder().tools([LetterCounter]).build()?;
        agent
            .invoke(r#"How many letter R's are in the word "strawberry"?"#)
            .await?;
        agent.shutdown().await
    }
    // --8<-- [end:custom_tool]
}

fn main() {}
