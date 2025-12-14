//! Module Injector - Client API LLM
//!
//! Envoie les prompts aux modèles et récupère les réponses.
//! Compatible OpenAI API, Ollama, OpenRouter, Anthropic.
//!
//! Auteur: Julien DABERT
//! LDSI - Lyapunov-Dabert Stability Index

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Configuration de l'endpoint LLM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmConfig {
    /// URL de base de l'API
    pub base_url: String,
    /// Modèle à utiliser
    pub model: String,
    /// Clé API (optionnel pour Ollama)
    pub api_key: Option<String>,
    /// Timeout en secondes
    pub timeout_secs: u64,
    /// Température (0.0 = déterministe, 1.0+ = créatif)
    pub temperature: f32,
    /// Nombre max de tokens de réponse
    pub max_tokens: u32,
    /// Type d'API
    pub api_type: ApiType,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ApiType {
    /// Format OpenAI (/v1/chat/completions)
    OpenAI,
    /// Format Ollama (/api/generate) - LOCAL FIRST
    Ollama,
    /// Format Anthropic (/v1/messages)
    Anthropic,
    /// OpenRouter (OpenAI-compatible, multi-model gateway)
    OpenRouter,
}

impl Default for LlmConfig {
    fn default() -> Self {
        Self {
            base_url: "http://localhost:11434".to_string(),
            model: "llama3".to_string(),
            api_key: None,
            timeout_secs: 120,
            temperature: 0.7,
            max_tokens: 2048,
            api_type: ApiType::Ollama,
        }
    }
}

impl LlmConfig {
    /// Configuration pour OpenRouter
    pub fn openrouter(model: &str, api_key: &str) -> Self {
        Self {
            base_url: "https://openrouter.ai/api".to_string(),
            model: model.to_string(),
            api_key: Some(api_key.to_string()),
            timeout_secs: 120,
            temperature: 0.7,
            max_tokens: 2048,
            api_type: ApiType::OpenRouter,
        }
    }

    /// Configuration pour Ollama local (fallback prioritaire)
    pub fn ollama_local(model: &str) -> Self {
        Self {
            base_url: "http://localhost:11434".to_string(),
            model: model.to_string(),
            api_key: None,
            timeout_secs: 120,
            temperature: 0.7,
            max_tokens: 2048,
            api_type: ApiType::Ollama,
        }
    }

    /// Configuration pour OpenAI
    pub fn openai(model: &str, api_key: &str) -> Self {
        Self {
            base_url: "https://api.openai.com".to_string(),
            model: model.to_string(),
            api_key: Some(api_key.to_string()),
            timeout_secs: 120,
            temperature: 0.7,
            max_tokens: 2048,
            api_type: ApiType::OpenAI,
        }
    }

    /// Configuration pour Anthropic
    pub fn anthropic(model: &str, api_key: &str) -> Self {
        Self {
            base_url: "https://api.anthropic.com".to_string(),
            model: model.to_string(),
            api_key: Some(api_key.to_string()),
            timeout_secs: 120,
            temperature: 0.7,
            max_tokens: 2048,
            api_type: ApiType::Anthropic,
        }
    }
}

// ============ Modèles OpenRouter populaires ============

/// Liste des modèles OpenRouter courants pour le benchmark
pub mod openrouter_models {
    pub const CLAUDE_3_OPUS: &str = "anthropic/claude-3-opus";
    pub const CLAUDE_3_SONNET: &str = "anthropic/claude-3-sonnet";
    pub const CLAUDE_3_HAIKU: &str = "anthropic/claude-3-haiku";
    pub const GPT_4_TURBO: &str = "openai/gpt-4-turbo";
    pub const GPT_4O: &str = "openai/gpt-4o";
    pub const GPT_35_TURBO: &str = "openai/gpt-3.5-turbo";
    pub const LLAMA_3_70B: &str = "meta-llama/llama-3-70b-instruct";
    pub const LLAMA_3_8B: &str = "meta-llama/llama-3-8b-instruct";
    pub const MISTRAL_LARGE: &str = "mistralai/mistral-large";
    pub const MISTRAL_MEDIUM: &str = "mistralai/mistral-medium";
    pub const MIXTRAL_8X7B: &str = "mistralai/mixtral-8x7b-instruct";
    pub const GEMINI_PRO: &str = "google/gemini-pro";
    pub const COMMAND_R_PLUS: &str = "cohere/command-r-plus";
    pub const QWEN_72B: &str = "qwen/qwen-72b-chat";
    // Modèles non-censurés
    pub const DOLPHIN_MIXTRAL: &str = "cognitivecomputations/dolphin-mixtral-8x7b";
    pub const MYTHOMAX_13B: &str = "gryphe/mythomax-l2-13b";
}

// ============ Structures de requête/réponse OpenAI ============

#[derive(Serialize)]
struct OpenAiRequest {
    model: String,
    messages: Vec<OpenAiMessage>,
    temperature: f32,
    max_tokens: u32,
}

#[derive(Serialize)]
struct OpenAiMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct OpenAiResponse {
    choices: Vec<OpenAiChoice>,
}

#[derive(Deserialize)]
struct OpenAiChoice {
    message: OpenAiMessageResponse,
}

#[derive(Deserialize)]
struct OpenAiMessageResponse {
    content: String,
}

// ============ Structures de requête/réponse Ollama ============

#[derive(Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
    options: OllamaOptions,
}

#[derive(Serialize)]
struct OllamaOptions {
    temperature: f32,
    num_predict: u32,
}

#[derive(Deserialize)]
struct OllamaResponse {
    response: String,
}

// ============ Structures Anthropic ============

#[derive(Serialize)]
struct AnthropicRequest {
    model: String,
    messages: Vec<AnthropicMessage>,
    max_tokens: u32,
    temperature: f32,
}

#[derive(Serialize)]
struct AnthropicMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct AnthropicResponse {
    content: Vec<AnthropicContent>,
}

#[derive(Deserialize)]
struct AnthropicContent {
    text: String,
}

/// Erreur d'injection
#[derive(Debug, Clone)]
pub enum InjectorError {
    NetworkError(String),
    ApiError(String),
    ParseError(String),
    Timeout,
}

impl std::fmt::Display for InjectorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InjectorError::NetworkError(e) => write!(f, "Network error: {}", e),
            InjectorError::ApiError(e) => write!(f, "API error: {}", e),
            InjectorError::ParseError(e) => write!(f, "Parse error: {}", e),
            InjectorError::Timeout => write!(f, "Request timeout"),
        }
    }
}

impl std::error::Error for InjectorError {}

/// Client d'injection LLM
pub struct Injector {
    client: Client,
    config: LlmConfig,
}

impl Injector {
    /// Crée un nouvel injecteur avec la configuration donnée
    pub fn new(config: LlmConfig) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(config.timeout_secs))
            .build()
            .expect("Failed to create HTTP client");

        Self { client, config }
    }

    /// Retourne la configuration actuelle
    pub fn config(&self) -> &LlmConfig {
        &self.config
    }

    /// Envoie un prompt et récupère la réponse
    pub async fn inject(&self, prompt: &str) -> Result<String, InjectorError> {
        match self.config.api_type {
            ApiType::OpenAI => self.inject_openai(prompt).await,
            ApiType::Ollama => self.inject_ollama(prompt).await,
            ApiType::Anthropic => self.inject_anthropic(prompt).await,
            ApiType::OpenRouter => self.inject_openrouter(prompt).await,
        }
    }

    async fn inject_openai(&self, prompt: &str) -> Result<String, InjectorError> {
        let url = format!("{}/v1/chat/completions", self.config.base_url);

        let request = OpenAiRequest {
            model: self.config.model.clone(),
            messages: vec![OpenAiMessage {
                role: "user".to_string(),
                content: prompt.to_string(),
            }],
            temperature: self.config.temperature,
            max_tokens: self.config.max_tokens,
        };

        let mut req_builder = self.client.post(&url).json(&request);

        if let Some(ref api_key) = self.config.api_key {
            req_builder = req_builder.header("Authorization", format!("Bearer {}", api_key));
        }

        let response = req_builder
            .send()
            .await
            .map_err(|e| InjectorError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(InjectorError::ApiError(format!("{}: {}", status, body)));
        }

        let parsed: OpenAiResponse = response
            .json()
            .await
            .map_err(|e| InjectorError::ParseError(e.to_string()))?;

        parsed
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .ok_or_else(|| InjectorError::ParseError("No response content".to_string()))
    }

    async fn inject_ollama(&self, prompt: &str) -> Result<String, InjectorError> {
        let url = format!("{}/api/generate", self.config.base_url);

        let request = OllamaRequest {
            model: self.config.model.clone(),
            prompt: prompt.to_string(),
            stream: false,
            options: OllamaOptions {
                temperature: self.config.temperature,
                num_predict: self.config.max_tokens,
            },
        };

        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| InjectorError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(InjectorError::ApiError(format!("{}: {}", status, body)));
        }

        let parsed: OllamaResponse = response
            .json()
            .await
            .map_err(|e| InjectorError::ParseError(e.to_string()))?;

        Ok(parsed.response)
    }

    async fn inject_anthropic(&self, prompt: &str) -> Result<String, InjectorError> {
        let url = format!("{}/v1/messages", self.config.base_url);

        let request = AnthropicRequest {
            model: self.config.model.clone(),
            messages: vec![AnthropicMessage {
                role: "user".to_string(),
                content: prompt.to_string(),
            }],
            max_tokens: self.config.max_tokens,
            temperature: self.config.temperature,
        };

        let api_key = self
            .config
            .api_key
            .as_ref()
            .ok_or_else(|| InjectorError::ApiError("Anthropic requires API key".to_string()))?;

        let response = self
            .client
            .post(&url)
            .header("x-api-key", api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| InjectorError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(InjectorError::ApiError(format!("{}: {}", status, body)));
        }

        let parsed: AnthropicResponse = response
            .json()
            .await
            .map_err(|e| InjectorError::ParseError(e.to_string()))?;

        parsed
            .content
            .first()
            .map(|c| c.text.clone())
            .ok_or_else(|| InjectorError::ParseError("No response content".to_string()))
    }

    /// Injection via OpenRouter (OpenAI-compatible avec headers spécifiques)
    async fn inject_openrouter(&self, prompt: &str) -> Result<String, InjectorError> {
        let url = format!("{}/v1/chat/completions", self.config.base_url);

        let request = OpenAiRequest {
            model: self.config.model.clone(),
            messages: vec![OpenAiMessage {
                role: "user".to_string(),
                content: prompt.to_string(),
            }],
            temperature: self.config.temperature,
            max_tokens: self.config.max_tokens,
        };

        let api_key = self
            .config
            .api_key
            .as_ref()
            .ok_or_else(|| InjectorError::ApiError("OpenRouter requires API key".to_string()))?;

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("HTTP-Referer", "https://github.com/JulienDbrt/LDSI")
            .header("X-Title", "LDSI Benchmark")
            .json(&request)
            .send()
            .await
            .map_err(|e| InjectorError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(InjectorError::ApiError(format!("{}: {}", status, body)));
        }

        let parsed: OpenAiResponse = response
            .json()
            .await
            .map_err(|e| InjectorError::ParseError(e.to_string()))?;

        parsed
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .ok_or_else(|| InjectorError::ParseError("No response content".to_string()))
    }

    /// Exécute une injection A/B (standard puis fracturé)
    pub async fn inject_ab(
        &self,
        prompt_standard: &str,
        prompt_fractured: &str,
    ) -> Result<(String, String), InjectorError> {
        let response_a = self.inject(prompt_standard).await?;
        let response_b = self.inject(prompt_fractured).await?;
        Ok((response_a, response_b))
    }
}

/// Multi-Injector pour benchmarks parallèles sur plusieurs modèles
pub struct MultiInjector {
    injectors: Vec<(String, Injector)>,
}

impl MultiInjector {
    /// Crée un multi-injecteur vide
    pub fn new() -> Self {
        Self { injectors: Vec::new() }
    }

    /// Ajoute un modèle au benchmark
    pub fn add_model(&mut self, name: &str, config: LlmConfig) {
        self.injectors.push((name.to_string(), Injector::new(config)));
    }

    /// Ajoute un modèle OpenRouter
    pub fn add_openrouter(&mut self, model_id: &str, api_key: &str) {
        let config = LlmConfig::openrouter(model_id, api_key);
        // Extraire le nom court du modèle (après le /)
        let name = model_id.split('/').last().unwrap_or(model_id);
        self.injectors.push((name.to_string(), Injector::new(config)));
    }

    /// Ajoute un modèle Ollama local
    pub fn add_ollama(&mut self, model: &str) {
        let config = LlmConfig::ollama_local(model);
        self.injectors.push((model.to_string(), Injector::new(config)));
    }

    /// Retourne la liste des modèles configurés
    pub fn models(&self) -> Vec<&str> {
        self.injectors.iter().map(|(name, _)| name.as_str()).collect()
    }

    /// Exécute le prompt sur tous les modèles en parallèle
    pub async fn inject_all(&self, prompt: &str) -> Vec<(String, Result<String, InjectorError>)> {
        use futures::future::join_all;

        let futures: Vec<_> = self
            .injectors
            .iter()
            .map(|(name, injector)| {
                let name = name.clone();
                let prompt = prompt.to_string();
                async move {
                    let result = injector.inject(&prompt).await;
                    (name, result)
                }
            })
            .collect();

        join_all(futures).await
    }

    /// Exécute un benchmark A/B sur tous les modèles en parallèle
    pub async fn benchmark_all(
        &self,
        prompt_a: &str,
        prompt_b: &str,
    ) -> Vec<(String, Result<(String, String), InjectorError>)> {
        use futures::future::join_all;

        let futures: Vec<_> = self
            .injectors
            .iter()
            .map(|(name, injector)| {
                let name = name.clone();
                let pa = prompt_a.to_string();
                let pb = prompt_b.to_string();
                async move {
                    let result = injector.inject_ab(&pa, &pb).await;
                    (name, result)
                }
            })
            .collect();

        join_all(futures).await
    }
}

impl Default for MultiInjector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = LlmConfig::default();
        assert_eq!(config.api_type, ApiType::Ollama);
        assert!(config.base_url.contains("11434"));
    }

    #[test]
    fn test_openrouter_config() {
        let config = LlmConfig::openrouter("anthropic/claude-3-opus", "test-key");
        assert_eq!(config.api_type, ApiType::OpenRouter);
        assert!(config.base_url.contains("openrouter.ai"));
    }

    #[test]
    fn test_injector_creation() {
        let config = LlmConfig::default();
        let _injector = Injector::new(config);
    }

    #[test]
    fn test_multi_injector() {
        let mut multi = MultiInjector::new();
        multi.add_ollama("llama3");
        multi.add_openrouter("openai/gpt-4-turbo", "fake-key");
        assert_eq!(multi.models().len(), 2);
    }
}
