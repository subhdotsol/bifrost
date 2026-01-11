use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// AI configuration for GLM API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIConfig {
    pub api_key: String,
    #[serde(default = "default_model")]
    pub model: String,
    #[serde(default = "default_base_url")]
    pub base_url: String,
    #[serde(default = "default_enabled")]
    pub enabled: bool,
}

fn default_model() -> String {
    "gemini-2.0-flash".to_string() // Google Gemini 2.0 Flash - fast and free
}

fn default_base_url() -> String {
    "https://generativelanguage.googleapis.com/v1beta".to_string() // Google Gemini API
}

fn default_enabled() -> bool {
    true
}

impl Default for AIConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            model: default_model(),
            base_url: default_base_url(),
            enabled: default_enabled(),
        }
    }
}

impl AIConfig {
    /// Get the config file path
    fn get_config_path() -> Option<PathBuf> {
        ProjectDirs::from("", "", "vimgram").map(|p| p.config_dir().join("ai.json"))
    }

    /// Load config from file or environment
    pub fn load() -> Self {
        // First check environment variable
        if let Ok(api_key) = std::env::var("VIMGRAM_AI_KEY") {
            return Self {
                api_key,
                ..Default::default()
            };
        }

        // Then check config file
        if let Some(path) = Self::get_config_path() {
            if path.exists() {
                if let Ok(contents) = fs::read_to_string(&path) {
                    if let Ok(config) = serde_json::from_str(&contents) {
                        return config;
                    }
                }
            }
        }

        Self::default()
    }

    /// Save config to file
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(path) = Self::get_config_path() {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }
            let contents = serde_json::to_string_pretty(self)?;
            fs::write(path, contents)?;
        }
        Ok(())
    }

    /// Check if AI is configured and enabled
    pub fn is_ready(&self) -> bool {
        self.enabled && !self.api_key.is_empty()
    }
}
