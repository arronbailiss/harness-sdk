//! Compile-checked snippets used by the Rust harness overview and quickstart.

#![allow(dead_code)]

mod basic {
    // --8<-- [start:basic]
    use strands_harness::{HarnessAgentOptions, create_harness};

    #[tokio::main]
    async fn main() -> strands::Result<()> {
        let mut agent = create_harness(HarnessAgentOptions::default()).await?;
        agent
            .invoke(
                "Research the top three vector databases, compare pricing and limits, \
                 and write it up in comparison.md",
            )
            .await?;
        agent.shutdown().await
    }
    // --8<-- [end:basic]
}

mod bedrock {
    // --8<-- [start:bedrock]
    use strands_harness::{HarnessAgentOptions, create_harness};

    #[tokio::main]
    async fn main() -> strands::Result<()> {
        let mut agent = create_harness(HarnessAgentOptions::default()).await?;
        agent
            .invoke(
                "Research the three most common strategies for versioning a \
                 REST API, compare their tradeoffs, and write a recommendation \
                 to api-versioning.md",
            )
            .await?;
        agent.shutdown().await
    }
    // --8<-- [end:bedrock]
}

mod anthropic {
    // --8<-- [start:anthropic]
    use strands_harness::{HarnessAgentOptions, create_harness};

    #[tokio::main]
    async fn main() -> strands::Result<()> {
        let options = HarnessAgentOptions::builder()
            .model("anthropic/claude-sonnet-5")
            .build();
        let mut agent = create_harness(options).await?;
        agent
            .invoke(
                "Research the three most common strategies for versioning a \
                 REST API, compare their tradeoffs, and write a recommendation \
                 to api-versioning.md",
            )
            .await?;
        agent.shutdown().await
    }
    // --8<-- [end:anthropic]
}

mod openai {
    // --8<-- [start:openai]
    use strands_harness::{HarnessAgentOptions, create_harness};

    #[tokio::main]
    async fn main() -> strands::Result<()> {
        let options = HarnessAgentOptions::builder().model("openai/gpt-5.4").build();
        let mut agent = create_harness(options).await?;
        agent
            .invoke(
                "Research the three most common strategies for versioning a \
                 REST API, compare their tradeoffs, and write a recommendation \
                 to api-versioning.md",
            )
            .await?;
        agent.shutdown().await
    }
    // --8<-- [end:openai]
}

mod session {
    // --8<-- [start:session]
    use strands_harness::{HarnessAgentOptions, SessionConfig, create_harness};

    #[tokio::main]
    async fn main() -> strands::Result<()> {
        let session = SessionConfig::builder().id("api-design").build();
        let options = HarnessAgentOptions::builder().session(session).build();
        let mut agent = create_harness(options).await?;
        agent
            .invoke(
                "Which versioning strategy would you pick for an API with \
                 external customers, and why?",
            )
            .await?;
        agent.shutdown().await
    }
    // --8<-- [end:session]
}

fn main() {}
