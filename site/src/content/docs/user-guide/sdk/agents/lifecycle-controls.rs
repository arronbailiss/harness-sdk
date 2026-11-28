#![allow(dead_code)]

mod limits {
    // --8<-- [start:limits]
    use strands::{
        Agent,
        InvokeLimits,
        InvokeOptions,
        StopReason,
    };

    async fn run() -> strands::Result<()> {
        let mut agent = Agent::builder().build()?;
        let result = agent
            .invoke_with(
                "Summarize this document.",
                InvokeOptions {
                    limits: Some(InvokeLimits {
                        turns: Some(5),
                        output_tokens: Some(2_000),
                        total_tokens: Some(10_000),
                        ..InvokeLimits::default()
                    }),
                    ..InvokeOptions::default()
                },
            )
            .await?;

        match result.stop_reason() {
            StopReason::LimitTurns => println!("Hit turn budget"),
            StopReason::LimitTotalTokens => {
                println!("Hit token budget");
            }
            _ => {}
        }
        Ok(())
    }
    // --8<-- [end:limits]
}

mod cancellation {
    // --8<-- [start:cancel_timeout]
    use std::time::Duration;

    use strands::{
        Agent,
        CancelHandle,
        CancelOptions,
        InvokeOptions,
    };

    async fn run() -> strands::Result<()> {
        let mut agent = Agent::builder().build()?;
        let cancel = CancelHandle::new();
        let signal = cancel.signal();
        let deadline = tokio::spawn(async move {
            tokio::time::sleep(Duration::from_secs(30)).await;
            cancel.cancel(CancelOptions::default());
        });

        let result = agent
            .invoke_with(
                "Analyze this large dataset.",
                InvokeOptions {
                    cancel_signal: Some(signal),
                    ..InvokeOptions::default()
                },
            )
            .await?;
        deadline.abort();
        println!("{}", result.stop_reason());
        Ok(())
    }
    // --8<-- [end:cancel_timeout]
}

mod cancellation_tool {
    // --8<-- [start:cancel_in_tool]
    use strands::{ToolContext, ToolError, tool};

    #[tool(description = "Normalize a list of records.")]
    async fn normalize_records(
        #[tool(context)] context: &ToolContext,
        records: Vec<String>,
    ) -> Result<String, ToolError> {
        let mut normalized = Vec::new();
        for record in records {
            if context.cancel_signal().is_cancelled() {
                return Ok(format!(
                    "cancelled after {} records",
                    normalized.len(),
                ));
            }
            normalized.push(record.trim().to_uppercase());
        }
        Ok(normalized.join("\n"))
    }
    // --8<-- [end:cancel_in_tool]
}

mod service {
    // --8<-- [start:worked_example]
    use std::time::Duration;

    use strands::{
        Agent,
        CancelHandle,
        CancelOptions,
        InvokeLimits,
        InvokeOptions,
        StopReason,
    };

    enum Outcome {
        Completed,
        BudgetExceeded,
        Cancelled,
        Blocked,
        Stopped,
    }

    struct Reply {
        outcome: Outcome,
        text: Option<String>,
    }

    fn classify(reason: StopReason) -> Outcome {
        match reason {
            StopReason::EndTurn | StopReason::StopSequence => {
                Outcome::Completed
            }
            StopReason::Cancelled => Outcome::Cancelled,
            StopReason::LimitTurns
            | StopReason::LimitTotalTokens
            | StopReason::LimitOutputTokens
            | StopReason::LimitStructuredOutputAttempts => {
                Outcome::BudgetExceeded
            }
            StopReason::ContentFiltered
            | StopReason::GuardrailIntervened => Outcome::Blocked,
            _ => Outcome::Stopped,
        }
    }

    async fn handle_request(
        prompt: &str,
        deadline: Duration,
    ) -> strands::Result<Reply> {
        let mut agent = Agent::builder().printer(false).build()?;
        let cancel = CancelHandle::new();
        let signal = cancel.signal();
        let timer = tokio::spawn(async move {
            tokio::time::sleep(deadline).await;
            cancel.cancel(CancelOptions::default());
        });
        let result = agent
            .invoke_with(
                prompt,
                InvokeOptions {
                    limits: Some(InvokeLimits {
                        turns: Some(6),
                        total_tokens: Some(20_000),
                        ..InvokeLimits::default()
                    }),
                    cancel_signal: Some(signal),
                    ..InvokeOptions::default()
                },
            )
            .await?;
        timer.abort();

        let outcome = classify(result.stop_reason());
        let text = matches!(outcome, Outcome::Completed)
            .then(|| result.to_string());
        Ok(Reply { outcome, text })
    }
    // --8<-- [end:worked_example]
}
