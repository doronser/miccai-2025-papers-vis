use serde::Deserialize;
use std::fmt;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    #[serde(default = "default_host")]
    pub host: String,

    #[serde(default = "default_port")]
    pub port: u16,

    #[serde(
        default = "default_cors_origins",
        deserialize_with = "deserialize_cors_origins"
    )]
    pub cors_origins: Vec<String>,

    #[serde(default = "default_data_dir")]
    pub data_dir: String,

    #[serde(default = "default_cache_dir")]
    pub cache_dir: String,
}

fn default_host() -> String {
    "127.0.0.1".to_string()
}

fn default_port() -> u16 {
    8000
}

fn default_cors_origins() -> Vec<String> {
    vec![
        "http://localhost:3000".to_string(),
        "http://localhost:3001".to_string(),
        "http://localhost:5173".to_string(),
    ]
}

fn default_data_dir() -> String {
    "src/data".to_string()
}

fn default_cache_dir() -> String {
    "src/cache".to_string()
}

/// Custom deserializer for CORS origins that handles comma-separated strings
fn deserialize_cors_origins<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    // First try to deserialize as a Vec<String> (for config files)
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrVec {
        String(String),
        Vec(Vec<String>),
    }

    let value = StringOrVec::deserialize(deserializer)?;
    match value {
        StringOrVec::Vec(vec) => Ok(vec),
        StringOrVec::String(s) => {
            // Split by comma and clean up whitespace
            Ok(s.split(',')
                .map(|origin| origin.trim().to_string())
                .filter(|origin| !origin.is_empty())
                .collect())
        }
    }
}

#[derive(Debug)]
pub enum ConfigError {
    LoadError(String),
    ValidationError(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::LoadError(msg) => write!(f, "Configuration load error: {}", msg),
            ConfigError::ValidationError(msg) => {
                write!(f, "Configuration validation error: {}", msg)
            }
        }
    }
}

impl std::error::Error for ConfigError {}

impl AppConfig {
    /// Load configuration from environment variables and optional config files.
    ///
    /// Configuration sources are layered in order of precedence (highest to lowest):
    /// 1. Environment variables with APP_ prefix (e.g., APP_HOST, APP_PORT, APP_CORS_ORIGINS)
    /// 2. Config file at config/default (optional)
    /// 3. Default values defined in this module
    ///
    /// The .env file (if present) will be loaded first to populate environment variables.
    pub fn load() -> Result<Self, ConfigError> {
        // Load .env file if present (primarily for development)
        dotenvy::dotenv().ok();

        // Build configuration from multiple sources
        let builder = config::Config::builder()
            // Optional config file (doesn't error if missing)
            .add_source(config::File::with_name("config/default").required(false))
            // Environment variables with APP_ prefix, using __ as separator for nested keys
            .add_source(
                config::Environment::with_prefix("APP")
                    .separator("__")
                    .try_parsing(true),
            );

        let cfg = builder
            .build()
            .map_err(|e| ConfigError::LoadError(e.to_string()))?;

        let config: AppConfig = cfg
            .try_deserialize()
            .map_err(|e| ConfigError::LoadError(e.to_string()))?;

        // Validate configuration
        config.validate()?;

        Ok(config)
    }

    /// Validate the configuration values
    fn validate(&self) -> Result<(), ConfigError> {
        // Validate port range
        if self.port == 0 {
            return Err(ConfigError::ValidationError(
                "Port must be greater than 0".to_string(),
            ));
        }

        // Validate host is not empty
        if self.host.is_empty() {
            return Err(ConfigError::ValidationError(
                "Host cannot be empty".to_string(),
            ));
        }

        // Validate data_dir is not empty
        if self.data_dir.is_empty() {
            return Err(ConfigError::ValidationError(
                "Data directory cannot be empty".to_string(),
            ));
        }

        // Validate cache_dir is not empty
        if self.cache_dir.is_empty() {
            return Err(ConfigError::ValidationError(
                "Cache directory cannot be empty".to_string(),
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_values() {
        let config = AppConfig {
            host: default_host(),
            port: default_port(),
            cors_origins: default_cors_origins(),
            data_dir: default_data_dir(),
            cache_dir: default_cache_dir(),
        };

        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, 8000);
        assert_eq!(config.cors_origins.len(), 3);
        assert_eq!(config.data_dir, "src/data");
        assert_eq!(config.cache_dir, "src/cache");
    }

    #[test]
    fn test_config_deserialization_from_json() {
        // Test deserializing config from JSON with all fields
        let json = r#"{
            "host": "0.0.0.0",
            "port": 9000,
            "cors_origins": "http://example.com,http://test.com",
            "data_dir": "/tmp/data",
            "cache_dir": "/tmp/cache"
        }"#;

        let config: AppConfig = serde_json::from_str(json).expect("Failed to deserialize config");

        assert_eq!(config.host, "0.0.0.0");
        assert_eq!(config.port, 9000);
        assert_eq!(
            config.cors_origins,
            vec!["http://example.com", "http://test.com"]
        );
        assert_eq!(config.data_dir, "/tmp/data");
        assert_eq!(config.cache_dir, "/tmp/cache");
    }

    #[test]
    fn test_cors_origins_parsing_with_whitespace() {
        // Test parsing CORS origins with whitespace
        let json = r#"{
            "cors_origins": " http://localhost:3000 , http://localhost:5173 "
        }"#;

        let config: AppConfig = serde_json::from_str(json).expect("Failed to deserialize config");

        assert_eq!(config.cors_origins.len(), 2);
        assert_eq!(config.cors_origins[0], "http://localhost:3000");
        assert_eq!(config.cors_origins[1], "http://localhost:5173");
    }

    #[test]
    fn test_cors_origins_parsing_single_origin() {
        // Test parsing single CORS origin
        let json = r#"{
            "cors_origins": "http://localhost:3000"
        }"#;

        let config: AppConfig = serde_json::from_str(json).expect("Failed to deserialize config");

        assert_eq!(config.cors_origins.len(), 1);
        assert_eq!(config.cors_origins[0], "http://localhost:3000");
    }

    #[test]
    fn test_cors_origins_parsing_empty_entries() {
        // Test that empty entries are filtered out
        let json = r#"{
            "cors_origins": "http://localhost:3000,,http://localhost:5173,"
        }"#;

        let config: AppConfig = serde_json::from_str(json).expect("Failed to deserialize config");

        // Empty entries should be filtered out
        assert_eq!(config.cors_origins.len(), 2);
        assert_eq!(config.cors_origins[0], "http://localhost:3000");
        assert_eq!(config.cors_origins[1], "http://localhost:5173");
    }

    #[test]
    fn test_cors_origins_as_array() {
        // Test parsing CORS origins as an array (for config files)
        let json = r#"{
            "cors_origins": ["http://localhost:3000", "http://localhost:5173"]
        }"#;

        let config: AppConfig = serde_json::from_str(json).expect("Failed to deserialize config");

        assert_eq!(config.cors_origins.len(), 2);
        assert_eq!(config.cors_origins[0], "http://localhost:3000");
        assert_eq!(config.cors_origins[1], "http://localhost:5173");
    }

    #[test]
    fn test_config_with_defaults() {
        // Test that missing fields use defaults
        let json = r#"{
            "port": 9001
        }"#;

        let config: AppConfig = serde_json::from_str(json).expect("Failed to deserialize config");

        assert_eq!(config.host, "127.0.0.1"); // default
        assert_eq!(config.port, 9001); // overridden
        assert_eq!(config.data_dir, "src/data"); // default
        assert_eq!(config.cache_dir, "src/cache"); // default
        assert_eq!(config.cors_origins.len(), 3); // default
    }

    #[test]
    fn test_validation_invalid_port() {
        let config = AppConfig {
            host: "127.0.0.1".to_string(),
            port: 0,
            cors_origins: vec![],
            data_dir: "src/data".to_string(),
            cache_dir: "src/cache".to_string(),
        };

        assert!(config.validate().is_err());
    }

    #[test]
    fn test_validation_empty_host() {
        let config = AppConfig {
            host: "".to_string(),
            port: 8000,
            cors_origins: vec![],
            data_dir: "src/data".to_string(),
            cache_dir: "src/cache".to_string(),
        };

        assert!(config.validate().is_err());
    }

    #[test]
    fn test_validation_empty_data_dir() {
        let config = AppConfig {
            host: "127.0.0.1".to_string(),
            port: 8000,
            cors_origins: vec![],
            data_dir: "".to_string(),
            cache_dir: "src/cache".to_string(),
        };

        assert!(config.validate().is_err());
    }

    #[test]
    fn test_validation_empty_cache_dir() {
        let config = AppConfig {
            host: "127.0.0.1".to_string(),
            port: 8000,
            cors_origins: vec![],
            data_dir: "src/data".to_string(),
            cache_dir: "".to_string(),
        };

        assert!(config.validate().is_err());
    }
}
