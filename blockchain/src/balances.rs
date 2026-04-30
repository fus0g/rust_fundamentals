use std::collections::BTreeMap;

use num::{CheckedAdd, CheckedSub, Zero};

pub trait BalancesConfig: crate::system::SystemConfig {
    type BalanceAmount: Zero + CheckedAdd + CheckedSub + Copy;
}

#[derive(Debug)]
pub struct Pallet<T: BalancesConfig> {
    balances: BTreeMap<T::Identity, T::BalanceAmount>,
}

impl<T: BalancesConfig> Pallet<T> {
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    pub fn set_balance(&mut self, who: &T::Identity, amount: T::BalanceAmount) {
        self.balances.insert(who.clone(), amount);
    }

    pub fn balance(&self, who: &T::Identity) -> T::BalanceAmount {
        *self.balances.get(who).unwrap_or(&T::BalanceAmount::zero())
    }

    pub fn transfer(
        &mut self,
        caller: &T::Identity,
        to: &T::Identity,
        amount: T::BalanceAmount,
    ) -> Result<(), &'static str> {
        let caller_balance = self.balance(caller);
        let to_balance = self.balance(to);
        let new_caller_balance = caller_balance
            .checked_sub(&amount)
            .ok_or("Insufficient Balance!")?;
        let new_to_balance = to_balance.checked_add(&amount).ok_or("Overflow Error!")?;

        self.set_balance(caller, new_caller_balance);
        self.set_balance(to, new_to_balance);

        Ok(())
    }
}

mod tests {
    struct TestConfig;

    impl crate::system::SystemConfig for TestConfig {
        type BlockNumber = u32;
        type NonceCount = u32;
        type Identity = String;
    }

    impl super::BalancesConfig for TestConfig {
        type BalanceAmount = u128;
    }

    #[test]
    fn init_balances() {
        let mut balances: super::Pallet<TestConfig> = super::Pallet::new();

        assert_eq!(balances.balance(&"hello".to_string()), 0);
        balances.set_balance(&"hello".to_string(), 100);
        assert_eq!(balances.balance(&"hello".to_string()), 100);
        assert_eq!(balances.balance(&"bob".to_string()), 0);
    }

    #[test]
    fn transfer_balance() {
        let mut balances: super::Pallet<TestConfig> = super::Pallet::new();
        balances.set_balance(&"hello".to_string(), 100);
        assert_eq!(
            balances.transfer(&"hello".to_string(), &"fus".to_string(), 50),
            Ok(())
        );
        assert_eq!(
            balances.transfer(&"hello".to_string(), &"fus".to_string(), 150),
            Err("Insufficient Balance!")
        );

        balances.set_balance(&"fus".to_string(), u128::MAX);
        assert_eq!(
            balances.transfer(&"hello".to_string(), &"fus".to_string(), 30),
            Err("Overflow Error!")
        );
    }
}

pub enum Call<T: BalancesConfig> {
    Transfer {
        to: T::Identity,
        amount: T::BalanceAmount,
    },
}

impl<T: BalancesConfig> crate::support::Dispatch for Pallet<T> {
    type Caller = T::Identity;
    type Call = Call<T>;

    fn dispatch(&mut self, caller: T::Identity, call: Call<T>) -> crate::support::DispatchResult {
        match call {
            Call::Transfer { to, amount } => {
                self.transfer(&caller, &to, amount)?;
                Ok(())
            }
        }
    }
}
