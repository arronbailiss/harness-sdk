#![allow(dead_code)]

mod getting_started {
    // --8<-- [start:getting_started]
    use strands::vended_plugins::todos::Todos;
    use strands::Agent;

    fn agent() -> strands::Result<Agent> {
        Agent::builder()
            .plugins([Todos::default()])
            .printer(false)
            .build()
    }
    // --8<-- [end:getting_started]
}

mod inspect_state {
    // --8<-- [start:inspect_state]
    use strands::vended_plugins::todos::TodoItem;
    use strands::Agent;

    fn todos(agent: &Agent) -> strands::Result<Vec<TodoItem>> {
        Ok(agent
            .app_state()
            .get::<Vec<TodoItem>>("todos")?
            .unwrap_or_default())
    }
    // --8<-- [end:inspect_state]
}
