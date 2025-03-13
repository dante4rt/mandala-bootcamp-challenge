use crate::system::SystemConfig;
use num::traits::{CheckedAdd, CheckedSub, Zero};
use std::collections::HashMap;

pub trait StakingConfig: SystemConfig {
    // Define the Balance type with ability to perform checked arithmetic operations
    type Balance: Zero + CheckedSub + CheckedAdd + Copy + PartialOrd;
}

pub struct StakingPallet<T: StakingConfig> {
    // Track free balances for each account
    pub free_balances: HashMap<T::AccountId, T::Balance>,
    // Track staked balances for each account
    pub staked_balances: HashMap<T::AccountId, T::Balance>,
}

impl<T: StakingConfig> StakingPallet<T> {
    pub fn new() -> Self {
        Self {
            free_balances: HashMap::new(),
            staked_balances: HashMap::new(),
        }
    }

    // Set free balance for an account
    pub fn set_balance(&mut self, who: T::AccountId, amount: T::Balance) {
        self.free_balances.insert(who, amount);
    }

    // Stake tokens (move from free to staked)
    pub fn stake(&mut self, who: T::AccountId, amount: T::Balance) -> Result<(), &'static str> {
        let free_balance = *self.free_balances.get(&who).unwrap_or(&T::Balance::zero());
        if free_balance < amount {
            return Err("Not enough free balance to stake");
        }

        let new_free_balance = free_balance.checked_sub(&amount).unwrap();
        let new_staked_balance = self
            .staked_balances
            .get(&who)
            .unwrap_or(&T::Balance::zero())
            .checked_add(&amount)
            .unwrap();

        self.free_balances.insert(who.clone(), new_free_balance);
        self.staked_balances.insert(who, new_staked_balance);

        Ok(())
    }

    // Unstake tokens (move from staked to free)
    pub fn unstake(&mut self, who: T::AccountId, amount: T::Balance) -> Result<(), &'static str> {
        let staked_balance = *self
            .staked_balances
            .get(&who)
            .unwrap_or(&T::Balance::zero());
        if staked_balance < amount {
            return Err("Not enough staked balance to unstake");
        }

        let new_staked_balance = staked_balance.checked_sub(&amount).unwrap();
        let new_free_balance = self
            .free_balances
            .get(&who)
            .unwrap_or(&T::Balance::zero())
            .checked_add(&amount)
            .unwrap();

        self.staked_balances.insert(who.clone(), new_staked_balance);
        self.free_balances.insert(who, new_free_balance);

        Ok(())
    }

    // Get free balance for an account
    pub fn get_free_balance(&self, who: T::AccountId) -> T::Balance {
        *self.free_balances.get(&who).unwrap_or(&T::Balance::zero())
    }

    // Get staked balance for an account
    pub fn get_staked_balance(&self, who: T::AccountId) -> T::Balance {
        *self
            .staked_balances
            .get(&who)
            .unwrap_or(&T::Balance::zero())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Runtime;

    #[test]
    fn test_staking_should_work() {
        let alice = 1u64;
        let mut staking = StakingPallet::<Runtime>::new();

        // Set initial balance
        staking.set_balance(alice, 1000);

        // Check free balance
        assert_eq!(staking.get_free_balance(alice), 1000u64);
        assert_eq!(staking.get_staked_balance(alice), 0u64);

        // Stake tokens
        let result = staking.stake(alice, 400);
        assert!(result.is_ok());

        // Check balances after staking
        assert_eq!(staking.get_free_balance(alice), 600u64);
        assert_eq!(staking.get_staked_balance(alice), 400u64);

        // Unstake tokens
        let result = staking.unstake(alice, 100);
        assert!(result.is_ok());

        // Check balances after unstaking
        assert_eq!(staking.get_free_balance(alice), 700u64);
        assert_eq!(staking.get_staked_balance(alice), 300u64);
    }

    #[test]
    fn test_staking_errors() {
        let bob = 2u64;
        let mut staking = StakingPallet::<Runtime>::new();

        // Set initial balance
        staking.set_balance(bob, 500);

        // Try to stake more than available
        let result = staking.stake(bob, 600);
        assert!(result.is_err());

        // Stake valid amount
        let result = staking.stake(bob, 300);
        assert!(result.is_ok());

        // Try to unstake more than staked
        let result = staking.unstake(bob, 400);
        assert!(result.is_err());
    }
}
