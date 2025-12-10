use std::env;
use std::path::PathBuf;

/// Configuration for the MICCAI backend server
///
/// This struct holds all configuration parameters for the backend, including:
/// - Data directory paths (papers, embeddings, cache)
/// - Server runtime configuration (host, port)
/// - CORS allowed origins
///
/// Configuration is loaded from environment variables with sensible defaults.
#[derive(Debug, Clone)]
pub struct Config {
    /// Path to the directory containing paper JSON files
    pub papers_dir: PathBuf,
    /// Path to the directory containing embedding NPZ files
    pub embeddings_dir: PathBuf,
    /// Path to the directory for cache files
    pub cache_dir: PathBuf,
    /// Server bind host address
    pub host: String,
    /// Server bind port
    pub port: u16,
    /// List of allowed CORS origins
    pub cors_allowed_origins: Vec<String>,
}

impl Config {
    /// Load configuration from environment variables with fallback to defaults
    ///
    /// Environment variables:
    /// - `PAPERS_DIR`: Path to papers directory (default: ./data/papers_by_id/)
    /// - `EMBEDDINGS_DIR`: Path to embeddings directory (default: ./data/embeddings_by_id/)
    /// - `CACHE_DIR`: Path to cache directory (default: ./data/cache/)
    /// - `SERVER_HOST`: Server bind address (default: 0.0.0.0)
    /// - `SERVER_PORT`: Server bind port (default: 8000)
    /// - `CORS_ALLOWED_ORIGINS`: Comma-separated list of allowed origins (default: http://localhost:5173)
    ///
    /// # Example
    ///
    /// ```
    /// use infrastructure::config::Config;
    ///
    /// let config = Config::from_env();
    /// println!("Server will bind to {}:{}", config.host, config.port);
    /// ```
    pub fn from_env() -> Self {
        let papers_dir = env::var("PAPERS_DIR")
            .unwrap_or_else(|_| "./data/papers_by_id/".to_string())
            .into();

        let embeddings_dir = env::var("EMBEDDINGS_DIR")
            .unwrap_or_else(|_| "./data/embeddings_by_id/".to_string())
            .into();

        let cache_dir = env::var("CACHE_DIR")
            .unwrap_or_else(|_| "./data/cache/".to_string())
            .into();

        let host = env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());

        let port = env::var("SERVER_PORT")
            .ok()
            .and_then(|p| p.parse::<u16>().ok())
            .unwrap_or(8000);

        let cors_allowed_origins = env::var("CORS_ALLOWED_ORIGINS")
            .unwrap_or_else(|_| "http://localhost:5173".to_string())
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        Config {
            papers_dir,
            embeddings_dir,
            cache_dir,
            host,
            port,
            cors_allowed_origins,
        }
    }

    /// Get the bind address as a string (host:port)
    ///
    /// # Example
    ///
    /// ```
    /// use infrastructure::config::Config;
    ///
    /// let config = Config::from_env();
    /// let bind_addr = config.bind_address();
    /// assert_eq!(bind_addr, "0.0.0.0:8000");
    /// ```
    pub fn bind_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            papers_dir: PathBuf::from("./data/papers_by_id/"),
            embeddings_dir: PathBuf::from("./data/embeddings_by_id/"),
            cache_dir: PathBuf::from("./data/cache/"),
            host: "0.0.0.0".to_string(),
            port: 8000,
            cors_allowed_origins: vec!["http://localhost:5173".to_string()],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::env;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.papers_dir, PathBuf::from("./data/papers_by_id/"));
        assert_eq!(
            config.embeddings_dir,
            PathBuf::from("./data/embeddings_by_id/")
        );
        assert_eq!(config.cache_dir, PathBuf::from("./data/cache/"));
        assert_eq!(config.host, "0.0.0.0");
        assert_eq!(config.port, 8000);
        assert_eq!(config.cors_allowed_origins, vec!["http://localhost:5173"]);
    }

    #[test]
    #[serial]
    fn test_from_env_defaults() {
        // Clear all relevant env vars to test defaults
        env::remove_var("PAPERS_DIR");
        env::remove_var("EMBEDDINGS_DIR");
        env::remove_var("CACHE_DIR");
        env::remove_var("SERVER_HOST");
        env::remove_var("SERVER_PORT");
        env::remove_var("CORS_ALLOWED_ORIGINS");

        let config = Config::from_env();
        assert_eq!(config.papers_dir, PathBuf::from("./data/papers_by_id/"));
        assert_eq!(
            config.embeddings_dir,
            PathBuf::from("./data/embeddings_by_id/")
        );
        assert_eq!(config.cache_dir, PathBuf::from("./data/cache/"));
        assert_eq!(config.host, "0.0.0.0");
        assert_eq!(config.port, 8000);
        assert_eq!(config.cors_allowed_origins, vec!["http://localhost:5173"]);
    }

    #[test]
    #[serial]
    fn test_from_env_overrides() {
        env::set_var("PAPERS_DIR", "/custom/papers");
        env::set_var("EMBEDDINGS_DIR", "/custom/embeddings");
        env::set_var("CACHE_DIR", "/custom/cache");
        env::set_var("SERVER_HOST", "127.0.0.1");
        env::set_var("SERVER_PORT", "9000");
        env::set_var("CORS_ALLOWED_ORIGINS", "http://example.com,http://test.com");

        let config = Config::from_env();
        assert_eq!(config.papers_dir, PathBuf::from("/custom/papers"));
        assert_eq!(config.embeddings_dir, PathBuf::from("/custom/embeddings"));
        assert_eq!(config.cache_dir, PathBuf::from("/custom/cache"));
        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, 9000);
        assert_eq!(
            config.cors_allowed_origins,
            vec!["http://example.com", "http://test.com"]
        );

        // Clean up
        env::remove_var("PAPERS_DIR");
        env::remove_var("EMBEDDINGS_DIR");
        env::remove_var("CACHE_DIR");
        env::remove_var("SERVER_HOST");
        env::remove_var("SERVER_PORT");
        env::remove_var("CORS_ALLOWED_ORIGINS");
    }

    #[test]
    #[serial]
    fn test_cors_origins_parsing() {
        env::set_var("CORS_ALLOWED_ORIGINS", "http://localhost:3000, http://localhost:5173 ,http://example.com");

        let config = Config::from_env();
        assert_eq!(config.cors_allowed_origins.len(), 3);
        assert_eq!(config.cors_allowed_origins[0], "http://localhost:3000");
        assert_eq!(config.cors_allowed_origins[1], "http://localhost:5173");
        assert_eq!(config.cors_allowed_origins[2], "http://example.com");

        env::remove_var("CORS_ALLOWED_ORIGINS");
    }

    #[test]
    #[serial]
    fn test_cors_origins_empty_handling() {
        env::set_var("CORS_ALLOWED_ORIGINS", "http://example.com,,http://test.com, ");

        let config = Config::from_env();
        assert_eq!(config.cors_allowed_origins.len(), 2);
        assert_eq!(config.cors_allowed_origins[0], "http://example.com");
        assert_eq!(config.cors_allowed_origins[1], "http://test.com");

        env::remove_var("CORS_ALLOWED_ORIGINS");
    }

    #[test]
    #[serial]
    fn test_invalid_port_falls_back_to_default() {
        env::set_var("SERVER_PORT", "invalid_port");

        let config = Config::from_env();
        assert_eq!(config.port, 8000); // Should fall back to default

        env::remove_var("SERVER_PORT");
    }

    #[test]
    #[serial]
    fn test_bind_address() {
        let config = Config::default();
        assert_eq!(config.bind_address(), "0.0.0.0:8000");

        env::set_var("SERVER_HOST", "127.0.0.1");
        env::set_var("SERVER_PORT", "9090");
        let config = Config::from_env();
        assert_eq!(config.bind_address(), "127.0.0.1:9090");

        env::remove_var("SERVER_HOST");
        env::remove_var("SERVER_PORT");
    }
}
