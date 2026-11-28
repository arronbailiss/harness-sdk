#![allow(dead_code)]

mod default_tool {
    // --8<-- [start:example]
    use strands::Agent;
    use strands::vended_tools::notebook::notebook;

    fn agent() -> strands::Result<Agent> {
        Agent::builder()
            .tools([notebook()])
            .printer(false)
            .build()
    }
    // --8<-- [end:example]
}

mod custom {
    // --8<-- [start:custom]
    use strands::Agent;
    use strands::vended_tools::notebook::{
        make_notebook,
        MakeNotebookOptions,
    };

    fn agent() -> strands::Result<Agent> {
        let notes = make_notebook(
            MakeNotebookOptions::builder()
                .name("notes")
                .max_notebook_size_bytes(65_536)
                .build(),
        )?;
        Agent::builder().tools([notes]).printer(false).build()
    }
    // --8<-- [end:custom]
}

mod state {
    // --8<-- [start:state]
    use strands::{Agent, JsonObject};

    fn notebooks(agent: &Agent) -> strands::Result<Option<JsonObject>> {
        agent.app_state().get("notebooks")
    }
    // --8<-- [end:state]
}
