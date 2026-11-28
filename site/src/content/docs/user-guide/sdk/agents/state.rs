#![allow(dead_code)]

mod history {
    // --8<-- [start:conversation_history]
    async fn run() -> strands::Result<()> {
        let mut agent = strands::Agent::builder().build()?;
        agent.invoke("Hello!").await?;
        println!("{:?}", agent.messages());
        Ok(())
    }
    // --8<-- [end:conversation_history]

    // --8<-- [start:message_initialization]
    use strands::{Agent, Message};

    async fn continue_conversation() -> strands::Result<()> {
        let messages = vec![
            Message::user("Hello, my name is Strands!"),
            Message::assistant("Hi there! How can I help you today?"),
        ];
        let mut agent = Agent::builder().messages(messages).build()?;
        agent.invoke("What's my name?").await?;
        Ok(())
    }
    // --8<-- [end:message_initialization]
}

mod direct_tools {
    // --8<-- [start:direct_tool_calling]
    use strands::{Agent, DirectToolCallOptions};

    #[strands::tool(description = "Save and list notes.")]
    async fn notebook(mode: String) -> String {
        format!("Notebook mode: {mode}")
    }

    async fn run() -> strands::Result<()> {
        let mut agent = Agent::builder().tools([Notebook]).build()?;
        agent
            .tool("notebook")
            .invoke(serde_json::json!({"mode": "create"}))
            .await?;
        agent
            .tool("notebook")
            .invoke_with(
                serde_json::json!({"mode": "list"}),
                DirectToolCallOptions {
                    record_direct_tool_call: Some(false),
                },
            )
            .await?;
        println!("{:?}", agent.messages());
        Ok(())
    }
    // --8<-- [end:direct_tool_calling]
}

mod conversation {
    // --8<-- [start:conversation_manager]
    use strands::Agent;
    use strands::conversation_manager::{
        SlidingWindowConversationManager,
        SlidingWindowConversationManagerConfig,
    };

    fn agent() -> strands::Result<Agent> {
        let manager = SlidingWindowConversationManager::new(
            SlidingWindowConversationManagerConfig {
                window_size: Some(10),
                ..Default::default()
            },
        )?;
        Agent::builder().conversation_manager(manager).build()
    }
    // --8<-- [end:conversation_manager]
}

mod app_state {
    // --8<-- [start:agent_state_basic]
    use strands::{Agent, JsonObject};

    fn agent_state() -> strands::Result<()> {
        let initial: JsonObject = serde_json::from_value(
            serde_json::json!({
                "user_preferences": {"theme": "dark"},
                "session_count": 0
            }),
        )?;
        let agent = Agent::builder().app_state(initial).build()?;

        let preferences =
            agent.app_state().get_value("user_preferences");
        println!("{preferences:?}");
        agent.app_state().set("last_action", "login")?;
        agent.app_state().set("session_count", 1)?;
        println!("{:?}", agent.app_state().get_all());
        agent.app_state().delete("last_action");
        Ok(())
    }
    // --8<-- [end:agent_state_basic]

    // --8<-- [start:state_validation]
    fn validated(agent: &strands::Agent) -> strands::Result<()> {
        agent.app_state().set("string_value", "hello")?;
        agent.app_state().set("number_value", 42)?;
        agent.app_state().set("boolean_value", true)?;
        agent.app_state().set("list_value", [1, 2, 3])?;
        agent
            .app_state()
            .set("dict_value", serde_json::json!({"nested": "data"}))?;
        agent
            .app_state()
            .set("null_value", serde_json::Value::Null)?;
        Ok(())
    }
    // --8<-- [end:state_validation]
}

mod tool_state {
    // --8<-- [start:state_in_tools]
    use strands::{Agent, ToolContext, tool, tool_list};

    #[tool(description = "Track user actions in agent state")]
    async fn track_user_action(
        #[tool(context)] context: &ToolContext,
        action: String,
    ) -> Result<String, strands::ToolError> {
        let state = context.agent().app_state();
        state.update("action_count", |count: Option<u32>| {
            count.unwrap_or_default() + 1
        })?;
        state.set("last_action", &action)?;
        let count = state.get::<u32>("action_count")?.unwrap_or_default();
        Ok(format!(
            "Action '{action}' recorded. Total actions: {count}",
        ))
    }

    #[tool(description = "Get user statistics from agent state")]
    async fn get_user_stats(
        #[tool(context)] context: &ToolContext,
    ) -> Result<String, strands::ToolError> {
        let state = context.agent().app_state();
        let count = state.get::<u32>("action_count")?.unwrap_or_default();
        let last = state
            .get::<String>("last_action")?
            .unwrap_or_else(|| "none".into());
        Ok(format!(
            "Actions performed: {count}, Last action: {last}",
        ))
    }

    fn agent() -> strands::Result<Agent> {
        Agent::builder()
            .tools(tool_list![TrackUserAction, GetUserStats])
            .build()
    }
    // --8<-- [end:state_in_tools]
}

mod invocation {
    // --8<-- [start:invocation_state]
    use strands::{
        Agent,
        InvocationState,
        InvokeOptions,
        ToolContext,
        tool,
    };

    #[derive(Clone)]
    struct RequestContext {
        request_id: String,
        user_id: String,
    }

    #[tool(description = "Return the user ID carried in the invocation state.")]
    async fn whoami(
        #[tool(context)] context: &ToolContext,
    ) -> String {
        context
            .invocation_state()
            .get::<RequestContext>()
            .map_or_else(
                || "Current user: unknown".into(),
                |request| {
                    format!(
                        "Current user: {}, request: {}",
                        request.user_id,
                        request.request_id,
                    )
                },
            )
    }

    async fn run() -> strands::Result<()> {
        let mut agent = Agent::builder().tools([Whoami]).build()?;
        let state = InvocationState::new();
        state.insert(RequestContext {
            request_id: "r-42".into(),
            user_id: "u-1".into(),
        });
        agent
            .invoke_with(
                "Who am I?",
                InvokeOptions {
                    invocation_state: Some(state),
                    ..Default::default()
                },
            )
            .await?;
        Ok(())
    }
    // --8<-- [end:invocation_state]
}
