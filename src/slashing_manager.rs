use std::collections::HashMap;

use crate::errors::Error;
use crate::math;
use crate::types::{Event, SlashRecord};

pub struct SlashingManager {
    pub wave_scoping: String,
    pub max_penalty_bps: u64,
    pub burn_rate_bps: u64,
    slash_records: HashMap<String, HashMap<String, SlashRecord>>,
    total_slashed: HashMap<String, u64>,
    pub events: Vec<Event>,
}

impl SlashingManager {
    pub fn new(wave_scoping: &str, max_penalty_bps: u64, burn_rate_bps: u64) -> Result<Self, Error> {
        if max_penalty_bps > math::BASIS_POINTS {
            return Err(Error::PenaltyExceedsMax);
        }
        Ok(Self {
            wave_scoping: wave_scoping.to_string(),
            max_penalty_bps,
            burn_rate_bps,
            slash_records: HashMap::new(),
            total_slashed: HashMap::new(),
            events: Vec::new(),
        })
    }

    pub fn slash(
        &mut self,
        maintainer: &str,
        issue_url: &str,
        previous_points: u64,
        new_points: u64,
        reason: &str,
    ) -> Result<u64, Error> {
        if new_points <= previous_points {
            return Err(Error::NoChangeDetected);
        }

        let already = self
            .slash_records
            .get(maintainer)
            .and_then(|m| m.get(issue_url))
            .map(|r| r.applied)
            .unwrap_or(false);
        if already {
            return Err(Error::DuplicateSlash);
        }

        let point_diff = new_points - previous_points;
        let mut penalty = math::apply_basis_points(point_diff, self.max_penalty_bps);
        if penalty == 0 {
            penalty = 1;
        }

        let _burn_amount = math::apply_basis_points(penalty, self.burn_rate_bps);
        let _slashed_amount = penalty - _burn_amount;

        let record = SlashRecord {
            maintainer: maintainer.to_string(),
            issue_url: issue_url.to_string(),
            penalty,
            timestamp: 0, // not using block timestamp in Rust version
            applied: true,
        };

        self.slash_records
            .entry(maintainer.to_string())
            .or_default()
            .insert(issue_url.to_string(), record);

        let total = self.total_slashed.entry(maintainer.to_string()).or_insert(0);
        *total += penalty;

        self.events.push(Event::MaintainerSlashed {
            maintainer: maintainer.to_string(),
            url: issue_url.to_string(),
            penalty,
            reason: reason.to_string(),
        });

        Ok(penalty)
    }

    pub fn get_slash_record(&self, maintainer: &str, issue_url: &str) -> Option<&SlashRecord> {
        self.slash_records
            .get(maintainer)
            .and_then(|m| m.get(issue_url))
    }

    pub fn total_slashed(&self, maintainer: &str) -> u64 {
        *self.total_slashed.get(maintainer).unwrap_or(&0)
    }
}
