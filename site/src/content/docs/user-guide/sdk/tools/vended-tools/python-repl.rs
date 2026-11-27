#![allow(dead_code)]

mod default_tool {
    // --8<-- [start:example]
    use strands::Agent;
    use strands::vended_tools::python_repl::python_repl;

    fn agent() -> strands::Result<Agent> {
        Agent::builder()
            .tools([python_repl()])
            .printer(false)
            .build()
    }
    // --8<-- [end:example]
}

mod custom {
    // --8<-- [start:custom]
    use strands::Agent;
    use strands::vended_tools::python_repl::{
        make_python_repl,
        MakePythonReplOptions,
    };

    fn agent() -> strands::Result<Agent> {
        let python = make_python_repl(
            MakePythonReplOptions::builder()
                .language("python3.12")
                .build(),
        )?;
        Agent::builder().tools([python]).printer(false).build()
    }
    // --8<-- [end:custom]
}
