#![allow(dead_code)]

// --8<-- [start:deprecation_warning]
#[deprecated(
    since = "2.0.0",
    note = "Use new_function() instead."
)]
fn deprecated_function() {}
// --8<-- [end:deprecation_warning]
