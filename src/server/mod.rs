//! Module Server - Control Center Web UI
//!
//! Interface web locale pour LDSI (Axum + HTMX + ECharts)
//! Un seul binaire portable, zéro dépendances externes.
//!
//! Auteur: Julien DABERT
//! LDSI - Lyapunov-Dabert Stability Index

pub mod handlers;
pub mod state;

use axum::{
    Router,
    extract::Extension,
    routing::{get, post},
};
use rust_embed::Embed;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::cors::{Any, CorsLayer};

use state::AppState;

/// Fichiers statiques embarqués dans le binaire
#[derive(Embed)]
#[folder = "static/"]
pub struct StaticFiles;

/// Templates HTML embarqués
#[derive(Embed)]
#[folder = "templates/"]
pub struct Templates;

/// Lance le serveur Control Center
pub async fn start_server(port: u16, openrouter_key: Option<String>) {
    let state = Arc::new(RwLock::new(AppState::new(openrouter_key)));

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        // Pages HTML
        .route("/", get(handlers::dashboard))
        .route("/results/:id", get(handlers::results_page))
        // API endpoints
        .route("/api/benchmark", post(handlers::run_benchmark))
        .route("/api/benchmark/:id/status", get(handlers::benchmark_status))
        .route("/api/topology/:id/:model", get(handlers::get_topology_data))
        .route("/api/models", get(handlers::list_models))
        // Static files
        .route("/static/*path", get(handlers::serve_static))
        .layer(cors)
        .layer(Extension(state));

    let addr = format!("0.0.0.0:{}", port);
    println!();
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║                                                              ║");
    println!("║     ⨎ LDSI CONTROL CENTER ACTIF                            ║");
    println!("║                                                              ║");
    println!(
        "║     http://localhost:{}                                    ║",
        port
    );
    println!("║                                                              ║");
    println!("║     Auteur: Julien DABERT                                   ║");
    println!("║     Zéro réseau de neurones. Que des maths.                 ║");
    println!("║                                                              ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
