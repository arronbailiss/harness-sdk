#![allow(dead_code)]

mod default_tool {
    // --8<-- [start:example]
    use strands::Agent;
    use strands::vended_tools::web_fetch::web_fetch;

    fn agent() -> strands::Result<Agent> {
        Agent::builder()
            .tools([web_fetch()])
            .printer(false)
            .build()
    }
    // --8<-- [end:example]
}

mod markdown {
    // --8<-- [start:markdown]
    use strands::Agent;
    use strands::vended_tools::web_fetch::{
        make_web_fetch,
        MakeWebFetchOptions,
        WebFetchMode,
    };

    fn agent() -> strands::Result<Agent> {
        let fetch = make_web_fetch(
            MakeWebFetchOptions::builder()
                .mode(WebFetchMode::Markdown)
                .max_bytes(1_048_576)
                .max_content_chars(25_000)
                .build(),
        )?;
        Agent::builder().tools([fetch]).printer(false).build()
    }
    // --8<-- [end:markdown]
}
