#![allow(dead_code)]

mod basic {
    // --8<-- [start:basic]
    use strands::Agent;

    #[tokio::main]
    async fn main() -> strands::Result<()> {
        let mut agent = Agent::builder()
            .system_prompt("You are a concise assistant.")
            .build()?;

        agent
            .invoke("What is an agent harness, in one sentence?")
            .await?;
        agent.shutdown().await
    }
    // --8<-- [end:basic]
}

mod custom_tool {
    // --8<-- [start:custom_tool]
    use strands::{Agent, ToolError, tool};

    #[tool(description = "Count occurrences of a letter in a word.")]
    async fn letter_counter(word: String, letter: char) -> Result<usize, ToolError> {
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
