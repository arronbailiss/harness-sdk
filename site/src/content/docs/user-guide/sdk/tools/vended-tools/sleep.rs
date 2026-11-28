#![allow(dead_code)]

mod default_tool {
    // --8<-- [start:example]
    use strands::Agent;
    use strands::vended_tools::sleep::sleep;

    fn agent() -> strands::Result<Agent> {
        Agent::builder().tools([sleep()]).printer(false).build()
    }
    // --8<-- [end:example]
}

mod custom {
    // --8<-- [start:custom]
    use strands::Agent;
    use strands::vended_tools::sleep::{
        make_sleep,
        MakeSleepOptions,
    };

    fn agent() -> strands::Result<Agent> {
        let short_sleep = make_sleep(
            MakeSleepOptions::builder().max_duration(5.0).build(),
        )?;
        Agent::builder()
            .tools([short_sleep])
            .printer(false)
            .build()
    }
    // --8<-- [end:custom]
}
