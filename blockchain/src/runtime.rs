use crate::{
    RuntimeCall,
    support::{self, Dispatch},
    types::{self},
};

#[derive(Debug)]
pub struct Config;

impl crate::balances::BalancesConfig for Config {
    type BalanceAmount = u128;
}

impl crate::system::SystemConfig for Config {
    type BlockNumber = u32;
    type Identity = String;
    type NonceCount = u32;
}

impl crate::proof_of_existence::Config for Config {
    type Content = String;
}

#[derive(Debug)]
pub struct Runtime {
    pub balances: crate::balances::Pallet<Config>,
    pub systems: crate::system::Pallets<Config>,
    pub proof_of_existence: crate::proof_of_existence::Pallet<Config>,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            balances: crate::balances::Pallet::new(),
            systems: crate::system::Pallets::new(),
            proof_of_existence: crate::proof_of_existence::Pallet::new(),
        }
    }

    pub fn execute_block(&mut self, block: types::Block) -> support::DispatchResult {
        self.systems.increment_block_number();
        if self.systems.block_number() != block.header.block_number {
            return Err("Block number mismatch");
        }

        for (i, support::Extrinsic { caller, call }) in block.extrinsics.into_iter().enumerate() {
            self.systems.increment_nonce(caller.clone());
            let _ = self.dispatch(caller, call).map_err(|e| eprintln!("{}", e));
        }
        Ok(())
    }
}

impl crate::support::Dispatch for Runtime {
    type Caller = <Config as super::system::SystemConfig>::Identity;
    type Call = RuntimeCall;
    fn dispatch(
        &mut self,
        caller: Self::Caller,
        runtime_call: Self::Call,
    ) -> support::DispatchResult {
        match runtime_call {
            RuntimeCall::Balances(call) => {
                self.balances.dispatch(caller, call)?;
            }
            RuntimeCall::ProofOfExistence(call) => {
                self.proof_of_existence.dispatch(caller, call)?;
            }
        }

        Ok(())
    }
}
