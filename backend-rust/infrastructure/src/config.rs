//! Configuration management
//!
//! This module implements Design Decision #3: Data Directory Path Resolution Strategy.
//! It uses environment variables with fallback defaults resolved at application startup.
//!
//! # Environment Variables
//! - `PAPERS_DIR` - Path to papers directory (default: `./data/papers_by_id/`)
//! - `EMBEDDINGS_DIR` - Path to embeddings directory (default: `./data/embeddings_by_id/`)
//! - `CACHE_DIR` - Path to cache directory (default: `./data/cache/`)
//! - `HOST` - Server bind address (default: `0.0.0.0`)
//! - `PORT` - Server port (default: `8000`)
//! - `CORS_ORIGINS` - Comma-separated list of allowed CORS origins

use std::path::PathBuf;

/// Application configuration
#[derive(Debug, Clone)]
pub struct Config {
    /// Path to papers directory
    pub papers_dir: PathBuf,
    
    /// Path to embeddings directory
    pub embeddings_dir: PathBuf,
    
    /// Path to cache directory
    pub cache_dir: PathBuf,
    
    /// Server bind address
    pub host: String,
    
    /// Server port
    pub port: u16,
    
    /// Allowed CORS origins
    pub cors_origins: Vec<String>,
}

impl Config {
    /// Load configuration from environment variables with fallback defaults
    pub fn from_env() -> Self {
        Self {
            papers_dir: Self::get_env_path(
                "PAPERS_DIR",
                "./data/papers_by_id",
            ),
            embeddings_dir: Self::get_env_path(
                "EMBEDDINGS_DIR",
                "./data/embeddings_by_id",
            ),
            cache_dir: Self::get_env_path(
                "CACHE_DIR",
                "./data/cache",
            ),
            host: std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: std::env::var("PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(8000),
            cors_origins: Self::parse_cors_origins(),
        }
    }
    
    /// Get a path from environment variable or use default
    fn get_env_path(env_var: &str, default: &str) -> PathBuf {
        std::env::var(env_var)
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from(default))
    }
    
    /// Parse CORS origins from environment variable
    fn parse_cors_origins() -> Vec<String> {
        std::env::var("CORS_ORIGINS")
            .unwrap_or_else(|_| {
                "http://localhost:3000,http://localhost:3001,http://localhost:5173".to_string()
            })
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::from_env()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_config_uses_defaults_when_env_vars_unset() {
        // Clear relevant env vars if they exist
        env::remove_var("PAPERS_DIR");
        env::remove_var("EMBEDDINGS_DIR");
        env::remove_var("CACHE_DIR");
        env::remove_var("HOST");
        env::remove_var("PORT");
        env::remove_var("CORS_ORIGINS");

        let config = Config::from_env();

        // Verify defaults are used
        assert_eq!(config.papers_dir, PathBuf::from("./data/papers_by_id"));
        assert_eq!(config.embeddings_dir, PathBuf::from("./data/embeddings_by_id"));
        assert_eq!(config.cache_dir, PathBuf::from("./data/cache"));
        assert_eq!(config.host, "0.0.0.0");
        assert_eq!(config.port, 8000);
        assert_eq!(config.cors_origins, vec![
            "http://localhost:3000",
            "http://localhost:3001",
            "http://localhost:5173"
        ]);
    }

    #[test]
    #[ignore] // Ignore by default due to env var race conditions in parallel tests
    fn test_config_uses_env_vars_when_set() {
        // Set environment variables
        env::set_var("PAPERS_DIR", "/custom/papers");
        env::set_var("EMBEDDINGS_DIR", "/custom/embeddings");
        env::set_var("CACHE_DIR", "/custom/cache");
        env::set_var("HOST", "127.0.0.1");
        env::set_var("PORT", "9000");
        env::set_var("CORS_ORIGINS", "http://example.com,http://test.com");

        let config = Config::from_env();

        // Verify env vars are used
        assert_eq!(config.papers_dir, PathBuf::from("/custom/papers"));
        assert_eq!(config.embeddings_dir, PathBuf::from("/custom/embeddings"));
        assert_eq!(config.cache_dir, PathBuf::from("/custom/cache"));
        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, 9000);
        assert_eq!(config.cors_origins, vec!["http://example.com", "http://test.com"]);

        // Clean up
        env::remove_var("PAPERS_DIR");
        env::remove_var("EMBEDDINGS_DIR");
        env::remove_var("CACHE_DIR");
        env::remove_var("HOST");
        env::remove_var("PORT");
        env::remove_var("CORS_ORIGINS");
    }

    #[test]
    #[ignore] // Ignore by default due to env var race conditions in parallel tests
    fn test_cors_origins_parsing_handles_whitespace() {
        env::set_var("CORS_ORIGINS", " http://example.com , http://test.com ");

        let config = Config::from_env();

        assert_eq!(config.cors_origins, vec!["http://example.com", "http://test.com"]);

        env::remove_var("CORS_ORIGINS");
    }

    #[test]
    #[ignore] // Ignore by default due to env var race conditions in parallel tests
    fn test_cors_origins_filters_empty_strings() {
        env::set_var("CORS_ORIGINS", "http://example.com,,http://test.com");

        let config = Config::from_env();

        assert_eq!(config.cors_origins, vec!["http://example.com", "http://test.com"]);

        env::remove_var("CORS_ORIGINS");
    }

    #[test]
    #[ignore] // Ignore by default due to env var race conditions in parallel tests
    fn test_invalid_port_uses_default() {
        env::set_var("PORT", "invalid");

        let config = Config::from_env();

        assert_eq!(config.port, 8000);

        env::remove_var("PORT");
    }

    #[test]
    fn test_default_trait_implementation() {
        let config = Config::default();

        assert!(!config.papers_dir.as_os_str().is_empty());
        assert!(!config.embeddings_dir.as_os_str().is_empty());
        assert!(!config.cache_dir.as_os_str().is_empty());
        assert!(!config.host.is_empty());
        assert!(config.port > 0);
    }
}
