// src/main.rs
mod lib;
use lib::SpectreMarket;

fn main() {
    println!("--- SPECTRE: INITIALIZING PARTH STATE ---");

    let mut market = SpectreMarket::new();

    // 1. Register an Agent (Simulating On-Chain Identity)
    market.register_agent(
        "agent_007".to_string(), 
        vec!["LLaMA-3".to_string(), "Python-Scripting".to_string()]
    );

    // 2. Post a Job (User Request)
    market.post_job(
        101, 
        "User_Alice".to_string(), 
        "LLaMA-3".to_string(), 
        500
    );

    // 3. Attempt Execution
    match market.attempt_job_execution(101, "agent_007".to_string()) {
        Ok(msg) => println!("{}", msg),
        Err(e) => println!("ERROR: {}", e),
    }
    
    // 4. Attempt with unqualified Agent (Simulating Security Check)
    market.register_agent("bad_actor".to_string(), vec![]);
    match market.attempt_job_execution(101, "bad_actor".to_string()) {
        Ok(msg) => println!("{}", msg),
        Err(e) => println!("SECURITY CHECK PASSED: {}", e), // We expect this to fail
    }
}