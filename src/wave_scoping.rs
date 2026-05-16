use std::collections::HashMap;

use crate::emergency_scoping::EmergencyScoping;
use crate::errors::Error;
use crate::math;
use crate::reputation_manager::ReputationManager;
use crate::slashing_manager::SlashingManager;
use crate::types::{Event, Issue, WaveConfig, WaveData};
use crate::wave;

pub struct WaveScoping {
    pub owner: String,
    pub reputation_manager: ReputationManager,
    pub emergency_scoping: EmergencyScoping,
    pub slashing_manager: SlashingManager,

    pub wave_config: WaveConfig,

    registry: HashMap<String, Issue>,
    votes_by: HashMap<String, HashMap<String, u64>>,
    waves: HashMap<u64, WaveData>,
    issue_to_wave: HashMap<String, u64>,

    pub current_wave_id: u64,
    pub total_issues: u64,
    pub total_votes: u64,
    pub block_number: u64,
    pub events: Vec<Event>,
}

impl WaveScoping {
    pub fn new(
        owner: &str,
        voting_period_blocks: u64,
        emergency_timelock: u64,
        max_weight_per_vote: u64,
        points_divisor: u64,
        decay_rate_bps: u64,
        slash_max_bps: u64,
        slash_burn_rate_bps: u64,
    ) -> Result<Self, Error> {
        let wave_config = WaveConfig {
            voting_period_blocks,
            emergency_timelock,
            max_weight_per_vote,
            points_divisor,
        };

        let hub_id = format!("hub:{}", owner);

        Ok(Self {
            owner: owner.to_string(),
            reputation_manager: ReputationManager::new(&hub_id, decay_rate_bps, points_divisor),
            emergency_scoping: EmergencyScoping::new(&hub_id, emergency_timelock),
            slashing_manager: SlashingManager::new(&hub_id, slash_max_bps, slash_burn_rate_bps)?,
            wave_config,
            registry: HashMap::new(),
            votes_by: HashMap::new(),
            waves: HashMap::new(),
            issue_to_wave: HashMap::new(),
            current_wave_id: 0,
            total_issues: 0,
            total_votes: 0,
            block_number: 0,
            events: Vec::new(),
        })
    }

    pub fn set_owner(&mut self, caller: &str, new_owner: &str) -> Result<(), Error> {
        if caller != self.owner {
            return Err(Error::NotOwner);
        }
        if new_owner.is_empty() {
            return Err(Error::ZeroAddress);
        }
        self.owner = new_owner.to_string();
        Ok(())
    }

    pub fn create_wave(&mut self, caller: &str, name: &str, duration_blocks: u64) -> Result<u64, Error> {
        if caller != self.owner {
            return Err(Error::NotOwner);
        }

        let wave_id = wave::encode_wave_id(name, self.block_number);
        let wave = WaveData {
            id: wave_id,
            name: name.to_string(),
            start_block: self.block_number,
            end_block: self.block_number + duration_blocks,
            is_active: true,
            is_finalized: false,
            total_weight_cast: 0,
            issue_count: 0,
        };
        self.waves.insert(wave_id, wave);
        self.current_wave_id = wave_id;
        self.events.push(Event::WaveCreated {
            id: wave_id,
            name: name.to_string(),
            end_block: self.block_number + duration_blocks,
        });
        Ok(wave_id)
    }

    pub fn register_issue(&mut self, caller: &str, url: &str) -> Result<(), Error> {
        if caller != self.owner {
            return Err(Error::NotOwner);
        }
        if self.registry.contains_key(url) {
            return Err(Error::AlreadyRegistered);
        }

        let issue = Issue::new(url);
        self.registry.insert(url.to_string(), issue);

        self.issue_to_wave
            .insert(url.to_string(), self.current_wave_id);
        if let Some(wave) = self.waves.get_mut(&self.current_wave_id) {
            wave.issue_count += 1;
        }
        self.total_issues += 1;

        self.events.push(Event::IssueRegistered {
            url: url.to_string(),
            wave_id: self.current_wave_id,
        });
        Ok(())
    }

    pub fn vote_on_issue(&mut self, caller: &str, url: &str, weight: u64) -> Result<(), Error> {
        if weight == 0 || weight > self.wave_config.max_weight_per_vote {
            return Err(Error::InvalidWeight);
        }

        let exists = self
            .registry
            .get(url)
            .map(|i| i.exists)
            .unwrap_or(false);
        if !exists {
            return Err(Error::NotRegistered);
        }

        let issue_wave_id = *self
            .issue_to_wave
            .get(url)
            .ok_or(Error::NotRegistered)?;
        let wave_active = self
            .waves
            .get(&issue_wave_id)
            .map(|w| wave::is_within_voting_period(w, &self.wave_config, self.block_number))
            .unwrap_or(false);
        if !wave_active {
            return Err(Error::WaveNotActive);
        }

        if let Some(voter_votes) = self.votes_by.get(caller) {
            if voter_votes.contains_key(url) {
                return Err(Error::AlreadyVoted);
            }
        }

        let reputation_balance = self.reputation_manager.balance_of(caller);
        let reputation_weight = self.reputation_manager.reputation_to_weight(reputation_balance);
        let mut effective_weight =
            math::calculate_weighted_score(weight, reputation_weight, self.wave_config.points_divisor);
        effective_weight = math::min(effective_weight, self.wave_config.max_weight_per_vote);

        self.votes_by
            .entry(caller.to_string())
            .or_default()
            .insert(url.to_string(), effective_weight);

        if let Some(issue) = self.registry.get_mut(url) {
            issue.current_weight += effective_weight;
            issue.assigned_points = math::calculate_points(issue.current_weight, self.wave_config.points_divisor);
        }

        if let Some(wave) = self.waves.get_mut(&issue_wave_id) {
            wave.total_weight_cast += effective_weight;
        }

        let rep_amount = weight / 10 + 1;
        self.reputation_manager
            .mint_reputation(caller, rep_amount)?;
        self.total_votes += 1;

        while let Some(ev) = self.reputation_manager.events.pop() {
            self.events.push(ev);
        }

        self.events.push(Event::Voted {
            voter: caller.to_string(),
            url: url.to_string(),
            weight: effective_weight,
        });
        self.events.push(Event::ReputationEarned {
            voter: caller.to_string(),
            amount: rep_amount,
        });
        Ok(())
    }

    pub fn fast_track_issue(&mut self, caller: &str, url: &str, reason: &str) -> Result<(), Error> {
        if caller != self.owner {
            return Err(Error::NotOwner);
        }
        let exists = self
            .registry
            .get(url)
            .map(|i| i.exists)
            .unwrap_or(false);
        if !exists {
            return Err(Error::NotRegistered);
        }

        self.emergency_scoping
            .propose_fast_track(url, reason, &self.owner, caller, self.block_number)?;
        // Advance blocks past timelock so execution succeeds
        self.block_number += self.wave_config.emergency_timelock + 1;
        self.emergency_scoping
            .execute_fast_track(url, &self.owner, caller, self.block_number)?;

        while let Some(ev) = self.emergency_scoping.events.pop() {
            self.events.push(ev);
        }

        if let Some(issue) = self.registry.get_mut(url) {
            issue.is_emergency = true;
            issue.current_weight += self.wave_config.max_weight_per_vote * 2;
        }

        self.events.push(Event::IssueFastTracked {
            url: url.to_string(),
            maintainer: caller.to_string(),
        });
        Ok(())
    }

    pub fn start_work(&mut self, caller: &str, url: &str) -> Result<(), Error> {
        let already_assigned = self
            .registry
            .get(url)
            .map(|i| i.assigned_contributor.is_some())
            .unwrap_or(false);
        if already_assigned {
            return Err(Error::PointsAlreadyAssigned);
        }

        if !self.registry.contains_key(url) {
            return Err(Error::NotRegistered);
        }

        if let Some(issue) = self.registry.get_mut(url) {
            issue.assigned_contributor = Some(caller.to_string());
            issue.started_at_block = self.block_number;
        }

        self.events.push(Event::ContributionStarted {
            url: url.to_string(),
            contributor: caller.to_string(),
        });
        Ok(())
    }

    pub fn finalize_wave(&mut self, caller: &str, wave_id: u64) -> Result<(), Error> {
        if caller != self.owner {
            return Err(Error::NotOwner);
        }
        let wave = self
            .waves
            .get_mut(&wave_id)
            .ok_or(Error::WaveNotActive)?;
        if !wave.is_active {
            return Err(Error::WaveNotActive);
        }
        if wave.is_finalized {
            return Err(Error::WaveAlreadyFinalized);
        }

        wave.is_finalized = true;
        self.events
            .push(Event::WaveFinalized { id: wave_id });
        Ok(())
    }

    pub fn adjust_points(
        &mut self,
        caller: &str,
        url: &str,
        new_points: u64,
        reason: &str,
    ) -> Result<(), Error> {
        if caller != self.owner {
            return Err(Error::NotOwner);
        }

        let issue_info = self
            .registry
            .get(url)
            .filter(|i| i.exists)
            .map(|i| (i.assigned_points, i.assigned_contributor.is_some(), i.started_at_block))
            .ok_or(Error::NotRegistered)?;

        let (previous_points, has_contributor, started_at) = issue_info;
        let within_grace = has_contributor
            && self.block_number <= started_at + 5;

        if !has_contributor || within_grace {
            if let Some(issue) = self.registry.get_mut(url) {
                issue.assigned_points = new_points;
            }
        } else {
            self.slashing_manager.slash(
                caller,
                url,
                previous_points,
                new_points,
                reason,
            )?;
            while let Some(ev) = self.slashing_manager.events.pop() {
                self.events.push(ev);
            }
            if let Some(issue) = self.registry.get_mut(url) {
                issue.assigned_points = new_points;
            }
        }

        self.events.push(Event::PointsAdjusted {
            url: url.to_string(),
            old_points: previous_points,
            new_points,
        });
        Ok(())
    }

    pub fn get_issue(&self, url: &str) -> Result<(u64, u64, bool, Option<String>), Error> {
        let issue = self.registry.get(url).ok_or(Error::NotRegistered)?;
        if !issue.exists {
            return Err(Error::NotRegistered);
        }
        Ok((
            issue.current_weight,
            issue.assigned_points,
            issue.is_emergency,
            issue.assigned_contributor.clone(),
        ))
    }

    pub fn get_wave(&self, wave_id: u64) -> Option<&WaveData> {
        self.waves.get(&wave_id)
    }

    pub fn get_current_wave(&self) -> Option<&WaveData> {
        self.waves.get(&self.current_wave_id)
    }

    pub fn get_wave_config(&self) -> &WaveConfig {
        &self.wave_config
    }

    pub fn get_voter_weight(&self, voter: &str, url: &str) -> u64 {
        self.votes_by
            .get(voter)
            .and_then(|v| v.get(url))
            .copied()
            .unwrap_or(0)
    }

    pub fn advance_blocks(&mut self, n: u64) {
        self.block_number += n;
    }
}
