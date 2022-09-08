#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Arc;

use serde_json::Value;
use tauri::async_runtime::Mutex;

struct Node {
    host: String,
    id: String,
    password: String,
}

struct State {
    nodes: Vec<Node>,
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
#[tauri::command]
async fn addNode(token: String, mut state: tauri::State<'_, Arc<Mutex<State>>>) -> Result<(), u32> {
    let v = base64::decode(&token);
    println!("token {}", token);
    match v {
        Ok(vec) => match String::from_utf8(vec) {
            Ok(s) => match serde_json::from_str(&s) {
                Ok::<Value, _>(j) => match (j.get("id"), j.get("password"), j.get("host")) {
                    (Some(i), Some(p), Some(h)) => {
                        let id = i.to_string();
                        let password = p.to_string();
                        let host = h.to_string();
                        let mut lk = state.lock().await;
                        println!("Added new node");
                        lk.nodes.push(Node { id, host, password });
                        Ok(())
                    }
                    _ => Err(1),
                },
                Err(_) => Err(2),
            },
            Err(_) => Err(3),
        },
        Err(_) => Err(4),
    }
}

fn main() {
    tauri::Builder::default()
        .manage(Arc::new(Mutex::new(State { nodes: Vec::new() })))
        .invoke_handler(tauri::generate_handler![greet,addNode])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
