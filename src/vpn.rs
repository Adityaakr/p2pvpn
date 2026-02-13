use std::thread;
use std::time::Duration;

use rand::Rng;

use crate::contract::{self, H256};

/// Simulates a VPN connection attempt. Returns `true` on success.
pub fn try_connect(provider: H256) -> bool {
    let _file = contract::fetch_provider_file(provider);

    // Simulate connection delay.
    thread::sleep(Duration::from_secs(1));

    // 70 % chance of success in this mock.
    let success = rand::thread_rng().gen_bool(0.7);

    contract::rank_provider(success, provider);
    success
}
