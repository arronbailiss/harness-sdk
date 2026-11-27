mod using_presets {
    // --8<-- [start:using_presets]
    use strands::Agent;
    use strands::context_manager::{
        ContextManagerConfig, ContextManagerStrategy, StrategyPresetName,
    };

    fn agent() -> strands::Result<Agent> {
        let config = ContextManagerConfig {
            strategies: Some(vec![
                StrategyPresetName::ProactiveSummarization.into(),
                StrategyPresetName::LargeToolOffloading.into(),
            ]),
            ..ContextManagerConfig::default()
        };
        Agent::builder()
            .context_manager(ContextManagerStrategy::Config(config))
            .build()
    }
    // --8<-- [end:using_presets]
}

mod mixing {
    // --8<-- [start:mixing]
    use strands::Agent;
    use strands::context_manager::{
        ContextManagerConfig, ContextManagerStrategy, Offload, OffloadConditions,
        OffloadTarget, StrategyPresetName, SummarizeConfig,
    };

    fn agent() -> strands::Result<Agent> {
        let summarize = Offload::summarize(
            OffloadTarget::All,
            SummarizeConfig::default(),
        )?
        .when(OffloadConditions {
            context_utilization: Some("90%".into()),
            preserve_recent: Some(2.into()),
            ..OffloadConditions::default()
        })?;
        let config = ContextManagerConfig {
            strategies: Some(vec![
                StrategyPresetName::LargeToolOffloading.into(),
                summarize.into(),
            ]),
            ..ContextManagerConfig::default()
        };
        Agent::builder()
            .context_manager(ContextManagerStrategy::Config(config))
            .build()
    }
    // --8<-- [end:mixing]
}
