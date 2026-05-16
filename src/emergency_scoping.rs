use std::collections::HashMap;

use crate::errors::Error;
use crate::types::{Event, FastTrackProposal};

pub struct EmergencyScoping {
    pub wave_scoping: String,
    pub timelock_blocks: u64,
    proposals: HashMap<String, FastTrackProposal>,
    proposal_list: Vec<String>,
    pub events: Vec<Event>,
}

impl EmergencyScoping {
    pub fn new(wave_scoping: &str, timelock_blocks: u64) -> Self {
        Self {
            wave_scoping: wave_scoping.to_string(),
            timelock_blocks,
            proposals: HashMap::new(),
            proposal_list: Vec::new(),
            events: Vec::new(),
        }
    }

    pub fn propose_fast_track(
        &mut self,
        url: &str,
        reason: &str,
        owner: &str,
        caller: &str,
        block_number: u64,
    ) -> Result<(), Error> {
        if caller != owner {
            return Err(Error::NotAuthorized);
        }
        if self.proposals.contains_key(url) {
            return Err(Error::AlreadyProposed);
        }
        if reason.is_empty() {
            return Err(Error::EmptyReason);
        }

        self.proposals.insert(
            url.to_string(),
            FastTrackProposal {
                issue_url: url.to_string(),
                proposer: caller.to_string(),
                proposed_at: block_number,
                executed: false,
                reason: reason.to_string(),
            },
        );
        self.proposal_list.push(url.to_string());

        self.events.push(Event::FastTrackProposed {
            url: url.to_string(),
            proposer: caller.to_string(),
            reason: reason.to_string(),
        });
        Ok(())
    }

    pub fn execute_fast_track(
        &mut self,
        url: &str,
        owner: &str,
        caller: &str,
        block_number: u64,
    ) -> Result<(), Error> {
        if caller != owner {
            return Err(Error::NotAuthorized);
        }
        let prop = self
            .proposals
            .get(url)
            .ok_or(Error::NotAuthorized)?;
        if prop.executed {
            return Err(Error::AlreadyExecuted);
        }
        if block_number < prop.proposed_at + self.timelock_blocks {
            return Err(Error::TimelockNotMet);
        }

        if let Some(p) = self.proposals.get_mut(url) {
            p.executed = true;
        }

        self.events.push(Event::FastTrackExecuted {
            url: url.to_string(),
            timestamp: block_number,
        });
        Ok(())
    }

    pub fn get_proposal(&self, url: &str) -> Option<&FastTrackProposal> {
        self.proposals.get(url)
    }

    pub fn proposal_count(&self) -> usize {
        self.proposal_list.len()
    }

    pub fn is_fast_tracked(&self, url: &str) -> bool {
        self.proposals
            .get(url)
            .map(|p| p.executed)
            .unwrap_or(false)
    }
}
