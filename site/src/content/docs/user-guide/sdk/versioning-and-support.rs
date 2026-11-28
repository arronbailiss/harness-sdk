#![allow(dead_code)]

// --8<-- [start:deprecation_warning]
#[deprecated(
    since = "1.8.0",
    note = "Use new_function(); this function will be removed in 2.0.0."
)]
fn deprecated_function() {}
// --8<-- [end:deprecation_warning]
