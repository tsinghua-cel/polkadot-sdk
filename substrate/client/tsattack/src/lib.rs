//! # TSAttack Client Wrapper
//!
//! This crate provides high-level wrapper functions for the `tsattack-client` package,
//! simplifying the integration of attack services within the Substrate ecosystem.
//!
//! ## Features
//!
//! - **Thread-safe**: Uses `Arc<Mutex<AttackClient>>` for safe concurrent access
//! - **Runtime management**: Built-in Tokio runtime, no external async environment needed
//! - **Environment configuration**: Automatic configuration via `TSATTACK_SERVICE_URL`
//! - **Error handling**: Unified error handling and logging
//! - **Convenience macros**: Conditional execution to avoid errors in non-attack environments
//!
//! ## Usage
//!
//! ```rust
//! use tsattack;
//!
//! // Set environment variable first
//! std::env::set_var("TSATTACK_SERVICE_URL", "http://localhost:50051");
//!
//! // Report validator information
//! tsattack::report_validator(0, "validator_address")?;
//!
//! // Use conditional execution
//! tsattack::attack_if_enabled! {
//!     tsattack::report_validator_duty_now("validator", 42, 1)
//! };
//! ```

use std::env;
use tsattack_client::{AttackClient, attack::{
    ReportValidatorInfoRequest, DelayForBlockRequest, ReportDutyRequest, ModifyBlockRequest,
    ValidatorInfo, BlockInfo, DutyInfo, BlockData
}};
use once_cell::sync::OnceCell;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;

static ATTACKER: OnceCell<Arc<Mutex<AttackClient>>> = OnceCell::new();
static RUNTIME: OnceCell<Runtime> = OnceCell::new();

fn create_attack_client(endpoint: &str) -> Result<Arc<Mutex<AttackClient>>, String> {
    let runtime = get_runtime();
    let client = runtime.block_on(AttackClient::connect(endpoint)).map_err(|e| e.to_string())?;
    Ok(Arc::new(Mutex::new(client)))
}

fn get_runtime() -> &'static Runtime {
    RUNTIME.get_or_init(|| {
        Runtime::new().expect("Failed to create tokio runtime")
    })
}

pub fn get_attacker() -> Option<Arc<Mutex<AttackClient>>> {
    ATTACKER.get_or_try_init(|| {
        let endpoint = env::var("TSATTACK_SERVICE_URL").map_err(|e| e.to_string())?;
        create_attack_client(&endpoint)
    }).ok().cloned()
}

/// Report validator information to the attack service
pub fn report_validator_info(
    idx: i32,
    address: String,
) -> Result<(), String> {
    let attacker = get_attacker().ok_or("Failed to get attack client")?;
    let runtime = get_runtime();
    
    let validator_info = ValidatorInfo {
        idx,
        address,
    };
    
    let request = ReportValidatorInfoRequest {
        info: Some(validator_info),
    };
    
    runtime.block_on(async {
        let mut client = attacker.lock().map_err(|e| e.to_string())?;
        client.report_validator_info(request).await.map_err(|e| e.to_string())?;
        Ok::<(), String>(())
    })
}

/// Request delay for a specific block
pub fn delay_for_block(
    block_number: i32,
    block_hash: Vec<u8>,
    timestamp: i64,
) -> Result<(), String> {
    let attacker = get_attacker().ok_or("Failed to get attack client")?;
    let runtime = get_runtime();
    
    let block_info = BlockInfo {
        number: block_number,
        block_hash,
        timestamp,
    };
    
    let request = DelayForBlockRequest {
        block_info: Some(block_info),
    };
    
    runtime.block_on(async {
        let mut client = attacker.lock().map_err(|e| e.to_string())?;
        client.delay_for_block(request).await.map_err(|e| e.to_string())?;
        Ok::<(), String>(())
    })
}

/// Report duty information
pub fn report_duty(
    duties: Vec<(String, i32, i64, i32)>, // (validator, slot, time, priority)
) -> Result<(), String> {
    let attacker = get_attacker().ok_or("Failed to get attack client")?;
    let runtime = get_runtime();
    
    let duty_infos: Vec<DutyInfo> = duties.into_iter().map(|(validator, slot, time, priority)| {
        DutyInfo {
            validator,
            slot,
            time,
            priority,
        }
    }).collect();
    
    let request = ReportDutyRequest {
        duties: duty_infos,
    };
    
    runtime.block_on(async {
        let mut client = attacker.lock().map_err(|e| e.to_string())?;
        client.report_duty(request).await.map_err(|e| e.to_string())?;
        Ok::<(), String>(())
    })
}

/// Report single duty information (convenience method)
pub fn report_single_duty(
    validator: String,
    slot: i32,
    time: i64,
    priority: i32,
) -> Result<(), String> {
    report_duty(vec![(validator, slot, time, priority)])
}

/// Request modification of a block
pub fn modify_block(
    block_data: Vec<u8>,
) -> Result<bool, String> {
    let attacker = get_attacker().ok_or("Failed to get attack client")?;
    let runtime = get_runtime();
    
    let block_data_msg = BlockData {
        data: block_data,
    };
    
    let request = ModifyBlockRequest {
        block_data: Some(block_data_msg),
    };
    
    runtime.block_on(async {
        let mut client = attacker.lock().map_err(|e| e.to_string())?;
        let response = client.modify_block(request).await.map_err(|e| e.to_string())?;
        Ok(response.into_inner().success)
    })
}

/// Check if the attack client is available
pub fn is_attacker_available() -> bool {
    env::var("TSATTACK_SERVICE_URL").is_ok() && get_attacker().is_some()
}

/// Convenience macro for conditional attack operations
#[macro_export]
macro_rules! attack_if_enabled {
    ($operation:expr) => {
        if $crate::is_attacker_available() {
            if let Err(e) = $operation {
                log::warn!("Attack operation failed: {}", e);
            }
        }
    };
}

/// High-level wrapper functions for common attack scenarios

/// Report a validator with simplified parameters
pub fn report_validator(idx: i32, address: &str) -> Result<(), String> {
    report_validator_info(idx, address.to_string())
}

/// Delay a block by hash (convenience method that accepts hex string)
pub fn delay_block_by_hash(block_hash_hex: &str, block_number: i32) -> Result<(), String> {
    let block_hash = hex::decode(block_hash_hex).map_err(|e| format!("Invalid hex: {}", e))?;
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    
    delay_for_block(block_number, block_hash, timestamp)
}

/// Report validator duty with current timestamp
pub fn report_validator_duty_now(
    validator: &str,
    slot: i32,
    priority: i32,
) -> Result<(), String> {
    let time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    
    report_single_duty(validator.to_string(), slot, time, priority)
}