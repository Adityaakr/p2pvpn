#![no_std]
extern crate alloc;

use alloc::collections::BTreeMap;
use sails_rs::prelude::*;

/// VPN configuration file fetched from a provider.
#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum VpnFile {
    Wireguard(Vec<u8>),
    OpenVpn(Vec<u8>),
}

impl VpnFile {
    pub fn kind(&self) -> &'static str {
        match self {
            VpnFile::Wireguard(_) => "WireGuard",
            VpnFile::OpenVpn(_) => "OpenVPN",
        }
    }

    pub fn bytes(&self) -> &[u8] {
        match self {
            VpnFile::Wireguard(b) | VpnFile::OpenVpn(b) => b,
        }
    }
}

struct P2PvpnContract {
    providers: BTreeMap<u64, String>,
}

impl P2PvpnContract {
    fn create() -> Self {
        Self {
            providers: BTreeMap::new(),
        }
    }
}

#[sails_rs::service]
impl P2PvpnContract {
    /// Fetch the list of available VPN providers.
    #[export]
    pub fn fetch_providers(&mut self) -> Vec<(u64, String)> {
        self.providers
            .iter()
            .map(|(k, v)| (*k, v.clone()))
            .collect()
    }

    /// Fetch the VPN configuration file for a given provider.
    #[export]
    pub fn fetch_provider_file(&mut self, provider: u64) -> VpnFile {
        // Alternate between WireGuard and OpenVPN based on the first byte.
        let raw = format!(
            "# Mock VPN config for provider {:02x}\n[Interface]\nAddress = 10.0.0.{}\n",
            provider.0[0], provider.0[0]
        )
        .into_bytes();

        if provider.0[0] % 2 == 0 {
            VpnFile::OpenVpn(raw)
        } else {
            VpnFile::Wireguard(raw)
        }
    }

    /// Rank a provider positively or negatively after a connection attempt.
    #[export]
    pub fn rank_provider(&mut self, good: bool, _provider: u64) {
        // In a real implementation this would call a smart-contract transaction.
        let _action = if good { "upvoted" } else { "downvoted" };
    }
}

#[derive(Default)]
pub struct Program(());

#[sails_rs::program]
impl Program {
    // Program's constructor
    pub fn create() -> Self {
        Self(())
    }

    // Exposed service
    pub fn p2pvpn_contract(&self) -> P2PvpnContract {
        P2PvpnContract::create()
    }
}
