mod register_event {
    // --8<-- [start:register_event]
    use strands::hooks::BeforeToolCallEvent;
    use strands::Agent;

    fn register_delete_guard(agent: Agent) -> Agent {
        agent.add_hook(|event: &mut BeforeToolCallEvent| {
            if event.tool_use().name == "delete_file" {
                event.set_cancel("Deleting files is disabled");
            }
        });
        agent
    }
    // --8<-- [end:register_event]
}
