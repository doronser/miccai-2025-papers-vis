use crate::{InfrastructureError, Result};
use std::path::PathBuf;

/// Application configuration loaded from environment variables
#[derive(Debug, Clone)]
pub struct Config {
    /// Base data directory (default: "src/data")
    pub data_dir: PathBuf,

    /// Papers directory (default: data_dir/papers_by_id)
    pub papers_dir: PathBuf,

    /// Embeddings directory (default: data_dir/embeddings_by_id)
    pub embeddings_dir: PathBuf,

    /// Cache directory (default: data_dir/cache)
    pub cache_dir: PathBuf,

    /// Server port (default: 8000)
    pub server_port: u16,

    /// CORS allowed origins (default: localhost development ports)
    pub cors_origins: Vec<String>,
}

impl Config {
    /// Load configuration from environment variables with sensible defaults
    pub fn from_env() -> Result<Self> {
        // Read DATA_DIR or use default
        let data_dir = std::env::var("DATA_DIR")
            .unwrap_or_else(|_| "src/data".to_string())
            .into();

        let papers_dir = Self::resolve_subdir(&data_dir, "papers_by_id");
        let embeddings_dir = Self::resolve_subdir(&data_dir, "embeddings_by_id");
        let cache_dir = Self::resolve_subdir(&data_dir, "cache");

        // Read SERVER_PORT or use default
        let server_port = std::env::var("SERVER_PORT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(8000);

        // Validate port range
        if server_port == 0 {
            return Err(InfrastructureError::ConfigError(
                "SERVER_PORT must be greater than 0".to_string(),
            ));
        }

        // Read CORS_ORIGINS or use default
        let cors_origins_str = std::env::var("CORS_ORIGINS").unwrap_or_else(|_| {
            "http://localhost:3000,http://localhost:3001,http://localhost:5173".to_string()
        });

        // Split by comma and trim whitespace
        let cors_origins: Vec<String> = cors_origins_str
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        if cors_origins.is_empty() {
            return Err(InfrastructureError::ConfigError(
                "CORS_ORIGINS must contain at least one origin".to_string(),
            ));
        }

        Ok(Config {
            data_dir,
            papers_dir,
            embeddings_dir,
            cache_dir,
            server_port,
            cors_origins,
        })
    }

    /// Helper to resolve a subdirectory path
    fn resolve_subdir(base: &PathBuf, subdir: &str) -> PathBuf {
        let mut path = base.clone();
        path.push(subdir);
        path
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_config_default_values() {
        // Clear any environment variables
        env::remove_var("DATA_DIR");
        env::remove_var("SERVER_PORT");
        env::remove_var("CORS_ORIGINS");

        let config = Config::from_env().unwrap();

        assert_eq!(config.data_dir, PathBuf::from("src/data"));
        assert_eq!(
            config.papers_dir,
            PathBuf::from("src/data/papers_by_id")
        );
        assert_eq!(
            config.embeddings_dir,
            PathBuf::from("src/data/embeddings_by_id")
        );
        assert_eq!(config.cache_dir, PathBuf::from("src/data/cache"));
        assert_eq!(config.server_port, 8000);
        assert_eq!(config.cors_origins.len(), 3);
        assert!(config.cors_origins.contains(&"http://localhost:3000".to_string()));
        assert!(config.cors_origins.contains(&"http://localhost:3001".to_string()));
        assert!(config.cors_origins.contains(&"http://localhost:5173".to_string()));
    }

    #[test]
    fn test_config_custom_data_dir() {
        env::set_var("DATA_DIR", "/custom/data");
        env::remove_var("SERVER_PORT");
        env::remove_var("CORS_ORIGINS");

        let config = Config::from_env().unwrap();

        assert_eq!(config.data_dir, PathBuf::from("/custom/data"));
        assert_eq!(
            config.papers_dir,
            PathBuf::from("/custom/data/papers_by_id")
        );
        assert_eq!(
            config.embeddings_dir,
            PathBuf::from("/custom/data/embeddings_by_id")
        );
        assert_eq!(config.cache_dir, PathBuf::from("/custom/data/cache"));

        // Clean up
        env::remove_var("DATA_DIR");
    }

    #[test]
    fn test_config_custom_server_port() {
        env::remove_var("DATA_DIR");
        env::set_var("SERVER_PORT", "9000");
        env::remove_var("CORS_ORIGINS");

        let config = Config::from_env().unwrap();

        assert_eq!(config.server_port, 9000);

        // Clean up
        env::remove_var("SERVER_PORT");
    }

    #[test]
    fn test_config_custom_cors_origins() {
        env::remove_var("DATA_DIR");
        env::remove_var("SERVER_PORT");
        env::set_var("CORS_ORIGINS", "http://example.com, http://test.com");

        let config = Config::from_env().unwrap();

        assert_eq!(config.cors_origins.len(), 2);
        assert!(config.cors_origins.contains(&"http://example.com".to_string()));
        assert!(config.cors_origins.contains(&"http://test.com".to_string()));

        // Clean up
        env::remove_var("CORS_ORIGINS");
    }

    #[test]
    fn test_config_cors_origins_with_extra_whitespace() {
        env::remove_var("DATA_DIR");
        env::remove_var("SERVER_PORT");
        env::set_var("CORS_ORIGINS", "  http://example.com  ,  http://test.com  , ");

        let config = Config::from_env().unwrap();

        // Should trim whitespace and filter empty strings
        assert_eq!(config.cors_origins.len(), 2);
        assert!(config.cors_origins.contains(&"http://example.com".to_string()));
        assert!(config.cors_origins.contains(&"http://test.com".to_string()));

        // Clean up
        env::remove_var("CORS_ORIGINS");
    }

    #[test]
    fn test_config_invalid_server_port_zero() {
        env::remove_var("DATA_DIR");
        env::set_var("SERVER_PORT", "0");
        env::remove_var("CORS_ORIGINS");

        let result = Config::from_env();

        assert!(result.is_err());
        match result {
            Err(InfrastructureError::ConfigError(msg)) => {
                assert!(msg.contains("SERVER_PORT must be greater than 0"));
            }
            _ => panic!("Expected ConfigError"),
        }

        // Clean up
        env::remove_var("SERVER_PORT");
    }

    #[test]
    fn test_config_invalid_server_port_non_numeric() {
        env::remove_var("DATA_DIR");
        env::set_var("SERVER_PORT", "not_a_number");
        env::remove_var("CORS_ORIGINS");

        let config = Config::from_env().unwrap();

        // Should fall back to default when parsing fails
        assert_eq!(config.server_port, 8000);

        // Clean up
        env::remove_var("SERVER_PORT");
    }

    #[test]
    fn test_config_empty_cors_origins() {
        env::remove_var("DATA_DIR");
        env::remove_var("SERVER_PORT");
        env::set_var("CORS_ORIGINS", "");

        let result = Config::from_env();

        assert!(result.is_err());
        match result {
            Err(InfrastructureError::ConfigError(msg)) => {
                assert!(msg.contains("CORS_ORIGINS must contain at least one origin"));
            }
            _ => panic!("Expected ConfigError"),
        }

        // Clean up
        env::remove_var("CORS_ORIGINS");
    }

    #[test]
    fn test_config_single_cors_origin() {
        env::remove_var("DATA_DIR");
        env::remove_var("SERVER_PORT");
        env::set_var("CORS_ORIGINS", "http://example.com");

        let config = Config::from_env().unwrap();

        assert_eq!(config.cors_origins.len(), 1);
        assert_eq!(config.cors_origins[0], "http://example.com");

        // Clean up
        env::remove_var("CORS_ORIGINS");
    }
}
