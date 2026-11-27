mod basic_config {
    // --8<-- [start:basic_config]
    use strands::context_manager::{
        ContextManagerConfig,
        ContextManagerStrategy,
        Offload,
        OffloadConditions,
        OffloadTarget,
        SummarizeConfig,
        TruncateConfig,
    };
    use strands::Agent;

    fn main() -> strands::Result<()> {
        let truncate = Offload::truncate(
            OffloadTarget::ToolResults,
            TruncateConfig::default(),
        )?
        .when(OffloadConditions {
            block_threshold: Some(2000.into()),
            ..OffloadConditions::default()
        })?;
        let summarize = Offload::summarize(
            OffloadTarget::All,
            SummarizeConfig::default(),
        )?
        .when(OffloadConditions {
            context_utilization: Some("80%".into()),
            preserve_recent: Some(4.into()),
            ..OffloadConditions::default()
        })?;
        let config = ContextManagerConfig {
            strategies: Some(vec![truncate.into(), summarize.into()]),
            ..ContextManagerConfig::default()
        };
        let agent = Agent::builder()
            .context_manager(ContextManagerStrategy::Config(config))
            .build()?;
        let _ = agent;
        Ok(())
    }
    // --8<-- [end:basic_config]
}

mod truncate {
    // --8<-- [start:truncate]
    use strands::context_manager::{
        Offload,
        OffloadConditions,
        OffloadTarget,
        TruncateConfig,
    };

    fn main() -> strands::Result<()> {
        let strategy = Offload::truncate(
            OffloadTarget::ToolResults,
            TruncateConfig {
                preview_tokens: Some(750.0),
                ..TruncateConfig::default()
            },
        )?
        .when(OffloadConditions {
            block_threshold: Some(1500.into()),
            ..OffloadConditions::default()
        })?;
        let _ = strategy;
        Ok(())
    }
    // --8<-- [end:truncate]
}

mod summarize {
    // --8<-- [start:summarize]
    use strands::context_manager::{
        Offload,
        OffloadConditions,
        OffloadTarget,
        SummarizeConfig,
    };

    fn main() -> strands::Result<()> {
        let strategy = Offload::summarize(
            OffloadTarget::All,
            SummarizeConfig::default(),
        )?
        .when(OffloadConditions {
            context_utilization: Some("85%".into()),
            preserve_recent: Some(4.into()),
            ..OffloadConditions::default()
        })?;
        let _ = strategy;
        Ok(())
    }
    // --8<-- [end:summarize]
}

mod drop_strategy {
    // --8<-- [start:drop]
    use strands::context_manager::{
        Offload,
        OffloadConditions,
        OffloadTarget,
    };

    fn main() -> strands::Result<()> {
        let strategy = Offload::drop(OffloadTarget::ToolResults)?
            .when(OffloadConditions {
                preserve_recent: Some(5.into()),
                ..OffloadConditions::default()
            })?;
        let _ = strategy;
        Ok(())
    }
    // --8<-- [end:drop]
}

mod conditions_threshold {
    // --8<-- [start:conditions_threshold]
    use strands::context_manager::{
        Offload,
        OffloadConditions,
        OffloadTarget,
        TruncateConfig,
    };

    fn main() -> strands::Result<()> {
        let strategy = Offload::truncate(
            OffloadTarget::ToolResults,
            TruncateConfig::default(),
        )?
        .when(OffloadConditions {
            block_threshold: Some(2000.into()),
            ..OffloadConditions::default()
        })?;
        let _ = strategy;
        Ok(())
    }
    // --8<-- [end:conditions_threshold]
}

mod conditions_utilization {
    // --8<-- [start:conditions_utilization]
    use strands::context_manager::{
        Offload,
        OffloadConditions,
        OffloadTarget,
        SummarizeConfig,
    };

    fn main() -> strands::Result<()> {
        let strategy = Offload::summarize(
            OffloadTarget::All,
            SummarizeConfig::default(),
        )?
        .when(OffloadConditions {
            context_utilization: Some("85%".into()),
            preserve_recent: Some(4.into()),
            ..OffloadConditions::default()
        })?;
        let _ = strategy;
        Ok(())
    }
    // --8<-- [end:conditions_utilization]
}

mod conditions_both {
    // --8<-- [start:conditions_both]
    use strands::context_manager::{
        Offload,
        OffloadConditions,
        OffloadTarget,
        TruncateConfig,
    };

    fn main() -> strands::Result<()> {
        let strategy = Offload::truncate(
            OffloadTarget::ToolResults,
            TruncateConfig::default(),
        )?
        .when(OffloadConditions {
            block_threshold: Some(1500.into()),
            context_utilization: Some("90%".into()),
            ..OffloadConditions::default()
        })?;
        let _ = strategy;
        Ok(())
    }
    // --8<-- [end:conditions_both]
}

mod archive_disabled {
    // --8<-- [start:archive_disabled]
    use strands::context_manager::{
        ContextManagerConfig,
        ContextManagerStrategy,
    };
    use strands::Agent;

    fn main() -> strands::Result<()> {
        let config = ContextManagerConfig {
            archive: Some(false.into()),
            ..ContextManagerConfig::default()
        };
        let agent = Agent::builder()
            .context_manager(ContextManagerStrategy::Config(config))
            .build()?;
        let _ = agent;
        Ok(())
    }
    // --8<-- [end:archive_disabled]
}
