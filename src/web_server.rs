use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse, Json},
    routing::{get, post},
    Router,
};
use rsiad::{ExerciseConfig, ExerciseType, OutputMode, ToneRange, VocalExerciseEngine};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;

#[derive(Clone)]
struct AppState {
    soundfont_path: String,
}

#[derive(Deserialize)]
struct ExerciseRequest {
    exercise_type: String,
    key_from: u8,
    key_to: u8,
    note_duration: f64,
    output_path: Option<String>,
    realtime: bool,
}

#[derive(Serialize)]
struct ExerciseResponse {
    success: bool,
    message: String,
    notes_played: Option<usize>,
}

#[tokio::main]
async fn main() {
    let soundfont_path = std::env::var("SOUNDFONT_PATH")
        .unwrap_or_else(|_| "UprightPianoKW-small-bright-20190703.sf2".to_string());

    let state = AppState {
        soundfont_path: soundfont_path.clone(),
    };

    let app = Router::new()
        .route("/", get(index_handler))
        .route("/api/exercise", post(exercise_handler))
        .route("/api/health", get(health_handler))
        .with_state(Arc::new(state))
        .layer(CorsLayer::permissive());

    let addr = "127.0.0.1:3000";
    println!("🎵 RSIAD Web Server");
    println!("===================");
    println!("Server running at: http://{}", addr);
    println!("Soundfont: {}", soundfont_path);
    println!("\nOpen http://{} in your browser", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_handler() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "ok",
        "version": env!("CARGO_PKG_VERSION")
    }))
}

async fn exercise_handler(
    State(state): State<Arc<AppState>>,
    Json(req): Json<ExerciseRequest>,
) -> Result<Json<ExerciseResponse>, StatusCode> {
    let exercise_type = match req.exercise_type.as_str() {
        "triads" => ExerciseType::Triads,
        "scales" => ExerciseType::Scales,
        "octaves" => ExerciseType::Octaves,
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    let config = ExerciseConfig {
        exercise_type,
        key_range: (req.key_from, req.key_to),
        note_duration: req.note_duration,
        vocal_range: None,
    };

    let engine = VocalExerciseEngine::new(&state.soundfont_path);

    let output_mode = if req.realtime {
        OutputMode::Realtime
    } else {
        OutputMode::File {
            path: req.output_path.unwrap_or_else(|| "output.mp3".to_string()),
        }
    };

    match engine.generate_exercise(config, output_mode) {
        Ok(result) => Ok(Json(ExerciseResponse {
            success: true,
            message: "Exercise generated successfully".to_string(),
            notes_played: Some(result.notes_played),
        })),
        Err(e) => Ok(Json(ExerciseResponse {
            success: false,
            message: format!("Error: {}", e),
            notes_played: None,
        })),
    }
}

async fn index_handler() -> Html<&'static str> {
    Html(include_str!("../static/index.html"))
}
