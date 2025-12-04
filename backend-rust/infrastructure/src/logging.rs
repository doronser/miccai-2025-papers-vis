//! Logging infrastructure using tracing
//!
//! This module provides structured logging setup using the `tracing` ecosystem.
//! It configures log levels, formatting, and filtering for both development
//! and production environments.
//!
//! # Environment Variables
//! - `RUST_LOG`: Controls log level and filtering (default: `info`)
//!   Examples:
//!   - `RUST_LOG=debug` - Enable debug logging for all modules
//!   - `RUST_LOG=infrastructure=debug,services=info` - Per-module filtering
//!   - `RUST_LOG=trace` - Maximum verbosity (includes all logs)
//!
//! # Usage
//! Call `init_logging()` once at application startup:
//!
//! ```ignore
//! use infrastructure::logging::init_logging;
//!
//! fn main() {
//!     init_logging();
//!     // Rest of application...
//! }
//! ```

use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

/// Initialize the tracing subscriber with sensible defaults
///
/// This function should be called once at application startup.
/// It configures the global tracing subscriber with:
/// - Log level filtering from `RUST_LOG` environment variable (default: `info`)
/// - Pretty formatting in development (with colors and timestamps)
/// - Structured JSON formatting in production (when `RUST_LOG=json` is set)
/// - Module-level filtering to reduce noise from dependencies
///
/// # Panics
/// This function will panic if called more than once, as the global subscriber
/// can only be set once.
///
/// # Example
/// ```
/// use infrastructure::logging::init_logging;
///
/// // Initialize logging at startup
/// init_logging();
///
/// // Now you can use tracing macros
/// tracing::info!("Application started");
/// tracing::debug!("Debug information");
/// ```
pub fn init_logging() {
    // Determine if we should use JSON formatting
    let use_json = std::env::var("RUST_LOG")
        .map(|v| v.contains("json"))
        .unwrap_or(false);

    // Create environment filter with sensible defaults
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        // Default to info level for our crates, warn for dependencies
        EnvFilter::new("info")
            .add_directive("infrastructure=info".parse().unwrap())
            .add_directive("services=info".parse().unwrap())
            .add_directive("api=info".parse().unwrap())
            .add_directive("models=info".parse().unwrap())
    });

    if use_json {
        // JSON formatting for production
        tracing_subscriber::registry()
            .with(env_filter)
            .with(fmt::layer().json())
            .init();
    } else {
        // Pretty formatting for development
        tracing_subscriber::registry()
            .with(env_filter)
            .with(
                fmt::layer()
                    .with_target(true)
                    .with_thread_ids(false)
                    .with_file(true)
                    .with_line_number(true),
            )
            .init();
    }
}

/// Initialize logging for testing
///
/// Similar to `init_logging()` but uses a try_init approach that won't panic
/// if the subscriber is already set. This is useful for tests where multiple
/// test cases might try to initialize logging.
pub fn init_test_logging() {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("debug"));

    let _ = tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt::layer().with_test_writer())
        .try_init();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_test_logging() {
        // Should not panic even if called multiple times
        init_test_logging();
        init_test_logging();

        // Test that logging works
        tracing::info!("Test log message");
        tracing::debug!("Debug message");
    }

    #[test]
    fn test_logging_with_structured_data() {
        init_test_logging();

        // Test structured logging
        tracing::info!(
            user_id = "test_123",
            action = "load_paper",
            "User action logged"
        );
    }
}
