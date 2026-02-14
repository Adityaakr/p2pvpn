#![no_std]

use sails_rs::prelude::*;

struct P2PvpnContract(());

impl P2PvpnContract {
    pub fn create() -> Self {
        Self(())
    }
}

#[sails_rs::service]
impl P2PvpnContract {
    // Service's method (command)
    #[export]
    pub fn do_something(&mut self) -> String {
        "Hello from P2PvpnContract!".to_string()
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
