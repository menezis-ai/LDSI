//! État de l'application et gestion des sessions de benchmark
//!
//! Auteur: Julien DABERT
//! LDSI - Lyapunov-Dabert Stability Index

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::core::{LdsiResult, LdsiVerdict};
use crate::core::topology::TopologyResult;

/// État global de l'application
pub struct AppState {
    /// Clé API OpenRouter (optionnelle)
    pub openrouter_key: Option<String>,
    /// Sessions de benchmark en cours ou terminées
    pub benchmarks: HashMap<String, BenchmarkSession>,
}

impl AppState {
    pub fn new(openrouter_key: Option<String>) -> Self {
        Self {
            openrouter_key,
            benchmarks: HashMap::new(),
        }
    }

    pub fn create_benchmark(&mut self, request: BenchmarkRequest) -> String {
        let id = Uuid::new_v4().to_string();
        let session = BenchmarkSession {
            id: id.clone(),
            status: BenchmarkStatus::Pending,
            request,
            results: Vec::new(),
            created_at: chrono::Utc::now().to_rfc3339(),
        };
        self.benchmarks.insert(id.clone(), session);
        id
    }

    pub fn get_benchmark(&self, id: &str) -> Option<&BenchmarkSession> {
        self.benchmarks.get(id)
    }

    pub fn update_benchmark(&mut self, id: &str, status: BenchmarkStatus, results: Vec<ModelResult>) {
        if let Some(session) = self.benchmarks.get_mut(id) {
            session.status = status;
            session.results = results;
        }
    }
}

/// Requête de benchmark
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkRequest {
    /// Prompt standard (A)
    pub prompt_a: String,
    /// Prompt fracturé (B)
    pub prompt_b: String,
    /// Liste des modèles à tester
    pub models: Vec<ModelConfig>,
}

/// Configuration d'un modèle pour le benchmark
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    /// Identifiant du modèle (ex: "anthropic/claude-3-opus")
    pub model_id: String,
    /// Nom d'affichage
    pub display_name: String,
    /// Type de provider
    pub provider: ProviderType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProviderType {
    OpenRouter,
    Ollama,
    OpenAI,
    Anthropic,
}

/// Session de benchmark
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkSession {
    pub id: String,
    pub status: BenchmarkStatus,
    pub request: BenchmarkRequest,
    pub results: Vec<ModelResult>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BenchmarkStatus {
    Pending,
    Running,
    Completed,
    Failed(String),
}

/// Résultat pour un modèle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelResult {
    /// Nom du modèle
    pub model_name: String,
    /// Statut de l'exécution
    pub status: ModelStatus,
    /// Réponse standard (A)
    pub response_a: Option<String>,
    /// Réponse fracturée (B)
    pub response_b: Option<String>,
    /// Score LDSI
    pub ldsi: Option<LdsiResultSummary>,
    /// Données de topologie pour visualisation
    pub topology: Option<TopologyData>,
    /// Message d'erreur si échec
    pub error: Option<String>,
    /// Temps d'exécution en ms
    pub duration_ms: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ModelStatus {
    Pending,
    Running,
    Success,
    Failed,
}

/// Résumé du score LDSI pour l'API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LdsiResultSummary {
    pub lambda: f64,
    pub verdict: String,
    pub verdict_class: String,
    pub ncd_score: f64,
    pub entropy_ratio: f64,
    pub topology_delta: f64,
    pub entropy_a: f64,
    pub entropy_b: f64,
    pub ttr_a: f64,
    pub ttr_b: f64,
}

impl From<&LdsiResult> for LdsiResultSummary {
    fn from(result: &LdsiResult) -> Self {
        let (verdict, verdict_class) = match result.verdict {
            LdsiVerdict::Zombie => ("ZOMBIE", "zombie"),
            LdsiVerdict::Rebelle => ("REBELLE", "rebelle"),
            LdsiVerdict::Architecte => ("ARCHITECTE", "architecte"),
            LdsiVerdict::Fou => ("FOU", "fou"),
        };

        Self {
            lambda: result.lambda,
            verdict: verdict.to_string(),
            verdict_class: verdict_class.to_string(),
            ncd_score: result.ncd.score,
            entropy_ratio: result.entropy.ratio,
            topology_delta: result.topology.delta,
            entropy_a: result.entropy.shannon_a,
            entropy_b: result.entropy.shannon_b,
            ttr_a: result.entropy.ttr_a,
            ttr_b: result.entropy.ttr_b,
        }
    }
}

/// Données de topologie pour la visualisation du graphe
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopologyData {
    /// Nœuds du graphe
    pub nodes: Vec<GraphNode>,
    /// Arêtes du graphe
    pub edges: Vec<GraphEdge>,
    /// Métriques
    pub metrics: TopologyMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphNode {
    pub id: String,
    pub label: String,
    pub size: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphEdge {
    pub source: String,
    pub target: String,
    pub weight: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopologyMetrics {
    pub node_count: usize,
    pub edge_count: usize,
    pub density: f64,
    pub clustering: f64,
    pub lcc_ratio: f64,
    pub small_world: f64,
}

impl From<&TopologyResult> for TopologyMetrics {
    fn from(result: &TopologyResult) -> Self {
        Self {
            node_count: result.node_count,
            edge_count: result.edge_count,
            density: result.density,
            clustering: result.clustering_coefficient,
            lcc_ratio: result.lcc_ratio,
            small_world: result.small_world_index,
        }
    }
}

/// Liste des modèles disponibles pour l'interface
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailableModels {
    pub openrouter: Vec<ModelInfo>,
    pub ollama: Vec<ModelInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub id: String,
    pub name: String,
    pub provider: String,
    pub category: String,
}

impl Default for AvailableModels {
    fn default() -> Self {
        Self {
            openrouter: vec![
                ModelInfo { id: "anthropic/claude-3-opus".into(), name: "Claude 3 Opus".into(), provider: "Anthropic".into(), category: "Premium".into() },
                ModelInfo { id: "anthropic/claude-3-sonnet".into(), name: "Claude 3 Sonnet".into(), provider: "Anthropic".into(), category: "Balanced".into() },
                ModelInfo { id: "anthropic/claude-3-haiku".into(), name: "Claude 3 Haiku".into(), provider: "Anthropic".into(), category: "Fast".into() },
                ModelInfo { id: "openai/gpt-4-turbo".into(), name: "GPT-4 Turbo".into(), provider: "OpenAI".into(), category: "Premium".into() },
                ModelInfo { id: "openai/gpt-4o".into(), name: "GPT-4o".into(), provider: "OpenAI".into(), category: "Premium".into() },
                ModelInfo { id: "openai/gpt-3.5-turbo".into(), name: "GPT-3.5 Turbo".into(), provider: "OpenAI".into(), category: "Fast".into() },
                ModelInfo { id: "meta-llama/llama-3-70b-instruct".into(), name: "Llama 3 70B".into(), provider: "Meta".into(), category: "Open".into() },
                ModelInfo { id: "meta-llama/llama-3-8b-instruct".into(), name: "Llama 3 8B".into(), provider: "Meta".into(), category: "Open".into() },
                ModelInfo { id: "mistralai/mistral-large".into(), name: "Mistral Large".into(), provider: "Mistral".into(), category: "Premium".into() },
                ModelInfo { id: "mistralai/mixtral-8x7b-instruct".into(), name: "Mixtral 8x7B".into(), provider: "Mistral".into(), category: "Open".into() },
                ModelInfo { id: "google/gemini-pro".into(), name: "Gemini Pro".into(), provider: "Google".into(), category: "Premium".into() },
                ModelInfo { id: "cohere/command-r-plus".into(), name: "Command R+".into(), provider: "Cohere".into(), category: "Premium".into() },
                // Non-censored
                ModelInfo { id: "cognitivecomputations/dolphin-mixtral-8x7b".into(), name: "Dolphin Mixtral".into(), provider: "Cognitive".into(), category: "Uncensored".into() },
                ModelInfo { id: "gryphe/mythomax-l2-13b".into(), name: "MythoMax 13B".into(), provider: "Gryphe".into(), category: "Uncensored".into() },
            ],
            ollama: vec![
                ModelInfo { id: "llama3".into(), name: "Llama 3".into(), provider: "Local".into(), category: "Open".into() },
                ModelInfo { id: "llama3:70b".into(), name: "Llama 3 70B".into(), provider: "Local".into(), category: "Open".into() },
                ModelInfo { id: "mistral".into(), name: "Mistral 7B".into(), provider: "Local".into(), category: "Open".into() },
                ModelInfo { id: "mixtral".into(), name: "Mixtral 8x7B".into(), provider: "Local".into(), category: "Open".into() },
                ModelInfo { id: "dolphin-mixtral".into(), name: "Dolphin Mixtral".into(), provider: "Local".into(), category: "Uncensored".into() },
                ModelInfo { id: "qwen2".into(), name: "Qwen 2".into(), provider: "Local".into(), category: "Open".into() },
            ],
        }
    }
}
