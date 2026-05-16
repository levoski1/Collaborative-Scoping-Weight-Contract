use std::collections::HashMap;

use crate::errors::Error;
use crate::math;
use crate::types::Event;

pub struct ReputationManager {
    pub wave_scoping: String,
    pub decay_rate_bps: u64,
    pub reputation_weight_divisor: u64,
    balances: HashMap<String, u64>,
    last_decay_epoch: HashMap<String, u64>,
    pub current_epoch: u64,
    pub events: Vec<Event>,
}

impl ReputationManager {
    pub fn new(wave_scoping: &str, decay_rate_bps: u64, divisor: u64) -> Self {
        Self {
            wave_scoping: wave_scoping.to_string(),
            decay_rate_bps,
            reputation_weight_divisor: divisor,
            balances: HashMap::new(),
            last_decay_epoch: HashMap::new(),
            current_epoch: 0,
            events: Vec::new(),
        }
    }

    pub fn balance_of(&self, user: &str) -> u64 {
        *self.balances.get(user).unwrap_or(&0)
    }

    pub fn mint_reputation(&mut self, user: &str, amount: u64) -> Result<(), Error> {
        if user.is_empty() {
            return Err(Error::ZeroAddress);
        }
        self.apply_decay(user);
        let balance = self.balances.entry(user.to_string()).or_insert(0);
        *balance += amount;
        self.events.push(Event::ReputationMinted {
            user: user.to_string(),
            amount,
        });
        Ok(())
    }

    pub fn burn_reputation(&mut self, user: &str, amount: u64) -> Result<(), Error> {
        if user.is_empty() {
            return Err(Error::ZeroAddress);
        }
        self.apply_decay(user);
        let balance = self.balances.entry(user.to_string()).or_insert(0);
        if *balance < amount {
            return Err(Error::InsufficientReputation(
                user.to_string(),
                *balance,
                amount,
            ));
        }
        *balance -= amount;
        self.events.push(Event::ReputationBurned {
            user: user.to_string(),
            amount,
        });
        Ok(())
    }

    pub fn consume_reputation(&mut self, user: &str, amount: u64) -> Result<bool, Error> {
        if user.is_empty() {
            return Err(Error::ZeroAddress);
        }
        self.apply_decay(user);
        let balance = self.balances.entry(user.to_string()).or_insert(0);
        if *balance < amount {
            return Ok(false);
        }
        *balance -= amount;
        self.events.push(Event::ReputationBurned {
            user: user.to_string(),
            amount,
        });
        Ok(true)
    }

    pub fn decay_all(&mut self) {
        self.current_epoch += 1;
        self.events
            .push(Event::EpochAdvanced {
                epoch: self.current_epoch,
            });
    }

    pub fn reputation_to_weight(&self, reputation: u64) -> u64 {
        reputation
    }

    fn apply_decay(&mut self, user: &str) {
        let last = *self.last_decay_epoch.get(user).unwrap_or(&0);
        let epochs_passed = self.current_epoch - last;
        if epochs_passed == 0 {
            return;
        }
        let capped = if epochs_passed > 100 { 100 } else { epochs_passed };
        let mut balance = *self.balances.get(user).unwrap_or(&0);
        for _ in 0..capped {
            balance = math::decay(balance, self.decay_rate_bps);
        }
        self.balances.insert(user.to_string(), balance);
        self.last_decay_epoch
            .insert(user.to_string(), self.current_epoch);
        self.events.push(Event::ReputationDecayed {
            user: user.to_string(),
            new_balance: balance,
        });
    }
}
