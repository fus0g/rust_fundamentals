use std::collections::BTreeMap;

use num::{CheckedAdd, One, Zero};

pub trait SystemConfig {
    type BlockNumber: Ord + Clone + Zero + One + Copy + CheckedAdd;
    type Identity: Clone + Ord;
    type NonceCount: Ord + Clone + Zero + One + Copy + CheckedAdd;
}

#[derive(Debug)]
pub struct Pallets<T: SystemConfig> {
    block_number: T::BlockNumber,
    nonce: BTreeMap<T::Identity, T::NonceCount>,
}

impl<T: SystemConfig> Pallets<T> {
    pub fn new() -> Self {
        Self {
            block_number: T::BlockNumber::zero(),
            nonce: BTreeMap::new(),
        }
    }

    pub fn block_number(&self) -> T::BlockNumber {
        self.block_number
    }

    pub fn increment_block_number(&mut self) {
        self.block_number = self
            .block_number
            .checked_add(&T::BlockNumber::one())
            .unwrap()
    }

    pub fn increment_nonce(&mut self, who: T::Identity) {
        let nonce = *self.nonce.get(&who).unwrap_or(&T::NonceCount::zero());
        let new_nonce = nonce.checked_add(&T::NonceCount::one()).unwrap();
        self.nonce.insert(who.clone(), new_nonce);
    }

    pub fn get_nonce(&self, who: T::Identity) -> T::NonceCount {
        *self.nonce.get(&who).unwrap_or(&T::NonceCount::zero())
    }
}

mod tests {
    use crate::system::SystemConfig;
    use std::collections::BTreeMap;

    struct Config;

    impl SystemConfig for Config {
        type Identity = String;
        type BlockNumber = u32;
        type NonceCount = u32;
    }

    #[test]
    fn init_system() {
        let system: super::Pallets<Config> = super::Pallets::new();

        assert_eq!(system.block_number(), 0);
        assert_eq!(system.nonce, BTreeMap::new());
    }

    #[test]
    fn increment_system() {
        let mut system: super::Pallets<Config> = super::Pallets::new();
        system.increment_block_number();
        assert_eq!(system.block_number, 1);
        system.increment_nonce("ferrio".to_string());
        assert_eq!(system.get_nonce("ferrio".to_string()), 1);
    }
}
