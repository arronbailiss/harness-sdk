#![allow(dead_code)]

mod default_tool {
    // --8<-- [start:example]
    use strands::Agent;
    use strands::vended_tools::subagent::subagent;

    fn agent() -> strands::Result<Agent> {
        Agent::builder()
            .system_prompt("You are a manager.")
            .tools([subagent()])
            .printer(false)
            .build()
    }
    // --8<-- [end:example]
}

mod custom {
    // --8<-- [start:custom]
    use strands::multiagent::{Axis, Choice, ChoiceOption, Preset};
    use strands::vended_tools::subagent::{
        make_subagent,
        MakeSubagentOptions,
    };
    use strands::Agent;

    fn agent() -> strands::Result<Agent> {
        let researcher = Preset::builder()
            .instructions("You research topics thoroughly.")
            .description("deep research on a topic")
            .build();
        let reviewer = Preset::builder()
            .instructions("You review code for correctness and style.")
            .description("code review")
            .build();
        let models = Choice::new([
            ChoiceOption::new("fast")
                .with_value("global.anthropic.claude-sonnet-4-6")
                .with_description("quick tasks"),
            ChoiceOption::new("deep")
                .with_value("global.anthropic.claude-opus-4-6-v1")
                .with_description("hard problems"),
        ]);
        let delegate = make_subagent(
            MakeSubagentOptions::builder()
                .presets([
                    ("researcher", researcher),
                    ("reviewer", reviewer),
                ])
                .model(Axis::Choice(models))
                .context(Axis::Choice(Choice::new([
                    "none",
                    "all",
                    "no_tools",
                ])))
                .tools(Axis::Choice(
                    Choice::new(["read", "shell", "write"]).multiple(true),
                ))
                .max_depth(3)
                .build(),
        )?;
        Agent::builder()
            .tools([delegate])
            .printer(false)
            .build()
    }
    // --8<-- [end:custom]
}
