#![allow(dead_code)]

fn code_configuration(
) -> strands::Result<(
    strands::telemetry::TracerGuard,
    strands::telemetry::MeterGuard,
)> {
    // --8<-- [start:code_configuration]
    use strands::telemetry::{
        MeterConfig, TelemetryExporters, TracerConfig, setup_meter,
        setup_tracer,
    };

    let exporters = TelemetryExporters {
        otlp: true,
        console: false,
    };
    let tracer = setup_tracer(TracerConfig {
        exporters,
        ..Default::default()
    })?;
    let meter = setup_meter(MeterConfig {
        exporters,
        ..Default::default()
    })?;
    tracer.try_init_subscriber()?;
    // --8<-- [end:code_configuration]
    Ok((tracer, meter))
}

fn console_exporter(
) -> strands::Result<strands::telemetry::TracerGuard> {
    // --8<-- [start:console_exporter]
    use strands::telemetry::{
        TelemetryExporters, TracerConfig, setup_tracer,
    };

    let tracer = setup_tracer(TracerConfig {
        exporters: TelemetryExporters {
            otlp: false,
            console: true,
        },
        ..Default::default()
    })?;
    tracer.try_init_subscriber()?;
    // --8<-- [end:console_exporter]
    Ok(tracer)
}

fn custom_attributes() -> strands::Result<strands::Agent> {
    // --8<-- [start:custom_attributes]
    let agent = strands::Agent::builder()
        .trace_attributes([
            ("session.id", "abc-1234"),
            ("deployment.environment", "production"),
        ])
        .build()?;
    // --8<-- [end:custom_attributes]
    Ok(agent)
}

fn baggage_propagation() -> strands::Result<strands::Agent> {
    // --8<-- [start:baggage_propagation]
    let agent = strands::Agent::builder()
        .baggage_attributes([
            ("tenant.id", "acme"),
            ("deployment.environment", "production"),
        ])
        .build()?;
    // --8<-- [end:baggage_propagation]
    Ok(agent)
}

fn baggage_processor(
) -> strands::Result<strands::telemetry::TracerGuard> {
    // --8<-- [start:baggage_processor]
    use strands::telemetry::{
        BaggageProcessor, TelemetryExporters, TracerConfig,
        setup_tracer,
    };

    let tracer = setup_tracer(TracerConfig {
        exporters: TelemetryExporters {
            otlp: true,
            console: false,
        },
        baggage_processor: Some(BaggageProcessor::predicate(|key| {
            matches!(key, "tenant.id" | "deployment.environment")
        })),
        ..Default::default()
    })?;
    // --8<-- [end:baggage_processor]
    Ok(tracer)
}

fn custom_spans() {
    // --8<-- [start:custom_spans]
    use strands::telemetry::opentelemetry::KeyValue;
    use strands::telemetry::opentelemetry::trace::{
        Span, Tracer,
    };

    let tracer = strands::telemetry::get_tracer();
    let mut span = tracer.start("my-custom-operation");
    span.set_attribute(KeyValue::new("custom.key", "value"));
    span.end();
    // --8<-- [end:custom_spans]
}

fn configuring_exporters(
) -> strands::Result<strands::telemetry::TracerGuard> {
    // --8<-- [start:configuring_exporters]
    use strands::telemetry::{
        TelemetryExporters, TracerConfig, setup_tracer,
    };

    let tracer = setup_tracer(TracerConfig {
        exporters: TelemetryExporters {
            otlp: true,
            console: true,
        },
        ..Default::default()
    })?;
    // --8<-- [end:configuring_exporters]
    Ok(tracer)
}

async fn end_to_end() -> strands::Result<()> {
    // --8<-- [start:end_to_end]
    use strands::telemetry::{
        MeterConfig, TelemetryExporters, TracerConfig, setup_meter,
        setup_tracer,
    };

    let exporters = TelemetryExporters {
        otlp: true,
        console: true,
    };
    let tracer = setup_tracer(TracerConfig {
        exporters,
        ..Default::default()
    })?;
    let _meter = setup_meter(MeterConfig {
        exporters,
        ..Default::default()
    })?;
    tracer.try_init_subscriber()?;

    let mut agent = strands::Agent::builder()
        .name("researcher")
        .trace_attributes([("application", "mars-research")])
        .printer(false)
        .build()?;
    let result = agent
        .invoke("What is the atmosphere of Mars like?")
        .await?;
    println!("{result}");
    agent.shutdown().await?;
    // --8<-- [end:end_to_end]
    Ok(())
}

fn main() {}
