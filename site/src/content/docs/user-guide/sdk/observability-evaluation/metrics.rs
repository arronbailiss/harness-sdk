#![allow(dead_code)]

async fn basic_metrics() -> strands::Result<()> {
    // --8<-- [start:basic_metrics]
    let mut agent = strands::Agent::builder().build()?;
    let result = agent.invoke("Explain observability.").await?;

    if let Some(metrics) = result.metrics() {
        let usage = &metrics.accumulated_usage;
        println!("Input tokens: {}", usage.input_tokens);
        println!("Output tokens: {}", usage.output_tokens);
        println!("Total tokens: {}", usage.total_tokens);
        println!("Cycles: {}", metrics.cycle_count);
        println!("Duration: {} ms", metrics.total_duration);
    }
    // --8<-- [end:basic_metrics]
    Ok(())
}

async fn agent_loop_metrics() -> strands::Result<()> {
    // --8<-- [start:agent_loop_metrics]
    let mut agent = strands::Agent::builder().build()?;
    let result = agent.invoke("Explain agent loops.").await?;

    if let Some(metrics) = result.metrics() {
        println!(
            "Average cycle: {:.1} ms",
            metrics.average_cycle_time()
        );
        if let Some(invocation) =
            metrics.latest_agent_invocation()
        {
            for cycle in &invocation.cycles {
                println!(
                    "{}: {} ms, {} tokens",
                    cycle.cycle_id,
                    cycle.duration,
                    cycle.usage.total_tokens
                );
            }
        }
    }
    // --8<-- [end:agent_loop_metrics]
    Ok(())
}

async fn tool_metrics() -> strands::Result<()> {
    // --8<-- [start:tool_metrics]
    let mut agent = strands::Agent::builder().build()?;
    let result = agent.invoke("Describe tool metrics.").await?;

    if let Some(metrics) = result.metrics() {
        for (name, usage) in metrics.tool_usage() {
            println!(
                "{name}: {} calls, {:.0}% successful, {:.1} ms average",
                usage.metrics.call_count,
                usage.success_rate * 100.0,
                usage.average_time
            );
        }
    }
    // --8<-- [end:tool_metrics]
    Ok(())
}

async fn metrics_summary(
) -> Result<(), Box<dyn std::error::Error>> {
    // --8<-- [start:metrics_summary]
    let mut agent = strands::Agent::builder().build()?;
    let result = agent.invoke("Summarize metrics.").await?;

    if let Some(metrics) = result.metrics() {
        println!("{}", serde_json::to_string_pretty(metrics)?);
    }
    // --8<-- [end:metrics_summary]
    Ok(())
}

async fn local_traces() -> strands::Result<()> {
    // --8<-- [start:local_traces]
    let mut agent = strands::Agent::builder().build()?;
    let result = agent.invoke("Explain local traces.").await?;

    if let Some(traces) = result.traces() {
        for trace in traces {
            println!("{}: {} ms", trace.name, trace.duration);
            for child in &trace.children {
                println!("  {}: {} ms", child.name, child.duration);
            }
        }
    }
    // --8<-- [end:local_traces]
    Ok(())
}

fn main() {}
