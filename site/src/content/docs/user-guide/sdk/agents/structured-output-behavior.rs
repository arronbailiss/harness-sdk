#![allow(dead_code)]

mod defaults {
    // --8<-- [start:defaults]
    use schemars::JsonSchema;
    use serde::Deserialize;
    use strands::{Agent, InvokeOptions};

    #[derive(Debug, Deserialize, JsonSchema)]
    struct CustomerInfo {
        name: String,
        email: String,
    }

    #[derive(Debug, Deserialize, JsonSchema)]
    struct RefundRequest {
        order_id: String,
        amount: f64,
    }

    async fn run() -> strands::Result<()> {
        let mut agent = Agent::builder()
            .structured_output_schema::<CustomerInfo>()
            .build()?;
        let result = agent
            .invoke("This is Dana Ruiz, dana@example.com.")
            .await?;
        let customer: CustomerInfo = result.structured_output()?;

        let result = agent
            .invoke_with(
                "Please refund order 4512. It was $49.99.",
                InvokeOptions::default()
                    .structured_output_schema::<RefundRequest>(),
            )
            .await?;
        let refund: RefundRequest = result.structured_output()?;
        println!("{customer:?}");
        println!("{refund:?}");
        Ok(())
    }
    // --8<-- [end:defaults]
}

mod validation {
    // --8<-- [start:validation]
    use schemars::JsonSchema;
    use serde::Deserialize;
    use strands::{Agent, InvokeOptions};

    #[derive(Debug, Deserialize, JsonSchema)]
    struct FollowUp {
        #[schemars(length(min = 1))]
        action: String,
        #[schemars(range(min = 1, max = 31))]
        due_day: u8,
    }

    async fn run() -> strands::Result<()> {
        let mut agent = Agent::builder().build()?;
        let result = agent
            .invoke_with(
                "Schedule a follow-up call next Tuesday.",
                InvokeOptions::default()
                    .structured_output_schema::<FollowUp>(),
            )
            .await?;
        let follow_up: FollowUp = result.structured_output()?;
        println!("{follow_up:?}");
        Ok(())
    }
    // --8<-- [end:validation]
}

mod attempts {
    // --8<-- [start:attempt_limit]
    use schemars::JsonSchema;
    use serde::Deserialize;
    use strands::{
        Agent,
        InvokeLimits,
        InvokeOptions,
        StopReason,
    };

    #[derive(Debug, Deserialize, JsonSchema)]
    struct Contact {
        name: String,
        email: String,
    }

    async fn run() -> strands::Result<()> {
        let mut agent = Agent::builder().build()?;
        let options = InvokeOptions {
            limits: Some(InvokeLimits {
                structured_output_attempts: Some(3),
                ..InvokeLimits::default()
            }),
            ..InvokeOptions::default()
        }
        .structured_output_schema::<Contact>();
        let result = agent
            .invoke_with(
                "Find the contact details: call Dana tomorrow.",
                options,
            )
            .await?;
        if result.stop_reason()
            == StopReason::LimitStructuredOutputAttempts
        {
            println!("No valid contact after 3 attempts");
        } else {
            let contact: Contact = result.structured_output()?;
            println!("{contact:?}");
        }
        Ok(())
    }
    // --8<-- [end:attempt_limit]
}
