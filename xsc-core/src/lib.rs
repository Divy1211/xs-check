pub mod parsing;
pub mod r#static;
pub mod utils;
pub mod doxygen;

// The synthetic path under which the bundled prelude is loaded. Both the CLI
// and the type-checker compare against this value to identify prelude code.
pub const PRELUDE_PATH: &str = "prelude.xs";