mod triage {
    // --8<-- [start:triage]
    use schemars::JsonSchema;
    use serde::Deserialize;
    use strands::{Agent, InvokeOptions};

    #[derive(Debug, Deserialize, JsonSchema)]
    #[serde(rename_all = "lowercase")]
    enum Category {
        Billing,
        Bug,
        Account,
        Other,
    }

    #[derive(Debug, Deserialize, JsonSchema)]
    #[serde(rename_all = "lowercase")]
    enum Priority {
        Low,
        Medium,
        High,
    }

    #[derive(Debug, Deserialize, JsonSchema)]
    /// Routing decision for one customer support message.
    struct TicketTriage {
        /// The team that should handle the message
        category: Category,
        /// high when the customer is blocked or losing money
        priority: Priority,
        /// One sentence describing the problem
        summary: String,
    }

    #[tokio::main]
    async fn main() -> strands::Result<()> {
        let mut agent = Agent::builder().printer(false).build()?;
        let result = agent
            .invoke_with(
                "I was charged twice this month and need one of the charges refunded.",
                InvokeOptions::default().structured_output_schema::<TicketTriage>(),
            )
            .await?;
        let ticket: TicketTriage = result.structured_output()?;
        let priority = format!("{:?}", ticket.priority).to_ascii_lowercase();
        let category = format!("{:?}", ticket.category).to_ascii_lowercase();
        println!("{priority} | {category} | {}", ticket.summary);
        agent.shutdown().await
    }
    // --8<-- [end:triage]
}

mod with_tools {
    // --8<-- [start:with_tools]
    use schemars::JsonSchema;
    use serde::Deserialize;
    use serde_json::json;
    use strands::{Agent, InvokeOptions, ToolError, tool};

    #[tool(description = "Look up a customer's name and plan by email address.")]
    async fn lookup_customer(email: String) -> Result<serde_json::Value, ToolError> {
        Ok(match email.as_str() {
            "dana@example.com" => json!({"name": "Dana Ruiz", "plan": "enterprise"}),
            _ => json!({"name": "unknown", "plan": "free"}),
        })
    }

    #[derive(Debug, Deserialize, JsonSchema)]
    struct CustomerTicket {
        customer_name: String,
        plan: String,
        priority: String,
    }

    async fn classify() -> strands::Result<CustomerTicket> {
        let mut agent = Agent::builder().tools([LookupCustomer]).printer(false).build()?;
        let result = agent
            .invoke_with(
                "From dana@example.com: our dashboard has been down for an hour.",
                InvokeOptions::default().structured_output_schema::<CustomerTicket>(),
            )
            .await?;
        let ticket = result.structured_output()?;
        agent.shutdown().await?;
        Ok(ticket)
    }
    // --8<-- [end:with_tools]
}

mod from_history {
    // --8<-- [start:from_history]
    use schemars::JsonSchema;
    use serde::Deserialize;
    use strands::{Agent, InvokeOptions};

    #[derive(Deserialize, JsonSchema)]
    struct Escalation {
        problem: String,
        steps_tried: Vec<String>,
    }

    async fn summarize() -> strands::Result<Escalation> {
        let mut agent = Agent::builder().printer(false).build()?;
        agent.invoke("Hi, I can't log in to my account.").await?;
        agent
            .invoke("I reset my password twice, and the reset email never arrives.")
            .await?;
        let result = agent
            .invoke_with(
                "Summarize this conversation for the escalation team.",
                InvokeOptions::default().structured_output_schema::<Escalation>(),
            )
            .await?;
        let escalation = result.structured_output()?;
        agent.shutdown().await?;
        Ok(escalation)
    }
    // --8<-- [end:from_history]
}

mod streaming {
    // --8<-- [start:streaming]
    use futures::StreamExt;
    use schemars::JsonSchema;
    use serde::Deserialize;
    use strands::{Agent, AgentStreamEvent, InvokeOptions};

    #[derive(Deserialize, JsonSchema)]
    struct ReplyDraft {
        subject: String,
        body: String,
    }

    #[tokio::main]
    async fn main() -> strands::Result<()> {
        let mut agent = Agent::builder().printer(false).build()?;
        {
            let options =
                InvokeOptions::default().structured_output_schema::<ReplyDraft>();
            let prompt = "Draft a reply apologizing for a delayed refund.";
            let mut stream = agent.stream_with(prompt, options);
            while let Some(event) = stream.next().await {
                if let AgentStreamEvent::AgentResult(result) = event? {
                    let draft: ReplyDraft = result.structured_output()?;
                    println!("Subject: {}", draft.subject);
                    println!("{}", draft.body);
                }
            }
        }
        agent.shutdown().await
    }
    // --8<-- [end:streaming]
}
