#![allow(dead_code)]

fn direct_passing() -> strands::Result<strands::Agent> {
    // --8<-- [start:direct_passing]
    let specialist = |name: &str, description: &str, prompt: &str| {
        strands::Agent::builder()
            .name(name)
            .description(description)
            .system_prompt(prompt)
            .printer(false)
            .build()
    };
    let researcher = specialist(
        "research_assistant",
        "Researches factual questions.",
        "Find concise, well-sourced information.",
    )?;
    let product = specialist(
        "product_recommendation_assistant",
        "Recommends products.",
        "Match products to the user's requirements.",
    )?;
    let travel = specialist(
        "trip_planning_assistant",
        "Plans trips.",
        "Build practical travel itineraries.",
    )?;

    strands::Agent::builder()
        .system_prompt("Route each request to the right specialist.")
        .tools([researcher, product, travel])
        .build()
    // --8<-- [end:direct_passing]
}

fn as_tool_customization() -> strands::Result<strands::Agent> {
    // --8<-- [start:as_tool_customization]
    let researcher = strands::Agent::builder()
        .name("researcher")
        .description("Researches factual questions.")
        .system_prompt("Return concise, sourced findings.")
        .printer(false)
        .build()?;
    let research_tool = researcher.as_tool(
        strands::agent::AgentAsToolOptions::builder()
            .name("research_assistant")
            .description("Answer research questions with factual findings.")
            .build(),
    );

    strands::Agent::builder()
        .system_prompt("Delegate research questions.")
        .tools([research_tool])
        .build()
    // --8<-- [end:as_tool_customization]
}

fn as_tool_context() -> strands::Result<strands::Agent> {
    // --8<-- [start:as_tool_context]
    let researcher = strands::Agent::builder()
        .name("researcher")
        .description("Researches factual questions.")
        .system_prompt("Build on prior research when useful.")
        .printer(false)
        .build()?;
    let research_tool = researcher.as_tool(
        strands::agent::AgentAsToolOptions::builder()
            .preserve_context(true)
            .build(),
    );

    strands::Agent::builder().tools([research_tool]).build()
    // --8<-- [end:as_tool_context]
}

// --8<-- [start:research_assistant]
#[strands::tool(description = "Research a factual question.")]
async fn research_assistant(
    query: String,
) -> Result<String, strands::ToolError> {
    let mut agent = strands::Agent::builder()
        .system_prompt("Return concise, well-sourced findings.")
        .printer(false)
        .build()
        .map_err(strands::ToolError::new)?;
    let response = agent
        .invoke(query)
        .await
        .map_err(strands::ToolError::new)?;
    Ok(response.to_string())
}
// --8<-- [end:research_assistant]

// --8<-- [start:multiple_specialists]
#[strands::tool(description = "Recommend products for a set of requirements.")]
async fn product_recommendation_assistant(
    query: String,
) -> Result<String, strands::ToolError> {
    Ok(format!("Product requirements: {query}"))
}

#[strands::tool(description = "Create a practical travel itinerary.")]
async fn trip_planning_assistant(
    query: String,
) -> Result<String, strands::ToolError> {
    Ok(format!("Travel requirements: {query}"))
}
// --8<-- [end:multiple_specialists]

fn orchestrator() -> strands::Result<strands::Agent> {
    // --8<-- [start:orchestrator]
    strands::Agent::builder()
        .system_prompt(
            "Route research, product, and travel requests to specialists.",
        )
        .tools(strands::tool_list![
            ResearchAssistant,
            ProductRecommendationAssistant,
            TripPlanningAssistant,
        ])
        .printer(false)
        .build()
    // --8<-- [end:orchestrator]
}

async fn usage() -> strands::Result<()> {
    // --8<-- [start:usage]
    let mut agent = orchestrator()?;
    let response = agent
        .invoke(
            "Recommend hiking boots for a Patagonia trip next month.",
        )
        .await?;
    println!("{response}");
    // --8<-- [end:usage]
    Ok(())
}

async fn delegation_basic() -> strands::Result<()> {
    // --8<-- [start:delegation_basic]
    let billing = strands::Agent::builder()
        .name("billing_expert")
        .description("Answers questions about charges and refunds.")
        .system_prompt("Handle billing questions precisely.")
        .printer(false)
        .build()?;
    let billing_tool = billing.as_tool(
        strands::agent::AgentAsToolOptions::builder()
            .delegate(true)
            .build(),
    );
    let mut orchestrator = strands::Agent::builder()
        .system_prompt("Route billing questions to billing_expert.")
        .tools([billing_tool])
        .printer(false)
        .build()?;
    let result = orchestrator.invoke("Why was I charged twice?").await?;
    println!("{result}");
    // --8<-- [end:delegation_basic]
    Ok(())
}
