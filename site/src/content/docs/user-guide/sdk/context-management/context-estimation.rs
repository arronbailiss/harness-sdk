mod context_window_limit {
    // --8<-- [start:context_window_limit]
    use strands::models::bedrock::BedrockModel;

    fn main() {
        let model = BedrockModel::builder()
            .model_id("my-custom-model")
            .context_window_limit(128_000)
            .build();
        let _ = model;
    }
    // --8<-- [end:context_window_limit]
}

mod utilization {
    // --8<-- [start:utilization]
    use strands::models::bedrock::BedrockModel;
    use strands::models::Model;

    fn main() {
        let model = BedrockModel::builder()
            .context_window_limit(128_000)
            .build();
        let projected_tokens = 64_000;
        let ratio = model.estimate_utilization(projected_tokens);
        let _ = ratio;
    }
    // --8<-- [end:utilization]
}
