#![allow(dead_code)]

// --8<-- [start:posix_sandbox]
use futures::stream;
use strands::sandbox::{
    ExecuteOptions,
    ExecutionResult,
    ExecutionStream,
    PosixShellSandbox,
};

struct RemoteShell;

impl PosixShellSandbox for RemoteShell {
    fn execute_streaming(
        &self,
        command: &str,
        _options: ExecuteOptions,
    ) -> ExecutionStream {
        let result = ExecutionResult::new(
            0,
            format!("remote output for {command}"),
            "",
        );
        Box::pin(stream::iter([Ok(result.into())]))
    }
}
// --8<-- [end:posix_sandbox]

mod vend_tools {
    // --8<-- [start:vend_tools]
    use std::sync::Arc;

    use strands::vended_tools::edit::edit;
    use strands::vended_tools::read::read;
    use strands::vended_tools::shell::{
        make_shell,
        MakeShellOptions,
    };
    use strands::vended_tools::write::write;
    use strands::Tool;

    fn get_tools() -> Vec<Arc<dyn Tool>> {
        let shell = make_shell(MakeShellOptions::default())
            .expect("default shell options are valid");
        vec![
            Arc::new(shell),
            Arc::new(read()),
            Arc::new(write()),
            Arc::new(edit()),
        ]
    }
    // --8<-- [end:vend_tools]
}
