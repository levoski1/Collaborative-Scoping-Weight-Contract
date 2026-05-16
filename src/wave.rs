use crate::types::{WaveConfig, WaveData};

pub fn is_within_voting_period(wave: &WaveData, _config: &WaveConfig, block_number: u64) -> bool {
    wave.is_active
        && !wave.is_finalized
        && block_number >= wave.start_block
        && block_number <= wave.end_block
}

pub fn has_voting_ended(wave: &WaveData, block_number: u64) -> bool {
    block_number > wave.end_block
}

pub fn validate_wave_config(config: &WaveConfig) -> bool {
    config.voting_period_blocks > 0
        && config.emergency_timelock > 0
        && config.max_weight_per_vote > 0
        && config.points_divisor > 0
}

pub fn encode_wave_id(name: &str, start_block: u64) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    name.hash(&mut hasher);
    start_block.hash(&mut hasher);
    hasher.finish()
}
