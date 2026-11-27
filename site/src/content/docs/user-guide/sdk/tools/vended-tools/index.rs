#![allow(dead_code)]

mod quickstart {
    // --8<-- [start:quickstart]
    use strands::vended_tools::edit::edit;
    use strands::vended_tools::http_request::http_request;
    use strands::vended_tools::notebook::notebook;
    use strands::vended_tools::shell::{
        make_shell,
        MakeShellOptions,
    };
    use strands::Agent;

    fn agent() -> strands::Result<Agent> {
        let shell = make_shell(MakeShellOptions::default())?;
        Agent::builder()
            .tools([shell, edit(), http_request(), notebook()])
            .printer(false)
            .build()
    }
    // --8<-- [end:quickstart]
}

mod multi_agent {
    // --8<-- [start:multi_agent]
    use strands::multiagent::{Axis, Choice, Fixed, Inherit, Open, Preset};
    use strands::vended_tools::subagent::{
        make_subagent,
        MakeSubagentOptions,
    };
    use strands::Agent;

    fn agent() -> strands::Result<Agent> {
        let researcher = Preset::builder()
            .instructions("Research topics and cite sources.")
            .description("web research")
            .build();
        let delegate = make_subagent(
            MakeSubagentOptions::builder()
                .presets([("researcher", researcher)])
                .instructions(Axis::Open(Open::new()))
                .tools(Axis::Fixed(Fixed::new(vec!["http_request"])))
                .model(Axis::Inherit(Inherit::new()))
                .context(Axis::Choice(Choice::new(["none", "no_tools"])))
                .build(),
        )?;
        Agent::builder()
            .tools([delegate])
            .printer(false)
            .build()
    }
    // --8<-- [end:multi_agent]
}

mod custom_builder {
    // --8<-- [start:custom_builder]
    use strands::multiagent::{AgentSpec, ChildAgentBuilder};
    use strands::vended_tools::http_request::http_request;
    use strands::vended_tools::subagent::{
        make_subagent,
        MakeSubagentOptions,
    };
    use strands::Agent;

    fn agent() -> strands::Result<Agent> {
        let builder = ChildAgentBuilder::sync(|spec: &AgentSpec| {
            Agent::builder()
                .system_prompt(
                    spec.instructions
                        .as_deref()
                        .unwrap_or("Complete the delegated task."),
                )
                .tools([http_request()])
                .printer(false)
                .build()
        });
        let delegate = make_subagent(
            MakeSubagentOptions::builder().builder(builder).build(),
        )?;
        Agent::builder()
            .tools([delegate])
            .printer(false)
            .build()
    }
    // --8<-- [end:custom_builder]
}
