# SPECTRE Quick Start Guide
## Get Running in 2 Minutes

### For Judges & Evaluators

**Option 1: Full Demo (Recommended)**
```bash
# Clone and enter directory
git clone <repo-url>
cd spectre_protocol

# Run comprehensive demo
cargo run --bin demo
```

This will show:
- Market validation data ($8.18B TAM)
- Psy Protocol integration
- SDKey verification in action
- Live agent simulation
- Network statistics

**Option 2: Live Dashboard**
```bash
# Terminal 1: Start the swarm engine
cargo run --bin swarm

# Terminal 2: Open dashboard.html in browser
# Or just double-click dashboard.html
```

You'll see:
- Real-time agent transactions
- Live TPS counter
- Average latency metrics
- Interactive pause/resume controls

**Option 3: Run Tests**
```bash
# Run all tests
cargo test

# Run with detailed output
cargo test -- --nocapture
```

---

### For Developers

**1. Install Dependencies**
```bash
# Rust (if not installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone repo
git clone <repo-url>
cd spectre_protocol
```

**2. Basic Usage**
```rust
use spectre_protocol::*;

#[tokio::main]
async fn main() {
    // Initialize Psy client
    let mut psy_client = PsyClient::new();
    
    // Initialize SDKey registry
    let mut registry = SDKeyRegistry::new();
    
    // Register your agent
    registry.register_agent(
        "my_agent".to_string(),
        "LLaMA-3-70B".to_string(),
        5000  // Stake amount
    ).unwrap();
    
    // Generate proof
    let proof = generate_mock_proof("my_agent".to_string(), "LLaMA-3-70B");
    
    // Verify and submit
    if registry.verify_sdkey_proof(&proof, 4000).is_ok() {
        psy_client.submit_agent_transaction(
            "my_agent".to_string(),
            "inference".to_string(),
            100
        ).await.unwrap();
    }
}
```

**3. API Reference**

**Psy Integration:**
```rust
let mut client = PsyClient::new();
client.submit_agent_transaction(agent_id, task_type, amount).await?;
client.check_transaction_status(&tx_id).await;
client.get_network_stats();
```

**SDKey Verification:**
```rust
let mut registry = SDKeyRegistry::new();
registry.register_agent(agent_id, model_type, stake)?;
registry.verify_sdkey_proof(&proof, task_complexity)?;
registry.slash_agent(agent_id, penalty);
registry.reward_agent(agent_id, bonus);
```

**Market Data:**
```rust
let report = generate_market_validation();
let benchmarks = get_latency_benchmarks();
let impact = calculate_economic_impact(current_ms, target_ms, market_size);
```

---

### Troubleshooting

**"Connection refused" on dashboard:**
- Make sure `cargo run --bin swarm` is running first
- Dashboard connects to `ws://127.0.0.1:3030/spectre`

**"Compilation error":**
```bash
# Update Rust
rustup update

# Clean and rebuild
cargo clean
cargo build
```

**"Tests failing":**
```bash
# Make sure you're in the project root
cd spectre_protocol

# Run tests with verbose output
cargo test -- --nocapture
```

---

### Project Structure

```
spectre_protocol/
├── src/
│   ├── lib.rs                    # Core market logic
│   ├── psy_integration.rs        # Psy Protocol client
│   ├── sdkey_verification.rs    # Agent verification system
│   ├── market_validation.rs     # Market data & economics
│   └── bin/
│       ├── swarm.rs              # Live swarm engine
│       └── demo.rs               # Comprehensive demo
├── tests/
│   └── integration_tests.rs     # Test suite
├── dashboard.html                # Real-time UI
├── README.md                     # Main documentation
├── BUSINESS_MODEL.md            # Revenue & GTM strategy
├── ECOSYSTEM_VALUE.md           # Psy integration details
├── PITCH_DECK.md                # Presentation guide
└── QUICKSTART.md                # This file
```

---

### Key Features to Highlight

1. **Sub-100ms Latency:** Watch the dashboard - most transactions <100ms
2. **1000+ TPS:** Swarm engine generates realistic high-frequency load
3. **SDKey Verification:** Cryptographic proof of agent capabilities
4. **Psy PARTH Integration:** Parallel execution, zero state contention
5. **Market Validation:** $8.18B TAM with concrete use cases

---

### Next Steps

- Read [BUSINESS_MODEL.md](BUSINESS_MODEL.md) for revenue strategy
- Read [ECOSYSTEM_VALUE.md](ECOSYSTEM_VALUE.md) for Psy integration
- Read [PITCH_DECK.md](PITCH_DECK.md) for presentation flow
- Explore the code - it's well-commented!

---

### Contact & Support

**Questions?** Check the inline code comments - every module is documented.

**Want to contribute?** See the roadmap in README.md for post-hackathon plans.

**Built for Psy: Ascend Hack 2025** 🚀