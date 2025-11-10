use crate::errors::{FsError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// Main configuration file structure
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    /// User preferences
    #[serde(default)]
    pub preferences: Preferences,
    /// Saved query profiles
    #[serde(default)]
    pub profiles: HashMap<String, QueryProfile>,
}

/// User preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preferences {
    /// Default output format
    #[serde(default = "default_format")]
    pub default_format: String,
    /// Enable colored output by default
    #[serde(default = "default_true")]
    pub color: bool,
    /// Number of threads for parallel operations
    #[serde(default = "default_threads")]
    pub threads: usize,
    /// Respect gitignore by default
    #[serde(default = "default_true")]
    pub respect_gitignore: bool,
}

fn default_format() -> String {
    "pretty".to_string()
}

fn default_true() -> bool {
    true
}

fn default_threads() -> usize {
    4
}

impl Default for Preferences {
    fn default() -> Self {
        Self {
            default_format: default_format(),
            color: true,
            threads: 4,
            respect_gitignore: true,
        }
    }
}

/// Saved query profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryProfile {
    /// Profile description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Command to run (list, find, size, etc.)
    pub command: String,
    /// Command arguments as key-value pairs
    #[serde(default)]
    pub args: HashMap<String, serde_json::Value>,
}

impl Config {
    /// Load config from the default location
    pub fn load() -> Result<Self> {
        let config_path = Self::config_file_path()?;

        if !config_path.exists() {
            // Return default config if file doesn't exist
            return Ok(Self::default());
        }

        let content = fs::read_to_string(&config_path).map_err(|e| FsError::PathAccess {
            path: config_path.clone(),
            source: e,
        })?;

        toml::from_str(&content).map_err(|e| FsError::InvalidFormat {
            format: format!("Failed to parse config file: {}", e),
        })
    }

    /// Save config to the default location
    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_file_path()?;

        // Create config directory if it doesn't exist
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent).map_err(|e| FsError::PathAccess {
                path: parent.to_path_buf(),
                source: e,
            })?;
        }

        let content = toml::to_string_pretty(self).map_err(|e| FsError::InvalidFormat {
            format: format!("Failed to serialize config: {}", e),
        })?;

        fs::write(&config_path, content).map_err(|e| FsError::PathAccess {
            path: config_path,
            source: e,
        })
    }

    /// Get the default config file path
    pub fn config_file_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir().ok_or_else(|| FsError::InvalidFormat {
            format: "Could not determine config directory".to_string(),
        })?;

        Ok(config_dir.join("fexplorer").join("config.toml"))
    }

    /// Get the config directory path
    pub fn config_dir() -> Result<PathBuf> {
        let config_dir = dirs::config_dir().ok_or_else(|| FsError::InvalidFormat {
            format: "Could not determine config directory".to_string(),
        })?;

        Ok(config_dir.join("fexplorer"))
    }

    /// Initialize a default config file with examples
    pub fn init() -> Result<()> {
        let config_path = Self::config_file_path()?;

        if config_path.exists() {
            return Err(FsError::InvalidFormat {
                format: format!("Config file already exists at {}", config_path.display()),
            });
        }

        // Create example config with sample profiles
        let mut config = Config::default();

        // Add example profiles
        config.profiles.insert(
            "cleanup".to_string(),
            QueryProfile {
                description: Some("Find old log and temp files for cleanup".to_string()),
                command: "find".to_string(),
                args: {
                    let mut args = HashMap::new();
                    args.insert("ext".to_string(), serde_json::json!(["log", "tmp"]));
                    args.insert("before".to_string(), serde_json::json!("30 days ago"));
                    args.insert("min_size".to_string(), serde_json::json!("1MB"));
                    args
                },
            },
        );

        config.profiles.insert(
            "recent-code".to_string(),
            QueryProfile {
                description: Some("Find recently modified source code files".to_string()),
                command: "find".to_string(),
                args: {
                    let mut args = HashMap::new();
                    args.insert(
                        "ext".to_string(),
                        serde_json::json!(["rs", "go", "ts", "py"]),
                    );
                    args.insert("after".to_string(), serde_json::json!("7 days ago"));
                    args
                },
            },
        );

        config.profiles.insert(
            "large-files".to_string(),
            QueryProfile {
                description: Some("Find files larger than 100MB".to_string()),
                command: "find".to_string(),
                args: {
                    let mut args = HashMap::new();
                    args.insert("min_size".to_string(), serde_json::json!("100MB"));
                    args.insert("kind".to_string(), serde_json::json!(["file"]));
                    args
                },
            },
        );

        config.save()
    }

    /// Get a profile by name
    pub fn get_profile(&self, name: &str) -> Option<&QueryProfile> {
        self.profiles.get(name)
    }

    /// List all profile names
    pub fn profile_names(&self) -> Vec<String> {
        self.profiles.keys().cloned().collect()
    }
}

/// Configuration for px (project switcher)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PxConfig {
    /// Directories to scan for projects
    #[serde(default = "default_scan_dirs")]
    pub scan_dirs: Vec<PathBuf>,

    /// Default editor command
    #[serde(default = "default_editor")]
    pub default_editor: String,

    /// Optional Obsidian vault path for note integration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obsidian_vault: Option<PathBuf>,
}

fn default_scan_dirs() -> Vec<PathBuf> {
    let home = dirs::home_dir().unwrap_or_default();
    vec![
        home.join("Developer"),
        home.join("projects"),
        home.join("code"),
    ]
}

fn default_editor() -> String {
    "code".to_string()
}

impl Default for PxConfig {
    fn default() -> Self {
        Self {
            scan_dirs: default_scan_dirs(),
            default_editor: default_editor(),
            obsidian_vault: None,
        }
    }
}

impl PxConfig {
    /// Load px config from ~/.config/px/config.toml
    pub fn load() -> Result<Self> {
        let config_path = Self::config_file_path()?;

        if !config_path.exists() {
            return Ok(Self::default());
        }

        let content = fs::read_to_string(&config_path).map_err(|e| FsError::PathAccess {
            path: config_path.clone(),
            source: e,
        })?;

        toml::from_str(&content).map_err(|e| FsError::InvalidFormat {
            format: format!("Failed to parse px config: {}", e),
        })
    }

    /// Save px config
    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_file_path()?;

        // Create config directory if it doesn't exist
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent).map_err(|e| FsError::PathAccess {
                path: parent.to_path_buf(),
                source: e,
            })?;
        }

        let content = toml::to_string_pretty(self).map_err(|e| FsError::InvalidFormat {
            format: format!("Failed to serialize px config: {}", e),
        })?;

        fs::write(&config_path, content).map_err(|e| FsError::PathAccess {
            path: config_path,
            source: e,
        })
    }

    /// Get config file path (~/.config/px/config.toml)
    pub fn config_file_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir().ok_or_else(|| FsError::InvalidFormat {
            format: "Could not determine config directory".to_string(),
        })?;

        Ok(config_dir.join("px").join("config.toml"))
    }

    /// Initialize default px config with helpful comments
    pub fn init() -> Result<()> {
        let config_path = Self::config_file_path()?;

        if config_path.exists() {
            println!("Config file already exists at: {}", config_path.display());
            println!("Edit manually or delete to regenerate");
            return Ok(());
        }

        let config = Self::default();
        config.save()?;

        println!("✓ Created px config at: {}", config_path.display());
        println!();
        println!("Edit this file to customize:");
        println!("  - scan_dirs: directories to search for projects");
        println!("  - default_editor: editor command (code, cursor, vim, etc.)");
        println!("  - obsidian_vault: optional Obsidian vault path");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.preferences.default_format, "pretty");
        assert!(config.preferences.color);
        assert_eq!(config.preferences.threads, 4);
        assert!(config.preferences.respect_gitignore);
    }

    #[test]
    fn test_config_serialization() {
        let mut config = Config::default();
        config.profiles.insert(
            "test".to_string(),
            QueryProfile {
                description: Some("Test profile".to_string()),
                command: "list".to_string(),
                args: HashMap::new(),
            },
        );

        let toml_str = toml::to_string(&config).unwrap();
        assert!(toml_str.contains("[preferences]"));
        assert!(toml_str.contains("profiles.test"));
    }

    #[test]
    fn test_config_deserialization() {
        let toml_str = r#"
            [preferences]
            default_format = "json"
            color = false
            threads = 8

            [profiles.example]
            description = "Example profile"
            command = "find"
            args = { ext = ["rs"] }
        "#;

        let config: Config = toml::from_str(toml_str).unwrap();
        assert_eq!(config.preferences.default_format, "json");
        assert!(!config.preferences.color);
        assert_eq!(config.preferences.threads, 8);
        assert!(config.profiles.contains_key("example"));
    }
}
