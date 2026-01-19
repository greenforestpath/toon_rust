pub mod args;
pub mod conversion;
pub mod json_stream;
pub mod json_stringify;

use crate::error::Result;

/// Runs the CLI entrypoint.
///
/// # Errors
///
/// Returns a placeholder error until CLI wiring is implemented.
#[allow(clippy::missing_const_for_fn)]
pub fn run() -> Result<()> {
    Err(crate::error::ToonError::message(
        "CLI wiring not implemented",
    ))
}
