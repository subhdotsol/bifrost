use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

use super::config::AIConfig;

/// AI API error types
#[derive(Debug)]
pub enum AIError {
    NotConfigured,
    NetworkError(String),
    ApiError(String),
    ParseError(String),
    RateLimited(u64), // seconds to wait
}

impl fmt::Display for AIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AIError::NotConfigured => {
                write!(f, "AI not configured. Set VIMGRAM_AI_KEY or run :ai setup")
            }
            AIError::NetworkError(e) => write!(f, "Network error: {}", e),
            AIError::ApiError(e) => write!(f, "API error: {}", e),
            AIError::ParseError(e) => write!(f, "Parse error: {}", e),
            AIError::RateLimited(secs) => write!(f, "Rate limited. Try again in {}s", secs),
        }
    }
}

impl Error for AIError {}

/// Google Gemini request format
#[derive(Debug, Serialize)]
struct GeminiRequest {
    contents: Vec<GeminiContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system_instruction: Option<GeminiContent>,
    #[serde(rename = "generationConfig", skip_serializing_if = "Option::is_none")]
    generation_config: Option<GenerationConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GeminiContent {
    #[serde(skip_serializing_if = "Option::is_none")]
    role: Option<String>,
    parts: Vec<GeminiPart>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GeminiPart {
    text: String,
}

#[derive(Debug, Serialize)]
struct GenerationConfig {
    temperature: f32,
    #[serde(rename = "maxOutputTokens")]
    max_output_tokens: u32,
}

/// Google Gemini response format
#[derive(Debug, Deserialize)]
struct GeminiResponse {
    candidates: Option<Vec<GeminiCandidate>>,
    error: Option<GeminiError>,
}

#[derive(Debug, Deserialize, Clone)]
struct GeminiCandidate {
    content: GeminiContent,
}

#[derive(Debug, Deserialize)]
struct GeminiError {
    message: String,
}

/// AI Client for Google Gemini API
pub struct AIClient {
    http_client: reqwest::Client,
    config: AIConfig,
}

impl AIClient {
    /// Create a new AI client
    pub fn new(config: AIConfig) -> Self {
        Self {
            http_client: reqwest::Client::new(),
            config,
        }
    }

    /// Check if the client is ready to make requests
    pub fn is_ready(&self) -> bool {
        self.config.is_ready()
    }

    /// Complete a prompt and return the response text
    pub async fn complete(&self, prompt: &str) -> Result<String, AIError> {
        self.complete_with_system(None, prompt).await
    }

    /// Complete a prompt with a system message
    pub async fn complete_with_system(
        &self,
        system: Option<&str>,
        user: &str,
    ) -> Result<String, AIError> {
        if !self.config.is_ready() {
            return Err(AIError::NotConfigured);
        }

        let request = GeminiRequest {
            contents: vec![GeminiContent {
                role: Some("user".to_string()),
                parts: vec![GeminiPart {
                    text: user.to_string(),
                }],
            }],
            system_instruction: system.map(|s| GeminiContent {
                role: None,
                parts: vec![GeminiPart {
                    text: s.to_string(),
                }],
            }),
            generation_config: Some(GenerationConfig {
                temperature: 0.7,
                max_output_tokens: 2048,
            }),
        };

        // Gemini API URL format: {base_url}/models/{model}:generateContent?key={api_key}
        let url = format!(
            "{}/models/{}:generateContent?key={}",
            self.config.base_url, self.config.model, self.config.api_key
        );

        let response = self
            .http_client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| AIError::NetworkError(e.to_string()))?;

        if response.status() == 429 {
            return Err(AIError::RateLimited(60));
        }

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(AIError::ApiError(format!("{}: {}", status, body)));
        }

        let gemini_response: GeminiResponse = response
            .json()
            .await
            .map_err(|e| AIError::ParseError(e.to_string()))?;

        // Check for API error in response
        if let Some(error) = gemini_response.error {
            return Err(AIError::ApiError(error.message));
        }

        // Extract text from response
        gemini_response
            .candidates
            .and_then(|c| c.first().cloned())
            .and_then(|c| c.content.parts.first().cloned())
            .map(|p| p.text)
            .ok_or_else(|| AIError::ParseError("No response from AI".to_string()))
    }

    /// Parse a command from natural language
    pub async fn parse_command(&self, input: &str) -> Result<AICommand, AIError> {
        let system = r#"You are a Telegram command parser. Convert natural language to JSON actions.
Available actions:
- {"action": "mute", "duration_seconds": <int>} - Mute current chat (e.g., 3600 for 1 hour)
- {"action": "unmute"} - Unmute current chat
- {"action": "search", "query": "<text>", "from_user": "<optional username>"} - Search messages
- {"action": "send", "to": "<username>", "text": "<message>"} - Send message to user
- {"action": "reply", "tone": "<casual|formal|technical>"} - Generate a reply draft
- {"action": "unknown", "reason": "<explanation>"} - If you can't understand the command

Respond with ONLY valid JSON, no explanation."#;

        let response = self.complete_with_system(Some(system), input).await?;

        // Try to parse the JSON response
        let trimmed = response.trim();
        // Handle markdown code blocks
        let json_str = if trimmed.starts_with("```") {
            trimmed
                .trim_start_matches("```json")
                .trim_start_matches("```")
                .trim_end_matches("```")
                .trim()
        } else {
            trimmed
        };

        serde_json::from_str(json_str).map_err(|e| {
            AIError::ParseError(format!("Invalid JSON: {} - Response: {}", e, response))
        })
    }

    /// Generate a reply draft based on chat context
    pub async fn generate_reply(
        &self,
        context: &str,
        tone: Option<&str>,
    ) -> Result<String, AIError> {
        let tone_instruction = match tone {
            Some("formal") => "Use a professional, formal tone.",
            Some("technical") => "Use a detailed, technical tone with specific terminology.",
            Some("casual") | _ => "Use a friendly, casual tone.",
        };

        let system = format!(
            r#"You are helping draft a reply in a chat application.
Given the chat history, generate a helpful, concise reply.
{}
Do NOT include greetings unless the conversation warrants it.
Keep the reply brief and natural.
Respond with ONLY the reply text, no quotes or explanation."#,
            tone_instruction
        );

        self.complete_with_system(
            Some(&system),
            &format!("Chat history:\n{}\n\nDraft a reply:", context),
        )
        .await
    }

    /// Generate code or explain programming concepts
    pub async fn code_assist(&self, query: &str) -> Result<String, AIError> {
        let system = r#"You are a coding assistant integrated into a terminal app.
- Respond concisely
- Use markdown code blocks with language tags
- For debugging, explain the issue clearly
- Provide working, practical code examples"#;

        self.complete_with_system(Some(system), query).await
    }
}

/// Parsed AI command
#[derive(Debug, Deserialize)]
#[serde(tag = "action", rename_all = "lowercase")]
pub enum AICommand {
    Mute {
        duration_seconds: u32,
    },
    Unmute,
    Search {
        query: String,
        from_user: Option<String>,
    },
    Send {
        to: String,
        text: String,
    },
    Reply {
        tone: Option<String>,
    },
    Unknown {
        reason: String,
    },
}
