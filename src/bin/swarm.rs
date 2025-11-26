// src/bin/swarm.rs
// --- SPECTRE SWARM SERVER ---
// Broadcasts agent activity to the Frontend Dashboard

use tokio::time::{sleep, Duration};
use rand::Rng;
use colored::*;
use warp::Filter;
use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;

// Define the Data Structure we send to the Frontend
#[derive(Serialize, Clone, Debug)]
struct AgentEvent {
    agent_id: u32,
    status: String, // "SUCCESS" or "FAILED"
    task: String,
    reward: u64,
    latency: u64,
    timestamp: u64,
}

#[tokio::main]
async fn main() {
    println!("{}", "--- SPECTRE NETWORK: INITIALIZING WEBSOCKETS ---".bold().cyan());

    // 1. Create a "Broadcast Channel" (Like a radio station)
    let (tx, _rx) = broadcast::channel::<AgentEvent>(100);
    let tx_clone = tx.clone();

    // 2. Start the Swarm Simulation (In the background)
    tokio::spawn(async move {
        run_swarm_engine(tx_clone).await;
    });

    // 3. Start the Web Server (Listening on port 3030)
    let users = Arc::new(Mutex::new(0)); // Track connected users (optional)
    
    // The WebSocket Route: ws://localhost:3030/spectre
    let ws_route = warp::path("spectre")
        .and(warp::ws())
        .map(move |ws: warp::ws::Ws| {
            let tx = tx.clone();
            ws.on_upgrade(move |socket| handle_connection(socket, tx))
        });

    println!("{}", "[SYSTEM] WebSocket Server Active on ws://127.0.0.1:3030/spectre".green());
    println!("{}", "[SYSTEM] Waiting for Frontend Connection...".yellow());

    warp::serve(ws_route).run(([127, 0, 0, 1], 3030)).await;
}

// Handle a new Frontend connecting
async fn handle_connection(ws: warp::ws::WebSocket, tx: broadcast::Sender<AgentEvent>) {
    use futures::{SinkExt, StreamExt};
    let (mut user_ws_tx, _user_ws_rx) = ws.split();
    let mut rx = tx.subscribe();

    // Loop: When a new agent event happens, send it to the frontend
    while let Ok(event) = rx.recv().await {
        let json = serde_json::to_string(&event).unwrap();
        if let Err(_e) = user_ws_tx.send(warp::ws::Message::text(json)).await {
            break; // Client disconnected
        }
    }
}

// The Simulation Engine (Same as before, but broadcasting events)
async fn run_swarm_engine(tx: broadcast::Sender<AgentEvent>) {
    let agent_count = 100;
    
    loop {
        // Pick a random agent to do work
        let id = rand::thread_rng().gen_range(0..agent_count);
        
        let (is_failure, job_index, work_time) = {
            let mut rng = rand::thread_rng();
            (rng.gen_range(0..100) < 5, rng.gen_range(0..4), rng.gen_range(50..300))
        };

        let status = if is_failure { "FAILED" } else { "SUCCESS" };
        let job_types = vec!["LLaMA-3 Inference", "StableDiff Gen", "Data Scraping", "Arb Trade"];
        let task = job_types[job_index].to_string();
        let reward = rand::thread_rng().gen_range(10..500);

        // Create the event object
        let event = AgentEvent {
            agent_id: id,
            status: status.to_string(),
            task: task.clone(),
            reward,
            latency: work_time,
            timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
        };

        // 1. Broadcast to WebSocket (Frontend)
        let _ = tx.send(event);

        // 2. Print to Terminal (Backend Log)
        if is_failure {
             println!("{} [Agent-{:03}] {}", "✖".red(), id, "SDKey Verification FAILED".red());
        } else {
             println!("{} [Agent-{:03}] {} | ${}", "✔".green(), id, task, reward);
        }

        // Wait a tiny bit before next transaction (High TPS)
        sleep(Duration::from_millis(50)).await; 
    }
}