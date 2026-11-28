#![allow(dead_code)]

fn agents_as_tools_hub() -> strands::Result<strands::Agent> {
    // --8<-- [start:agents_as_tools_hub]
    let weather = strands::Agent::builder()
        .name("weather_agent")
        .description("Answers questions about the weather.")
        .system_prompt("Give practical weather advice.")
        .printer(false)
        .build()?;

    strands::Agent::builder()
        .system_prompt(
            "Route weather questions to weather_agent and answer the rest.",
        )
        .tools([weather])
        .build()
    // --8<-- [end:agents_as_tools_hub]
}
