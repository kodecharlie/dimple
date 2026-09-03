use axum::{
    extract::{Path, State},
    http::{Method, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use rhai::serde::{from_dynamic, to_dynamic};
use rhai::{Engine, Scope, AST};
use serde_json::{json, Value};
use std::{
    collections::HashMap,
    fs,
    path::PathBuf,
    sync::{Arc, RwLock},
};

#[derive(Clone)]
struct EndpointRegistration {
    method: String,
    name: String,
    input_schema: Value,
    output_schema: Value,
    ast: AST,
}

#[derive(Clone, Default)]
struct AppState {
    engine: Arc<Engine>,
    // Key format: "METHOD:/endpoint-name"
    endpoints: Arc<RwLock<HashMap<String, EndpointRegistration>>>,
}

#[tokio::main]
async fn main() {
    let engine = Engine::new();

    let state = AppState {
        engine: Arc::new(engine),
        endpoints: Arc::new(RwLock::new(HashMap::new())),
    };

    // Load scripts on boot
    load_scripts_from_dir(&state, "./scripts");

    let app = Router::new()
        .route("/_admin/reload", post(reload_scripts))
        .route("/_admin/openapi", get(get_openapi_specs))
        .route("/api/*path", post(dynamic_handler).get(dynamic_handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Server running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}

// Ingests Rhai scripts and registers them dynamically
fn load_scripts_from_dir(state: &AppState, dir_path: &str) {
    let mut endpoints = state.endpoints.write().unwrap();
    endpoints.clear();

    if let Ok(entries) = fs::read_dir(dir_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("rhai") {
                if let Err(err) = register_script(&state.engine, &mut endpoints, &path) {
                    eprintln!("Error loading {:?}: {}", path, err);
                }
            }
        }
    }
}

fn register_script(
    engine: &Engine,
    endpoints: &mut HashMap<String, EndpointRegistration>,
    path: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let script = fs::read_to_string(path)?;
    let ast = engine.compile(&script)?;
    let mut scope = Scope::new();

    let name: String = engine.call_fn(&mut scope, &ast, "endpoint_name", ())?;
    let method: String = engine.call_fn(&mut scope, &ast, "http_method", ())?;
    let input_dyn: rhai::Dynamic = engine.call_fn(&mut scope, &ast, "input_schema", ())?;
    let output_dyn: rhai::Dynamic = engine.call_fn(&mut scope, &ast, "output_schema", ())?;

    let key = format!("{}:/{}", method.to_uppercase(), name);
    endpoints.insert(
        key.clone(),
        EndpointRegistration {
            method: method.to_uppercase(),
            name: name.clone(),
            input_schema: from_dynamic(&input_dyn)?,
            output_schema: from_dynamic(&output_dyn)?,
            ast,
        },
    );

    println!("Registered endpoint: {} -> {}", key, path.display());
    Ok(())
}

// Dynamic Request Dispatcher
async fn dynamic_handler(
    State(state): State<AppState>,
    method: Method,
    Path(path): Path<String>,
    Json(payload): Json<Value>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let key = format!("{}:/{}", method.as_str(), path);
    
    let registration = {
        let guard = state.endpoints.read().unwrap();
        guard.get(&key).cloned()
    };

    let reg = match registration {
        Some(r) => r,
        None => return Err((StatusCode::NOT_FOUND, Json(json!({"error": "Endpoint not found"})))),
    };

    // Prepare inputs for Rhai execution
    let rhai_input = to_dynamic(payload).map_err(|e| {
        (StatusCode::BAD_REQUEST, Json(json!({"error": format!("Invalid JSON: {}", e)})))
    })?;

    let mut scope = Scope::new();
    let result_dyn: rhai::Dynamic = state
        .engine
        .call_fn(&mut scope, &reg.ast, "handle", (rhai_input,))
        .map_err(|e| {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": format!("Script runtime error: {}", e)})))
        })?;

    let result_json: Value = from_dynamic(&result_dyn).map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": format!("Response serialization error: {}", e)})))
    })?;

    Ok(Json(result_json))
}

// Reload scripts ingest endpoint
async fn reload_scripts(State(state): State<AppState>) -> impl IntoResponse {
    load_scripts_from_dir(&state, "./scripts");
    Json(json!({"status": "scripts reloaded"}))
}

// Aggregated OpenAPI Specifications endpoint
async fn get_openapi_specs(State(state): State<AppState>) -> impl IntoResponse {
    let guard = state.endpoints.read().unwrap();
    let specs: Vec<Value> = guard
        .values()
        .map(|reg| {
            json!({
                "endpoint": reg.name,
                "method": reg.method,
                "requestBody": { "content": { "application/json": { "schema": reg.input_schema } } },
                "responses": { "200": { "content": { "application/json": { "schema": reg.output_schema } } } }
            })
        })
        .collect();

    Json(json!({ "endpoints": specs }))
}
