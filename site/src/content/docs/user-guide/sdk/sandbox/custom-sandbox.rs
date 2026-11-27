#![allow(dead_code, deprecated)]

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

    use strands::sandbox::Sandbox;
    use strands::vended_tools::file_editor::{
        make_file_editor,
        MakeFileEditorOptions,
    };
    use strands::vended_tools::shell::{
        make_shell,
        MakeShellOptions,
    };
    use strands::Tool;

    fn tools(
        sandbox: Arc<dyn Sandbox>,
    ) -> strands::Result<Vec<Arc<dyn Tool>>> {
        let shell = make_shell(
            MakeShellOptions::builder()
                .sandbox(Arc::clone(&sandbox))
                .name("sandbox_shell")
                .build(),
        )?;
        let editor = make_file_editor(
            MakeFileEditorOptions::builder()
                .sandbox(sandbox)
                .name("sandbox_file_editor")
                .build(),
        );
        Ok(vec![Arc::new(shell), Arc::new(editor)])
    }
    // --8<-- [end:vend_tools]
}
