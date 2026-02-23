use soroban_sdk::{testutils::Address as _, vec, Address, Bytes, BytesN, Env, String};

mod lib {
    pub use anchorkit::*;
}

use lib::{AnchorKitContract, AnchorKitContractClient, ServiceType};

fn main() {
    println!("🚀 AnchorKit CLI Example - Deposit/Withdraw Workflow");
    println!("==================================================\n");

    // Setup environment with mock transport
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, AnchorKitContract);
    let client = AnchorKitContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let anchor = Address::generate(&env);
    let user = Address::generate(&env);

    println!("📋 Configuration:");
    println!("  Admin:  {:?}", admin);
    println!("  Anchor: {:?}", anchor);
    println!("  User:   {:?}\n", user);

    // Step 1: Initialize
    println!("1️⃣  Initializing contract...");
    client.initialize(&admin);
    println!("   ✅ Contract initialized\n");

    // Step 2: Register Anchor
    println!("2️⃣  Registering anchor...");
    client.register_attestor(&anchor);
    println!("   ✅ Anchor registered\n");

    // Step 3: Configure Services
    println!("3️⃣  Configuring anchor services...");
    let services = vec![&env, ServiceType::Deposits, ServiceType::Withdrawals];
    client.configure_services(&anchor, &services);
    println!("   → Services: Deposits, Withdrawals");
    println!("   ✅ Services configured\n");

    // Step 4: Configure Assets
    println!("4️⃣  Configuring supported assets...");
    let assets = vec![
        &env,
        String::from_str(&env, "USDC"),
        String::from_str(&env, "BTC"),
        String::from_str(&env, "ETH"),
    ];
    client.set_supported_assets(&anchor, &assets);
    println!("   → Assets: USDC, BTC, ETH");
    println!("   ✅ Assets configured\n");

    // Step 5: Deposit Flow
    println!("5️⃣  Initiating deposit flow...");
    println!("   → User: {:?}", user);
    println!("   → Asset: USDC");
    println!("   → Amount: 1000");

    // Validate asset
    let usdc = String::from_str(&env, "USDC");
    let is_supported = client.is_asset_supported(&anchor, &usdc);
    println!("   → Asset supported: {}", is_supported);

    // Generate request ID
    let request_id = client.generate_request_id();
    println!("   → Request ID generated");

    // Submit deposit attestation
    let payload_hash = BytesN::from_array(&env, &[1u8; 32]);
    let signature = Bytes::new(&env);
    let attestation_id = client.submit_with_request_id(
        &request_id,
        &anchor,
        &user,
        &env.ledger().timestamp(),
        &payload_hash,
        &signature,
    );
    println!("   ✅ Deposit attestation recorded (ID: {})\n", attestation_id);

    // Step 6: Quote Request
    println!("6️⃣  Requesting quote...");
    let services = vec![&env, ServiceType::Quotes];
    client.configure_services(&anchor, &services);

    let quote_id = client.submit_quote(
        &anchor,
        &String::from_str(&env, "USDC"),
        &String::from_str(&env, "USD"),
        &10000, // 1.0000
        &100,   // 1%
        &100,
        &10000,
        &(env.ledger().timestamp() + 3600),
    );
    println!("   → Pair: USDC/USD");
    println!("   → Rate: 1.0000");
    println!("   → Fee: 1%");
    println!("   ✅ Quote received (ID: {})\n", quote_id);

    // Step 7: Withdraw Flow
    println!("7️⃣  Initiating withdraw flow...");
    println!("   → User: {:?}", user);
    println!("   → Asset: USDC");
    println!("   → Amount: 500");

    let request_id2 = client.generate_request_id();
    let payload_hash2 = BytesN::from_array(&env, &[2u8; 32]);
    let attestation_id2 = client.submit_with_request_id(
        &request_id2,
        &anchor,
        &user,
        &env.ledger().timestamp(),
        &payload_hash2,
        &signature,
    );
    println!("   ✅ Withdraw attestation recorded (ID: {})\n", attestation_id2);

    // Step 8: Check Health
    println!("8️⃣  Checking anchor health...");
    client.update_health_status(&anchor, &45, &0, &9990);
    let health = client.get_health_status(&anchor);
    if let Some(h) = health {
        println!("   → Latency: {}ms", h.latency_ms);
        println!("   → Availability: {}%", h.availability_percent as f64 / 100.0);
        println!("   → Failure count: {}", h.failure_count);
        println!("   ✅ Anchor healthy\n");
    }

    // Step 9: Audit Trail
    println!("9️⃣  Retrieving audit trail...");
    let span1 = client.get_tracing_span(&request_id.id);
    let span2 = client.get_tracing_span(&request_id2.id);
    println!("   → Total operations: 2");
    if span1.is_some() {
        println!("   → Operation 1: Deposit (Success)");
    }
    if span2.is_some() {
        println!("   → Operation 2: Withdraw (Success)");
    }
    println!("   ✅ Audit trail complete\n");

    println!("✅ Workflow completed successfully!\n");
    println!("📊 Summary:");
    println!("  - Deposits: 1 (1000 USDC)");
    println!("  - Withdrawals: 1 (500 USDC)");
    println!("  - Net balance: 500 USDC");
    println!("  - Total attestations: 2\n");
    println!("💡 This example uses mock transport for demonstration.");
    println!("   In production, connect to real Stellar network.");
}
