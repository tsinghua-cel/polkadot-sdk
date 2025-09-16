use tsattack;

/// Example of how to use the tsattack wrapper functions
fn main() -> Result<(), String> {
    // Initialize logging (optional)
    env_logger::init();

    println!("TSAttack Usage Example");
    println!("=====================");

    // Set the attack service URL (normally done via environment variable)
    // Note: This example will fail to connect since there's no actual service
    std::env::set_var("TSATTACK_SERVICE_URL", "http://localhost:50051");

    // Check if attacker is available
    if !tsattack::is_attacker_available() {
        println!("⚠️  Attack service is not available");
        println!("   This is expected when running the example without a real attack service.");
        println!("   The following examples show how the API would be used:");
        
        demonstrate_api_usage();
        return Ok(());
    }

    println!("✅ Attack service is available");

    // Example 1: Report validator information
    println!("\n1. Reporting validator information...");
    match tsattack::report_validator(0, "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY") {
        Ok(_) => println!("   ✅ Validator reported successfully"),
        Err(e) => println!("   ❌ Failed to report validator: {}", e),
    }

    // Example 2: Delay a block by hex hash
    println!("\n2. Requesting block delay...");
    match tsattack::delay_block_by_hash("0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef", 123) {
        Ok(_) => println!("   ✅ Block delay requested successfully"),
        Err(e) => println!("   ❌ Failed to request block delay: {}", e),
    }

    // Example 3: Report validator duty with current timestamp
    println!("\n3. Reporting validator duty...");
    match tsattack::report_validator_duty_now("validator_id_1", 42, 1) {
        Ok(_) => println!("   ✅ Validator duty reported successfully"),
        Err(e) => println!("   ❌ Failed to report validator duty: {}", e),
    }

    // Example 4: Report multiple duties at once
    println!("\n4. Reporting multiple duties...");
    let duties = vec![
        ("validator_1".to_string(), 10, 1234567890, 1),
        ("validator_2".to_string(), 11, 1234567891, 2),
    ];
    match tsattack::report_duty(duties) {
        Ok(_) => println!("   ✅ Multiple duties reported successfully"),
        Err(e) => println!("   ❌ Failed to report duties: {}", e),
    }

    // Example 5: Modify block data
    println!("\n5. Modifying block data...");
    let block_data = vec![0x01, 0x02, 0x03, 0x04];
    match tsattack::modify_block(block_data) {
        Ok(success) => println!("   ✅ Block modification success: {}", success),
        Err(e) => println!("   ❌ Failed to modify block: {}", e),
    }

    // Example 6: Using the conditional macro
    println!("\n6. Using conditional execution...");
    tsattack::attack_if_enabled! {
        tsattack::report_validator(1, "another_validator_address")
    };
    println!("   ✅ Conditional execution completed (check logs for any warnings)");

    println!("\n🎉 All attack operations completed!");
    Ok(())
}

/// Demonstrate API usage without actually calling the service
fn demonstrate_api_usage() {
    println!("\n📚 API Usage Examples:");
    println!("   tsattack::report_validator(idx, address)");
    println!("   tsattack::delay_block_by_hash(hash_hex, block_number)");
    println!("   tsattack::report_validator_duty_now(validator, slot, priority)");
    println!("   tsattack::modify_block(data)");
    println!("\n🔧 Conditional execution:");
    println!("   tsattack::attack_if_enabled! {{ operation }};");
    println!("\n💡 To run with a real service:");
    println!("   1. Start your attack service on http://localhost:50051");
    println!("   2. Export TSATTACK_SERVICE_URL=http://localhost:50051");
    println!("   3. Run: cargo run --example usage");
}

/// Example of integration within a hypothetical consensus module
pub fn on_block_imported(block_number: u32, block_hash: &[u8]) {
    // Use conditional execution to avoid errors when attack service is not available
    tsattack::attack_if_enabled! {
        tsattack::delay_for_block(
            block_number as i32,
            block_hash.to_vec(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64
        )
    };
}

/// Example of integration within a validator selection module
pub fn on_validator_selected(validator_idx: usize, validator_address: &str) {
    tsattack::attack_if_enabled! {
        tsattack::report_validator(validator_idx as i32, validator_address)
    };
}

/// Example of integration within a duty scheduler
pub fn on_duty_assigned(validator: &str, slot: u32, priority: u8) {
    tsattack::attack_if_enabled! {
        tsattack::report_validator_duty_now(validator, slot as i32, priority as i32)
    };
}