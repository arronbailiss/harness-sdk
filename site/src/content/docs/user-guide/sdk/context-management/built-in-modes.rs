mod automatic {
    // --8<-- [start:automatic]
    use strands::Agent;
    use strands::context_manager::ContextManagerStrategy;

    fn agent() -> strands::Result<Agent> {
        Agent::builder()
            .context_manager(ContextManagerStrategy::Auto)
            .build()
    }
    // --8<-- [end:automatic]
}

mod agentic {
    // --8<-- [start:agentic]
    use strands::Agent;
    use strands::context_manager::ContextManagerStrategy;

    fn agent() -> strands::Result<Agent> {
        Agent::builder()
            .context_manager(ContextManagerStrategy::Agentic)
            .build()
    }
    // --8<-- [end:agentic]
}

mod disabled {
    // --8<-- [start:disabled]
    use strands::Agent;
    use strands::context_manager::ContextManagerStrategy;

    fn agent() -> strands::Result<Agent> {
        Agent::builder()
            .context_manager(ContextManagerStrategy::Disabled)
            .build()
    }
    // --8<-- [end:disabled]
}
