// src/bin/swarm.rs
// --- SPECTRE SWARM SERVER V3: CHAOS EDITION (STABLE) ---

use tokio::time::{sleep, Duration};
use rand::Rng;
use colored::*;
use warp::Filter;
use serde::{Serialize}; 
use tokio::sync::broadcast;

#[derive(Serialize, Clone, Debug)]
struct AgentEvent {
    agent_id: u32,
    status: String, 
    task: String,     
    tx_hash: String,  
    reward: u64,
    latency: u64,
    timestamp: u64,
}

#[tokio::main]
async fn main() {
    println!("{}", "--- SPECTRE V3: CRYPTO-HASH ENGINE ACTIVE ---".bold().purple());

    let (tx, _rx) = broadcast::channel::<AgentEvent>(100);
    let tx_clone = tx.clone();

    // Spawn the Engine
    tokio::spawn(async move {
        run_swarm_engine(tx_clone).await;
    });

    // Start Server
    let ws_route = warp::path("spectre")
        .and(warp::ws())
        .map(move |ws: warp::ws::Ws| {
            let tx = tx.clone();
            ws.on_upgrade(move |socket| handle_connection(socket, tx))
        });

    println!("{}", "[SYSTEM] WebSocket Server Active on ws://127.0.0.1:3030/spectre".green());
    warp::serve(ws_route).run(([127, 0, 0, 1], 3030)).await;
}

async fn handle_connection(ws: warp::ws::WebSocket, tx: broadcast::Sender<AgentEvent>) {
    use futures::{SinkExt, StreamExt};
    let (mut user_ws_tx, _user_ws_rx) = ws.split();
    let mut rx = tx.subscribe();

    while let Ok(event) = rx.recv().await {
        let json = serde_json::to_string(&event).unwrap();
        if let Err(_e) = user_ws_tx.send(warp::ws::Message::text(json)).await {
            break; 
        }
    }
}

// --- THE CHAOS ENGINE (FIXED) ---
async fn run_swarm_engine(tx: broadcast::Sender<AgentEvent>) {
    let agent_count = 1000;
    
    let models = vec!["LLaMA-3-70B", "GPT-4-Turbo", "Claude-3-Opus", "Mistral-Large"];
    let pairs = vec!["SOL/USDC", "SUI/USDT", "ETH/BTC", "PSY/USDC"];
    let sites = vec!["Twitter", "Reddit", "Bloomberg", "Discord"];
    
    loop {
        // STEP 1: GENERATE DATA (Sync Block)
        // We do all math here so 'rng' is dropped before we ever await.
        let (event, delay_ms, log_msg, _is_error) = {
            let mut rng = rand::thread_rng();

            // Fake Hash
            let tx_hash: String = (0..8).map(|_| format!("{:x}", rng.gen::<u8>())).collect::<String>();
            let short_hash = format!("0x{}...", &tx_hash[0..6]);

            // Task Details
            let job_type = rng.gen_range(0..4);
            let task_detail = match job_type {
                0 => format!("Inference [{}]", models[rng.gen_range(0..models.len())]),
                1 => format!("Gen-Z Proof [{}]", short_hash), 
                2 => format!("Scrape [{}]", sites[rng.gen_range(0..sites.len())]),
                _ => format!("Arb Swap [{}]", pairs[rng.gen_range(0..pairs.len())]),
            };

            // Status
            let is_failure = rng.gen_range(0..100) < 3; 
            let status = if is_failure { "FAILED".to_string() } else { "VERIFIED".to_string() };
            let reward = rng.gen_range(5..1200);
            let fee = rng.gen_range(1..5);
            let delay = rng.gen_range(20..150); // Calculate delay HERE

            let event = AgentEvent {
                agent_id: rng.gen_range(0..agent_count),
                status: status.clone(),
                task: task_detail.clone(),
                tx_hash: short_hash,
                reward,
                latency: rng.gen_range(12..450),
                timestamp: 0,
            };

            // Prepare log string to avoid printing inside the block
            let log = if is_failure {
                format!("{} {} | {}", "✖".red(), status.red(), task_detail)
            } else {
                format!("{} {} | {} | Fee: ${}", "✔".green(), "CONFIRMED".green(), task_detail, fee)
            };

            (event, delay, log, is_failure)
        }; // <--- RNG DIES HERE. Safe to await now.

        // STEP 2: BROADCAST & LOG
        let _ = tx.send(event);
        println!("{}", log_msg);

        // STEP 3: SLEEP (Safe now because delay_ms is just a number)
        sleep(Duration::from_millis(delay_ms)).await; 
    }
}