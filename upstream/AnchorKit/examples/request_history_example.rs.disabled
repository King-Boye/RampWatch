/// Example demonstrating the Request History Panel feature
/// 
/// This example shows how to:
/// 1. Track API calls automatically
/// 2. Retrieve request history
/// 3. View detailed information about specific calls
/// 4. Monitor API call statistics

use soroban_sdk::{testutils::Address as _, Address, Bytes, BytesN, Env, String};

// Import the contract types
extern crate anchorkit;
use anchorkit::{AnchorKitContract, AnchorKitContractClient};

fn main() {
    println!("=== AnchorKit Request History Panel Example ===\n");

    // Setup environment
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, AnchorKitContract);
    let client = AnchorKitContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let attestor1 = Address::generate(&env);
    let attestor2 = Address::generate(&env);
    let subject = Address::generate(&env);

    println!("1. Initializing contract...");
    client.initialize(&admin);
    println!("   ✓ Contract initialized\n");

    println!("2. Registering attestors with tracking...");
    client.register_attestor_tracked(&attestor1).unwrap();
    println!("   ✓ Attestor 1 registered");
    
    client.register_attestor_tracked(&attestor2).unwrap();
    println!("   ✓ Attestor 2 registered\n");

    println!("3. Submitting attestations with tracking...");
    let timestamp = env.ledger().timestamp();
    let payload_hash1 = BytesN::from_array(&env, &[1u8; 32]);
    let payload_hash2 = BytesN::from_array(&env, &[2u8; 32]);
    let signature = Bytes::new(&env);

    client
        .submit_attestation_tracked(&attestor1, &subject, &timestamp, &payload_hash1, &signature)
        .unwrap();
    println!("   ✓ Attestation 1 submitted");

    client
        .submit_attestation_tracked(&attestor2, &subject, &timestamp, &payload_hash2, &signature)
        .unwrap();
    println!("   ✓ Attestation 2 submitted\n");

    println!("4. Attempting a failed operation (unregistered attestor)...");
    let unregistered = Address::generate(&env);
    let payload_hash3 = BytesN::from_array(&env, &[3u8; 32]);
    
    let result = client.submit_attestation_tracked(
        &unregistered,
        &subject,
        &timestamp,
        &payload_hash3,
        &signature,
    );
    
    if result.is_err() {
        println!("   ✓ Operation failed as expected (tracked)\n");
    }

    println!("5. Retrieving Request History Panel...");
    let history = client.get_request_history(&10);
    
    println!("\n   === REQUEST HISTORY PANEL ===");
    println!("   Total API Calls: {}", history.total_calls);
    println!("   Successful: {}", history.success_count);
    println!("   Failed: {}", history.failed_count);
    println!("   Last Updated: {}\n", history.last_updated);

    println!("   Recent API Calls:");
    println!("   {:<8} {:<25} {:<12} {:<10}", "Call ID", "Operation", "Status", "Duration");
    println!("   {}", "-".repeat(60));

    for i in 0..history.recent_calls.len() {
        let call = history.recent_calls.get(i).unwrap();
        let status_str = match call.status {
            anchorkit::ApiCallStatus::Success => "Success",
            anchorkit::ApiCallStatus::Failed => "Failed",
            anchorkit::ApiCallStatus::Pending => "Pending",
        };
        
        println!(
            "   {:<8} {:<25} {:<12} {}ms",
            call.call_id,
            call.operation.to_string(),
            status_str,
            call.duration_ms
        );
    }

    println!("\n6. Viewing detailed information for a specific call...");
    if history.recent_calls.len() > 0 {
        let first_call = history.recent_calls.get(0).unwrap();
        let details = client.get_api_call_details(&first_call.call_id);
        
        if let Some(details) = details {
            println!("\n   === CALL DETAILS ===");
            println!("   Call ID: {}", details.record.call_id);
            println!("   Operation: {}", details.record.operation.to_string());
            println!("   Timestamp: {}", details.record.timestamp);
            println!("   Duration: {}ms", details.record.duration_ms);
            
            if let Some(target) = details.target_address {
                println!("   Target Address: {:?}", target);
            }
            
            if let Some(result) = details.result_data {
                println!("   Result: {}", result.to_string());
            }
            
            if let Some(error_code) = details.record.error_code {
                println!("   Error Code: {}", error_code);
            }
        }
    }

    println!("\n7. Configuring services and submitting a quote...");
    let mut services = soroban_sdk::Vec::new(&env);
    services.push_back(anchorkit::ServiceType::Quotes);
    client.configure_services(&attestor1, &services).unwrap();
    println!("   ✓ Services configured");

    let base_asset = String::from_str(&env, "USD");
    let quote_asset = String::from_str(&env, "USDC");
    let rate = 10000u64;
    let fee_percentage = 100u32;
    let minimum_amount = 100u64;
    let maximum_amount = 10000u64;
    let valid_until = env.ledger().timestamp() + 3600;

    client
        .submit_quote_tracked(
            &attestor1,
            &base_asset,
            &quote_asset,
            &rate,
            &fee_percentage,
            &minimum_amount,
            &maximum_amount,
            &valid_until,
        )
        .unwrap();
    println!("   ✓ Quote submitted with tracking\n");

    println!("8. Final request history summary...");
    let final_history = client.get_request_history(&20);
    println!("   Total operations tracked: {}", final_history.total_calls);
    println!("   Success rate: {:.1}%", 
        (final_history.success_count as f64 / final_history.total_calls as f64) * 100.0
    );

    println!("\n=== Example Complete ===");
    println!("\nKey Features Demonstrated:");
    println!("✓ Automatic API call tracking");
    println!("✓ Request history panel with statistics");
    println!("✓ Detailed call information retrieval");
    println!("✓ Success/failure tracking with error codes");
    println!("✓ Timestamp and duration tracking");
    println!("✓ Support for multiple operation types");
}
