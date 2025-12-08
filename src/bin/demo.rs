// src/bin/demo.rs
// Demo showcasing Psy Integration + SDKey Verification + Market Validation

use spectre_protocol::*;
use tokio::time::{sleep, Duration};
use colored::*;

#[tokio::main]
async fn main() {
    println!("{}", "=== SPECTRE PROTOCOL DEMO ===".bold().purple());
    
    // 1. Show Market Validation Data
    println!("\n{}", "📊 MARKET VALIDATION".bold().cyan());
    let market_report = generate_market_validation();
    println!("Total Market Size: ${:.1}B", market_report.total_market_size as f64 / 1_000_000_000.0);
    println!("Total Agents Affected: {}", market_report.total_agents);
    println!("{}", market_report.competitive_advantage);
    
    // 2. Initialize Psy Client
    println!("\n{}", "🔗 PSY PROTOCOL INTEGRATION".bold().cyan());
    let mut psy_client = PsyClient::new();
    println!("Connected to: {}", psy_client.testnet_url);
    
    // 3. Initialize SDKey Registry
    println!("\n{}", "🔐 SDKEY VERIFICATION SYSTEM".bold().cyan());
    let mut sdkey_registry = SDKeyRegistry::new();
    
    // Register some test agents
    sdkey_registry.register_agent("agent_001".to_string(), "LLaMA-3-70B".to_string(), 5000).unwrap();
    sdkey_registry.register_agent("agent_002".to_string(), "GPT-4-Turbo".to_string(), 3000).unwrap();
    println!("✅ Registered 2 agents with verified capabilities");
    
    // 4. Simulate Agent Transactions
    println!("\n{}", "⚡ LIVE AGENT SIMULATION".bold().cyan());
    
    for i in 0..5 {
        let agent_id = format!("agent_{:03}", (i % 2) + 1);
        let model_type = if i % 2 == 0 { "LLaMA-3-70B" } else { "GPT-4-Turbo" };
        
        // Generate SDKey proof
        let proof = generate_mock_proof(agent_id.clone(), model_type);
        
        // Verify with SDKey system
        match sdkey_registry.verify_sdkey_proof(&proof, 4000) {
            Ok(true) => {
                println!("✅ {} verified for {}", agent_id.green(), model_type);
                
                // Submit to Psy Protocol
                match psy_client.submit_agent_transaction(
                    agent_id.clone(), 
                    "inference".to_string(), 
                    100
                ).await {
                    Ok(tx_id) => {
                        println!("📡 Submitted to Psy PARTH: {}", tx_id.blue());
                        
                        // Check status after delay
                        sleep(Duration::from_millis(100)).await;
                        if let Some(status) = psy_client.check_transaction_status(&tx_id).await {
                            println!("🎯 Transaction status: {:?}", status);
                        }
                    },
                    Err(e) => println!("❌ Psy submission failed: {}", e.red()),
                }
            },
            Ok(false) => println!("❌ {} verification failed", agent_id.red()),
            Err(e) => println!("❌ {} error: {}", agent_id.red(), e),
        }
        
        sleep(Duration::from_millis(200)).await;
    }
    
    // 5. Show Network Stats
    println!("\n{}", "📈 PSY NETWORK STATS".bold().cyan());
    let stats = psy_client.get_network_stats();
    println!("Active Agents: {}", stats.active_agents);
    println!("Current TPS: {}", stats.tps);
    println!("Avg Latency: {}ms", stats.avg_latency_ms);
    println!("PARTH Depth: {}", stats.parth_depth);
    
    // 6. Show Latency Benchmarks
    println!("\n{}", "⏱️  LATENCY BENCHMARKS".bold().cyan());
    for (network, latency, note) in get_latency_benchmarks() {
        let color = if latency <= 100 { "green" } else if latency <= 500 { "yellow" } else { "red" };
        println!("{}: {}ms - {}", 
            network, 
            format!("{}", latency).color(color), 
            note
        );
    }
    
    println!("\n{}", "🏆 Demo Complete - Ready for Hackathon Judging!".bold().green());
}