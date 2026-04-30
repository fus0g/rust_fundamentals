use core::fmt::Debug;
use std::collections::BTreeMap;

use crate::support::DispatchResult;

pub trait Config: crate::system::SystemConfig {
    type Content: Debug + Ord;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
    claims: BTreeMap<T::Content, T::Identity>,
}

impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Self {
            claims: BTreeMap::new(),
        }
    }

    pub fn get_claim(&self, content: &T::Content) -> Option<&T::Identity> {
        self.claims.get(content)
    }

    pub fn create_claim(&mut self, claim: T::Content, identity: T::Identity) -> DispatchResult {
        match self.claims.get(&claim) {
            Some(_) => Err("Claim Exists!"),
            None => {
                self.claims.insert(claim, identity);
                Ok(())
            }
        }
    }

    pub fn revoke_claim(&mut self, caller: T::Identity, claim: &T::Content) -> DispatchResult {
        match self.claims.get(claim) {
            Some(owner) => {
                if owner == &caller {
                    self.claims.remove(claim);
                    Ok(())
                } else {
                    Err("Not the owner!")
                }
            }
            None => Err("Claim Not Found!"),
        }
    }
}

pub enum Call<T: Config> {
    CreateClaim { content: T::Content },
    RevokeClaim { claim: T::Content },
}

impl<T: Config> crate::support::Dispatch for Pallet<T> {
    type Caller = T::Identity;
    type Call = Call<T>;

    fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> DispatchResult {
        match call {
            Call::CreateClaim { content } => self.create_claim(content, caller),
            Call::RevokeClaim { claim } => self.revoke_claim(caller, &claim),
        }
    }
}
