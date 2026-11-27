mod create_model_class {
    // --8<-- [start:create_model_class]
    use std::sync::RwLock;

    use strands::models::{
        BaseModelConfig,
        BoxStream,
        Model,
        ModelRequest,
    };
    use strands::models::streaming::ModelStreamEvent;

    #[derive(Default)]
    struct CustomModel {
        config: RwLock<BaseModelConfig>,
    }

    #[strands::async_trait]
    impl Model for CustomModel {
        fn stream(
            &self,
            _request: ModelRequest,
        ) -> BoxStream<'static, strands::Result<ModelStreamEvent>> {
            Box::pin(futures::stream::empty())
        }

        fn get_config(&self) -> BaseModelConfig {
            self.config.read().unwrap().clone()
        }

        fn update_config(
            &self,
            config: BaseModelConfig,
        ) -> strands::Result<()> {
            self.config.write().unwrap().merge(config);
            Ok(())
        }
    }
    // --8<-- [end:create_model_class]
}

mod implement_stream {
    // --8<-- [start:implement_stream]
    use std::sync::RwLock;

    use strands::models::streaming::{
        ModelContentBlockDeltaEvent,
        ModelContentBlockStartEvent,
        ModelContentBlockStopEvent,
        ModelMessageStartEvent,
        ModelMessageStopEvent,
        ModelStreamEvent,
        TextDelta,
    };
    use strands::models::{
        BaseModelConfig,
        BoxStream,
        Model,
        ModelRequest,
    };
    use strands::{Message, Role, StopReason};

    struct CustomModel {
        config: RwLock<BaseModelConfig>,
    }

    #[strands::async_trait]
    impl Model for CustomModel {
        fn stream(
            &self,
            request: ModelRequest,
        ) -> BoxStream<'static, strands::Result<ModelStreamEvent>> {
            let text = request
                .messages
                .last()
                .map(Message::text)
                .unwrap_or_default();
            let events: Vec<ModelStreamEvent> = vec![
                ModelMessageStartEvent::new(Role::Assistant).into(),
                ModelContentBlockStartEvent::new().into(),
                ModelContentBlockDeltaEvent::new(
                    TextDelta::new(text),
                )
                .into(),
                ModelContentBlockStopEvent::new().into(),
                ModelMessageStopEvent::new(
                    StopReason::EndTurn,
                )
                .into(),
            ];
            Box::pin(futures::stream::iter(
                events.into_iter().map(Ok),
            ))
        }

        fn get_config(&self) -> BaseModelConfig {
            self.config.read().unwrap().clone()
        }

        fn update_config(
            &self,
            config: BaseModelConfig,
        ) -> strands::Result<()> {
            self.config.write().unwrap().merge(config);
            Ok(())
        }
    }
    // --8<-- [end:implement_stream]
}

mod stream_events {
    // --8<-- [start:stream_events]
    use strands::models::streaming::{
        ModelContentBlockDeltaEvent,
        ModelContentBlockStartEvent,
        ModelContentBlockStopEvent,
        ModelMessageStartEvent,
        ModelMessageStopEvent,
        ModelMetadataEvent,
        ModelStreamEvent,
        TextDelta,
        Usage,
    };
    use strands::{Role, StopReason};

    fn events() -> Vec<ModelStreamEvent> {
        vec![
            ModelMessageStartEvent::new(Role::Assistant).into(),
            ModelContentBlockStartEvent::new().into(),
            ModelContentBlockDeltaEvent::new(
                TextDelta::new("Hello"),
            )
            .into(),
            ModelContentBlockStopEvent::new().into(),
            ModelMessageStopEvent::new(
                StopReason::EndTurn,
            )
            .into(),
            ModelMetadataEvent::new()
                .with_usage(
                    Usage::new(234, 234, 468)
                        .with_cache_read_input_tokens(200),
                )
                .into(),
        ]
    }
    // --8<-- [end:stream_events]
}

mod usage_example {
    // --8<-- [start:usage_example]
    use std::sync::RwLock;

    use strands::models::streaming::{
        ModelMessageStartEvent,
        ModelMessageStopEvent,
        ModelStreamEvent,
    };
    use strands::models::{
        BaseModelConfig,
        BoxStream,
        Model,
        ModelRequest,
    };
    use strands::{Role, StopReason};

    struct CustomModel {
        config: RwLock<BaseModelConfig>,
    }

    #[strands::async_trait]
    impl Model for CustomModel {
        fn stream(
            &self,
            _request: ModelRequest,
        ) -> BoxStream<'static, strands::Result<ModelStreamEvent>> {
            let events: Vec<ModelStreamEvent> = vec![
                ModelMessageStartEvent::new(Role::Assistant).into(),
                ModelMessageStopEvent::new(
                    StopReason::EndTurn,
                )
                .into(),
            ];
            Box::pin(futures::stream::iter(
                events.into_iter().map(Ok),
            ))
        }

        fn get_config(&self) -> BaseModelConfig {
            self.config.read().unwrap().clone()
        }

        fn update_config(
            &self,
            config: BaseModelConfig,
        ) -> strands::Result<()> {
            self.config.write().unwrap().merge(config);
            Ok(())
        }
    }

    use strands::Agent;

    fn agent() -> strands::Result<Agent> {
        let model = CustomModel {
            config: RwLock::new(
                BaseModelConfig::builder()
                    .model_id("your-model-id")
                    .max_tokens(2000)
                    .temperature(0.7)
                    .build(),
            ),
        };
        Agent::builder()
            .model(model)
            .build()
    }
    // --8<-- [end:usage_example]
}

mod model_info {
    // --8<-- [start:model_info]
    use std::sync::RwLock;

    use strands::models::streaming::ModelStreamEvent;
    use strands::models::{
        BaseModelConfig,
        BoxStream,
        Model,
        ModelRequest,
    };

    struct CustomModel {
        config: RwLock<BaseModelConfig>,
    }

    #[strands::async_trait]
    impl Model for CustomModel {
        fn stream(
            &self,
            _request: ModelRequest,
        ) -> BoxStream<'static, strands::Result<ModelStreamEvent>> {
            Box::pin(futures::stream::empty())
        }

        fn get_config(&self) -> BaseModelConfig {
            self.config.read().unwrap().clone()
        }

        fn update_config(
            &self,
            config: BaseModelConfig,
        ) -> strands::Result<()> {
            self.config.write().unwrap().merge(config);
            Ok(())
        }

        fn model_info(&self) -> strands::models::ModelInfo {
            use strands::models::{
                ModelCapabilities,
                ModelInfo,
                base_model_info,
            };

            let config = self.get_config();
            let base = base_model_info(&config);
            let facts = ModelInfo {
                capabilities: ModelCapabilities {
                    server_side_summary: Some(false),
                    ..ModelCapabilities::default()
                },
                ..ModelInfo::default()
            };
            ModelInfo::merge([
                Some(&base),
                Some(&facts),
                config.model_info.as_ref(),
            ])
        }
    }
    // --8<-- [end:model_info]
}
