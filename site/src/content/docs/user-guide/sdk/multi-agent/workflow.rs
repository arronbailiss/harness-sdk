// --8<-- [start:workflow_sequential]
use strands::Agent;

async fn process_workflow(notes: &str) -> strands::Result<String> {
    let mut researcher = Agent::builder()
        .system_prompt("Extract the facts in the supplied incident notes.")
        .printer(false)
        .build()?;
    let mut analyst = Agent::builder()
        .system_prompt("Analyze the supplied facts. Distinguish causes from guesses.")
        .printer(false)
        .build()?;
    let mut writer = Agent::builder()
        .system_prompt("Write a concise incident report from the supplied analysis.")
        .printer(false)
        .build()?;

    let facts = researcher
        .invoke(format!("Extract facts from these notes:\n{notes}"))
        .await?;
    let analysis = analyst
        .invoke(format!("Analyze these incident facts:\n{facts}"))
        .await?;
    let report = writer
        .invoke(format!("Write a report from this analysis:\n{analysis}"))
        .await?;

    researcher.shutdown().await?;
    analyst.shutdown().await?;
    writer.shutdown().await?;
    Ok(report.to_string())
}

#[tokio::main]
async fn main() -> strands::Result<()> {
    let notes = "09:00: deployed version 2.3. 09:05: checkout errors increased. \
09:12: rolled back to version 2.2. 09:15: error rate returned to normal. \
Root cause has not been confirmed.";

    println!("{}", process_workflow(notes).await?);
    Ok(())
}
// --8<-- [end:workflow_sequential]
