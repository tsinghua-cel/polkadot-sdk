use std::env;
use tsattack_client::AttackClient;
use once_cell::sync::OnceCell;
use std::sync::Arc;

static ATTACKER: OnceCell<Arc<AttackClient>> = OnceCell::new();

fn create_attack_client(endpoint: &str) -> Option<Arc<AttackClient>> {
    let runtime = tokio::runtime::Runtime::new().ok()?;
    let client = runtime.block_on(AttackClient::connect(endpoint)).ok()?;
    Some(Arc::new(client))
}

pub fn get_attacker() -> Option<Arc<AttackClient>> {
    ATTACKER.get_or_try_init(|| {
        let endpoint = env::var("TSATTACK_SERVICE_URL").ok()?;
        create_attack_client(&endpoint)
    }).cloned()
}