#![allow(dead_code)]

fn worker() {
    // --8<-- [start:worker]
    use std::path::PathBuf;
    use strands_harness::{
        Isolation, MakeProgrammaticToolCallerOptions,
        make_programmatic_tool_caller,
    };

    let _caller = make_programmatic_tool_caller(
        MakeProgrammaticToolCallerOptions::builder()
            .isolation(Isolation::Subprocess(PathBuf::from(
                "/usr/local/bin/monty",
            )))
            .build(),
    );
    // --8<-- [end:worker]
}

fn main() {}
