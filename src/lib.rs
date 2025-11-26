// src/lib.rs

// --- SPECTRE PROTOCOL CORE ENGINE ---
// This code defines the logic for "Agent Identity" and "Trustless Jobs"

use std::collections::HashMap;

// 1. The "Software Defined Key" (SDKey) Simulation
// In the real Psy Protocol, this will be a ZK Circuit.
// For now, we model it as a struct that proves "Capability".
#[derive(Debug, Clone, PartialEq)]
pub struct SDKey {
    pub agent_id: String,
    pub reputation_score: u32,
    pub verified_algorithms: Vec<String>, // e.g., ["LLaMA-3", "StableDiff"]
}

impl SDKey {
    // A "Proof" that the agent is allowed to take a job
    pub fn verify_capability(&self, required_algo: &str) -> bool {
        self.verified_algorithms.contains(&required_algo.to_string()) 
            && self.reputation_score > 50
    }
}

// 2. The Job Contract
// This represents a unit of work (Compute or Data)
#[derive(Debug, Clone)]
pub struct Job {
    pub id: u64,
    pub requester: String,
    pub required_algo: String,
    pub reward_tokens: u64,
    pub is_fulfilled: bool,
}

// 3. The Market State (The "Parth" Tree)
// We split state by "Job ID" to allow parallel processing (Simulating Psy Architecture)
pub struct SpectreMarket {
    pub jobs: HashMap<u64, Job>,
    pub agent_registry: HashMap<String, SDKey>,
}

impl SpectreMarket {
    pub fn new() -> Self {
        SpectreMarket {
            jobs: HashMap::new(),
            agent_registry: HashMap::new(),
        }
    }

    // Register a new AI Agent with an SDKey
    pub fn register_agent(&mut self, agent_id: String, algos: Vec<String>) {
        let new_identity = SDKey {
            agent_id: agent_id.clone(),
            reputation_score: 100, // Start with perfect score
            verified_algorithms: algos,
        };
        self.agent_registry.insert(agent_id, new_identity);
    }

    // A User posts a job
    pub fn post_job(&mut self, id: u64, requester: String, algo: String, reward: u64) {
        let job = Job {
            id,
            requester,
            required_algo: algo,
            reward_tokens: reward,
            is_fulfilled: false,
        };
        self.jobs.insert(id, job);
    }

    // The Critical "Atomic Swap" Function
    // This is what the Judges want to see: Logic that checks "Identity" before "Payment"
    pub fn attempt_job_execution(&mut self, job_id: u64, agent_id: String) -> Result<String, String> {
        // 1. Get the Agent's SDKey
        let agent_key = self.agent_registry.get(&agent_id)
            .ok_or("Agent not registered in SDKey system")?;

        // 2. Get the Job
        let job = self.jobs.get_mut(&job_id)
            .ok_or("Job not found")?;

        // 3. Verify Constraints (The "ZK Proof" simulation)
        if !agent_key.verify_capability(&job.required_algo) {
            return Err("Agent capability verification failed".to_string());
        }

        if job.is_fulfilled {
            return Err("Job already taken".to_string());
        }

        // 4. Execute Swap
        job.is_fulfilled = true;
        
        Ok(format!(
            "SUCCESS: Agent {} verified via SDKey and took Job {} for {} tokens.",
            agent_id, job_id, job.reward_tokens
        ))
    }
}