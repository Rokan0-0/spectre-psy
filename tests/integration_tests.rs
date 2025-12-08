// tests/integration_tests.rs
// Comprehensive test suite for Spectre Protocol

use spectre_protocol::*;

#[tokio::test]
async fn test_psy_integration() {
    let mut client = PsyClient::new();
    
    let result = client.submit_agent_transaction(
        "test_agent".to_string(),
        "inference".to_string(),
        100
    ).await;
    
    assert!(result.is_ok());
    let tx_id = result.unwrap();
    assert!(tx_id.starts_with("psy_"));
}

#[test]
fn test_sdkey_verification_success() {
    let mut registry = SDKeyRegistry::new();
    
    registry.register_agent(
        "agent_001".to_string(),
        "LLaMA-3-70B".to_string(),
        5000
    ).unwrap();
    
    let proof = generate_mock_proof("agent_001".to_string(), "LLaMA-3-70B");
    let result = registry.verify_sdkey_proof(&proof, 4000);
    assert!(result.is_ok());
}

#[test]
fn test_market_validation_data() {
    let report = generate_market_validation();
    assert!(report.total_market_size > 8_000_000_000);
    assert!(report.total_agents > 100_000);
}

#[test]
fn test_spectre_market_basic() {
    let mut market = SpectreMarket::new();
    market.register_agent("agent_001".to_string(), vec!["LLaMA-3".to_string()]);
    market.post_job(1, "user_001".to_string(), "LLaMA-3".to_string(), 100);
    let result = market.attempt_job_execution(1, "agent_001".to_string());
    assert!(result.is_ok());
}