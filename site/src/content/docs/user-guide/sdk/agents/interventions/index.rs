mod basic_usage {
    // --8<-- [start:basic_usage]
    use strands::hooks::BeforeToolCallEvent;
    use strands::interventions::{
        InterventionAction,
        InterventionHandler,
        InterventionResult,
    };
    use strands::{Agent, async_trait};

    struct BlockDelete;

    #[async_trait]
    impl InterventionHandler for BlockDelete {
        fn name(&self) -> &str {
            "block-delete"
        }

        async fn before_tool_call(
            &self,
            event: &mut BeforeToolCallEvent,
        ) -> InterventionResult<BeforeToolCallEvent> {
            if event.tool_use().name == "delete_file" {
                return Ok(InterventionAction::deny("Deleting files is disabled"));
            }
            Ok(InterventionAction::proceed())
        }
    }

    fn main() -> strands::Result<()> {
        let agent = Agent::builder()
            .intervention(BlockDelete)
            .printer(false)
            .build()?;
        let _ = agent;
        Ok(())
    }
    // --8<-- [end:basic_usage]
}

mod action_types {
    // --8<-- [start:action_types]
    use strands::hooks::BeforeToolCallEvent;
    use strands::interventions::InterventionAction;
    use strands::{JsonObject, JsonValue};

    fn actions() {
        let proceed = InterventionAction::<BeforeToolCallEvent>::proceed();
        let deny = InterventionAction::<BeforeToolCallEvent>::deny("Not authorized");
        let guide =
            InterventionAction::<BeforeToolCallEvent>::guide("Use a read-only tool");
        let confirm =
            InterventionAction::<BeforeToolCallEvent>::confirm("Approve this tool call?");
        let transform = InterventionAction::transform(
            |event: &mut BeforeToolCallEvent| {
                event.tool_use_mut().input = JsonValue::Object(JsonObject::new());
            },
        );
        let _ = (proceed, deny, guide, confirm, transform);
    }
    // --8<-- [end:action_types]
}

mod ordered_handlers {
    // --8<-- [start:ordered_handlers]
    use strands::hooks::BeforeToolCallEvent;
    use strands::interventions::{
        InterventionAction,
        InterventionHandler,
        InterventionResult,
    };
    use strands::{Agent, async_trait};

    struct Authorization;
    struct Audit;

    #[async_trait]
    impl InterventionHandler for Authorization {
        fn name(&self) -> &str {
            "authorization"
        }

        async fn before_tool_call(
            &self,
            _event: &mut BeforeToolCallEvent,
        ) -> InterventionResult<BeforeToolCallEvent> {
            Ok(InterventionAction::deny("Not authorized"))
        }
    }

    #[async_trait]
    impl InterventionHandler for Audit {
        fn name(&self) -> &str {
            "audit"
        }
    }

    fn main() -> strands::Result<()> {
        let agent = Agent::builder()
            .intervention(Authorization)
            .intervention(Audit)
            .printer(false)
            .build()?;
        let _ = agent;
        Ok(())
    }
    // --8<-- [end:ordered_handlers]
}

mod error_policy {
    // --8<-- [start:error_policy]
    use strands::interventions::{InterventionHandler, OnError};

    struct Authorization;

    impl InterventionHandler for Authorization {
        fn name(&self) -> &str {
            "authorization"
        }

        fn on_error(&self) -> OnError {
            OnError::Deny
        }
    }
    // --8<-- [end:error_policy]
}

mod confirm_action {
    // --8<-- [start:confirm_action]
    use strands::hooks::BeforeToolCallEvent;
    use strands::interventions::{
        Confirm,
        InterventionAction,
        InterventionHandler,
        InterventionResult,
    };
    use strands::async_trait;

    struct ApproveWrites;

    #[async_trait]
    impl InterventionHandler for ApproveWrites {
        fn name(&self) -> &str {
            "approve-writes"
        }

        async fn before_tool_call(
            &self,
            event: &mut BeforeToolCallEvent,
        ) -> InterventionResult<BeforeToolCallEvent> {
            if event.tool_use().name != "write_file" {
                return Ok(InterventionAction::proceed());
            }
            Ok(Confirm::new("Approve this file write?").into())
        }
    }
    // --8<-- [end:confirm_action]
}
