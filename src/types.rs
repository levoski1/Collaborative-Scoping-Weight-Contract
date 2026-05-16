#[derive(Debug, Clone)]
pub struct Issue {
    pub github_issue_url: String,
    pub current_weight: u64,
    pub assigned_points: u64,
    pub exists: bool,
    pub is_emergency: bool,
    pub assigned_contributor: Option<String>,
    pub started_at_block: u64,
}

impl Issue {
    pub fn new(url: &str) -> Self {
        Self {
            github_issue_url: url.to_string(),
            current_weight: 0,
            assigned_points: 0,
            exists: true,
            is_emergency: false,
            assigned_contributor: None,
            started_at_block: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct WaveData {
    pub id: u64,
    pub name: String,
    pub start_block: u64,
    pub end_block: u64,
    pub is_active: bool,
    pub is_finalized: bool,
    pub total_weight_cast: u64,
    pub issue_count: u64,
}

#[derive(Debug, Clone)]
pub struct WaveConfig {
    pub voting_period_blocks: u64,
    pub emergency_timelock: u64,
    pub max_weight_per_vote: u64,
    pub points_divisor: u64,
}

#[derive(Debug, Clone)]
pub struct FastTrackProposal {
    pub issue_url: String,
    pub proposer: String,
    pub proposed_at: u64,
    pub executed: bool,
    pub reason: String,
}

#[derive(Debug, Clone)]
pub struct SlashRecord {
    pub maintainer: String,
    pub issue_url: String,
    pub penalty: u64,
    pub timestamp: u64,
    pub applied: bool,
}

#[derive(Debug)]
pub enum Event {
    IssueRegistered { url: String, wave_id: u64 },
    Voted { voter: String, url: String, weight: u64 },
    IssueFastTracked { url: String, maintainer: String },
    ContributionStarted { url: String, contributor: String },
    ReputationEarned { voter: String, amount: u64 },
    SlashApplied { maintainer: String, penalty: u64 },
    WaveCreated { id: u64, name: String, end_block: u64 },
    WaveFinalized { id: u64 },
    PointsAdjusted { url: String, old_points: u64, new_points: u64 },
    ReputationMinted { user: String, amount: u64 },
    ReputationBurned { user: String, amount: u64 },
    ReputationDecayed { user: String, new_balance: u64 },
    EpochAdvanced { epoch: u64 },
    FastTrackProposed { url: String, proposer: String, reason: String },
    FastTrackExecuted { url: String, timestamp: u64 },
    MaintainerSlashed { maintainer: String, url: String, penalty: u64, reason: String },
    SlashSettled { maintainer: String, amount: u64 },
    MaxPenaltyUpdated { old_max: u64, new_max: u64 },
}
