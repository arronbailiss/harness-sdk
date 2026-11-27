#![allow(dead_code)]

mod configured {
    // --8<-- [start:default_strategy]
    use strands::{
        Agent,
        DefaultModelRetryStrategy,
        DefaultModelRetryStrategyOptions,
        RetryStrategy,
    };

    fn agent() -> strands::Result<Agent> {
        let strategy = DefaultModelRetryStrategy::new(
            DefaultModelRetryStrategyOptions::builder()
                .max_attempts(3)
                .build(),
        )?;
        Agent::builder()
            .retry_strategy(vec![RetryStrategy::model(strategy)])
            .build()
    }
    // --8<-- [end:default_strategy]
}

mod backoff {
    // --8<-- [start:custom_backoff]
    use std::sync::Arc;
    use std::time::Duration;

    use strands::{
        Agent,
        DefaultModelRetryStrategy,
        DefaultModelRetryStrategyOptions,
        ExponentialBackoff,
        ExponentialBackoffOptions,
        JitterKind,
        RetryStrategy,
    };

    fn agent() -> strands::Result<Agent> {
        let backoff = ExponentialBackoff::new(
            ExponentialBackoffOptions::builder()
                .base(Duration::from_secs(2))
                .max(Duration::from_secs(60))
                .multiplier(2.0)
                .jitter(JitterKind::Full)
                .build(),
        );
        let strategy = DefaultModelRetryStrategy::new(
            DefaultModelRetryStrategyOptions::builder()
                .max_attempts(4)
                .backoff(Arc::new(backoff))
                .build(),
        )?;
        Agent::builder()
            .retry_strategy(vec![RetryStrategy::model(strategy)])
            .build()
    }
    // --8<-- [end:custom_backoff]
}

mod disabled {
    // --8<-- [start:disable]
    fn agent() -> strands::Result<strands::Agent> {
        strands::Agent::builder()
            .retry_strategy(Vec::new())
            .build()
    }
    // --8<-- [end:disable]
}

mod retryable {
    // --8<-- [start:custom_retryable]
    use strands::{
        Agent,
        DefaultModelRetryStrategy,
        DefaultModelRetryStrategyOptions,
        Error,
        IsRetryable,
        RetryStrategy,
    };

    fn agent() -> strands::Result<Agent> {
        let classifier = IsRetryable::new(|error: &Error| {
            DefaultModelRetryStrategy::is_default_retryable(error)
                || error.to_string().contains("transient service error")
        });
        let strategy = DefaultModelRetryStrategy::new(
            DefaultModelRetryStrategyOptions::builder()
                .max_attempts(5)
                .is_retryable(classifier)
                .build(),
        )?;
        Agent::builder()
            .retry_strategy(vec![RetryStrategy::model(strategy)])
            .build()
    }
    // --8<-- [end:custom_retryable]
}

mod hook {
    // --8<-- [start:hook_retry]
    use std::time::Duration;

    use strands::Agent;
    use strands::hooks::AfterModelCallEvent;

    fn agent() -> strands::Result<Agent> {
        let agent = Agent::builder().retry_strategy(Vec::new()).build()?;
        agent.add_async_hook::<AfterModelCallEvent<'_>>(|event| {
            Box::pin(async move {
                if event.error().is_some() && event.attempt_count() <= 3 {
                    tokio::time::sleep(Duration::from_secs(2)).await;
                    event.set_retry(true);
                }
                Ok::<(), strands::Error>(())
            })
        });
        Ok(agent)
    }
    // --8<-- [end:hook_retry]
}
