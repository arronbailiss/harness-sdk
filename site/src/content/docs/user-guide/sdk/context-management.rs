mod basic {
    // --8<-- [start:basic]
    use strands::Agent;
    use strands::context_manager::ContextManagerStrategy;

    fn agent() -> strands::Result<Agent> {
        Agent::builder()
            .context_manager(ContextManagerStrategy::Auto)
            .build()
    }
    // --8<-- [end:basic]
}

mod storage_backends {
    // --8<-- [start:storage_backends]
    use strands::Agent;
    use strands::context_manager::ContextManagerStrategy;
    use strands::storage::LocalFileStorage;

    fn agent() -> strands::Result<Agent> {
        Agent::builder()
            .storage(LocalFileStorage::new("./artifacts"))
            .context_manager(ContextManagerStrategy::Auto)
            .build()
    }
    // --8<-- [end:storage_backends]
}
