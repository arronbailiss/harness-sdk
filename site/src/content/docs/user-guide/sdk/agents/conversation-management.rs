mod null_manager {
    // --8<-- [start:null_manager]
    use strands::conversation_manager::NullConversationManager;
    use strands::Agent;

    fn main() -> strands::Result<()> {
        let agent = Agent::builder()
            .conversation_manager(NullConversationManager::new())
            .printer(false)
            .build()?;
        let _ = agent;
        Ok(())
    }
    // --8<-- [end:null_manager]
}

mod sliding_window {
    // --8<-- [start:sliding_window]
    use strands::conversation_manager::{
        SlidingWindowConversationManager,
        SlidingWindowConversationManagerConfig,
    };
    use strands::Agent;

    fn main() -> strands::Result<()> {
        let manager =
            SlidingWindowConversationManager::new(SlidingWindowConversationManagerConfig {
                window_size: Some(20),
                should_truncate_results: Some(true),
                ..Default::default()
            })?;
        let agent = Agent::builder()
            .conversation_manager(manager)
            .printer(false)
            .build()?;
        let _ = agent;
        Ok(())
    }
    // --8<-- [end:sliding_window]
}

mod summarizing_basic {
    // --8<-- [start:summarizing_basic]
    use strands::conversation_manager::{
        SummarizingConversationManager,
        SummarizingConversationManagerConfig,
    };
    use strands::Agent;

    fn main() -> strands::Result<()> {
        let config = SummarizingConversationManagerConfig::default();
        let manager = SummarizingConversationManager::new(config)?;
        let agent = Agent::builder()
            .conversation_manager(manager)
            .printer(false)
            .build()?;
        let _ = agent;
        Ok(())
    }
    // --8<-- [end:summarizing_basic]
}

mod summarizing_custom {
    // --8<-- [start:summarizing_custom]
    use strands::conversation_manager::{
        SummarizingConversationManager,
        SummarizingConversationManagerConfig,
    };
    use strands::Agent;

    fn main() -> strands::Result<()> {
        let manager =
            SummarizingConversationManager::new(SummarizingConversationManagerConfig {
                summary_ratio: Some(0.3),
                preserve_recent_messages: Some(10),
                ..Default::default()
            })?;
        let agent = Agent::builder()
            .conversation_manager(manager)
            .printer(false)
            .build()?;
        let _ = agent;
        Ok(())
    }
    // --8<-- [end:summarizing_custom]
}

mod custom_prompt {
    // --8<-- [start:custom_prompt]
    use strands::conversation_manager::{
        SummarizingConversationManager,
        SummarizingConversationManagerConfig,
    };

    fn main() -> strands::Result<()> {
        let manager =
            SummarizingConversationManager::new(SummarizingConversationManagerConfig {
                summarization_system_prompt: Some([
                    "Preserve code changes, file paths,",
                    "and architectural decisions.",
                ]
                .join(" ")),
                ..Default::default()
            })?;
        let _ = manager;
        Ok(())
    }
    // --8<-- [end:custom_prompt]
}

mod pin_messages {
    // --8<-- [start:pin_messages]
    use strands::conversation_manager::{
        SlidingWindowConversationManager,
        SlidingWindowConversationManagerConfig,
    };

    fn main() -> strands::Result<()> {
        let manager =
            SlidingWindowConversationManager::new(SlidingWindowConversationManagerConfig {
                window_size: Some(40),
                pin_first: Some(1),
                ..Default::default()
            })?;
        let _ = manager;
        Ok(())
    }
    // --8<-- [end:pin_messages]
}

mod proactive_compression {
    // --8<-- [start:proactive_compression]
    use strands::conversation_manager::{
        ProactiveCompression,
        SlidingWindowConversationManager,
        SlidingWindowConversationManagerConfig,
    };

    fn main() -> strands::Result<()> {
        let manager =
            SlidingWindowConversationManager::new(SlidingWindowConversationManagerConfig {
                window_size: Some(50),
                proactive_compression: Some(ProactiveCompression::Threshold(0.7)),
                ..Default::default()
            })?;
        let _ = manager;
        Ok(())
    }
    // --8<-- [end:proactive_compression]
}

mod custom_manager {
    // --8<-- [start:custom_manager]
    use strands::async_trait;
    use strands::conversation_manager::{
        ConversationManager,
        ConversationManagerReduceOptions,
    };

    struct LastTenMessages;

    #[async_trait]
    impl ConversationManager for LastTenMessages {
        fn name(&self) -> &str {
            "example:last-ten-messages"
        }

        async fn reduce(
            &self,
            options: ConversationManagerReduceOptions<'_>,
        ) -> strands::Result<bool> {
            let messages = options.agent.messages_mut();
            if messages.len() <= 10 {
                return Ok(false);
            }
            messages.drain(..messages.len() - 10);
            Ok(true)
        }
    }
    // --8<-- [end:custom_manager]
}
