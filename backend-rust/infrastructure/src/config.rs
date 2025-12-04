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

    #[test]
    fn test_config_creation() {
        let config = Config::from_env();
        assert!(!config.papers_dir.as_os_str().is_empty());
        assert!(!config.embeddings_dir.as_os_str().is_empty());
        assert!(!config.cache_dir.as_os_str().is_empty());
        assert!(!config.host.is_empty());
        assert!(config.port > 0);
    }
}
