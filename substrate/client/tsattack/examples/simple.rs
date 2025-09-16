use tsattack;

/// Simple example showing basic tsattack functionality
fn main() {
    println!("🔧 TSAttack Simple Example");
    println!("===========================");

    // Check if the environment variable is set
    match std::env::var("TSATTACK_SERVICE_URL") {
        Ok(url) => println!("✅ Attack service URL: {}", url),
        Err(_) => {
            println!("⚠️  TSATTACK_SERVICE_URL not set");
            println!("   Set it with: export TSATTACK_SERVICE_URL=http://localhost:50051");
        }
    }

    // Check if attacker is available
    if tsattack::is_attacker_available() {
        println!("✅ Attack service is available!");
        
        // Demonstrate conditional execution (safe to use even if service fails)
        println!("\n🔄 Testing conditional attack operations...");
        
        tsattack::attack_if_enabled! {
            tsattack::report_validator(0, "test_validator")
        };
        
        tsattack::attack_if_enabled! {
            tsattack::report_validator_duty_now("test_validator", 42, 1)
        };
        
        println!("✅ Conditional operations completed (check logs for details)");
    } else {
        println!("❌ Attack service is not available");
        println!("\n💡 This is normal when:");
        println!("   • No attack service is running");
        println!("   • TSATTACK_SERVICE_URL is not set");
        println!("   • Service is unreachable");
        
        println!("\n🚀 To test with a real service:");
        println!("   1. Start an attack service at http://localhost:50051");
        println!("   2. export TSATTACK_SERVICE_URL=http://localhost:50051");
        println!("   3. cargo run --example simple");
    }
    
    println!("\n✨ Example completed!");
}