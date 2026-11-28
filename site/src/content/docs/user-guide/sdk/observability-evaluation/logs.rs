#![allow(dead_code)]

// --8<-- [start:basic_logging]
fn configure_logging() {
    tracing_subscriber::fmt()
        .with_max_level(
            tracing_subscriber::filter::LevelFilter::DEBUG,
        )
        .init();
}
// --8<-- [end:basic_logging]

// --8<-- [start:advanced_logging]
fn configure_detailed_logging(
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing_subscriber::fmt()
        .with_max_level(
            tracing_subscriber::filter::LevelFilter::TRACE,
        )
        .with_target(true)
        .with_thread_ids(true)
        .try_init()?;
    Ok(())
}
// --8<-- [end:advanced_logging]

fn main() {}
