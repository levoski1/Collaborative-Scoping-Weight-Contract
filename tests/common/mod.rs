use wave_scoping::wave_scoping::WaveScoping;

pub const VOTING_PERIOD: u64 = 1000;
pub const EMERGENCY_TIMELOCK: u64 = 5;
pub const MAX_WEIGHT: u64 = 100;
pub const POINTS_DIVISOR: u64 = 10;
pub const DECAY_RATE: u64 = 100;
pub const SLASH_MAX_BPS: u64 = 2000;
pub const SLASH_BURN_RATE: u64 = 5000;

pub fn setup() -> WaveScoping {
    WaveScoping::new(
        "owner",
        VOTING_PERIOD,
        EMERGENCY_TIMELOCK,
        MAX_WEIGHT,
        POINTS_DIVISOR,
        DECAY_RATE,
        SLASH_MAX_BPS,
        SLASH_BURN_RATE,
    )
    .unwrap()
}

pub fn setup_with_wave() -> WaveScoping {
    let mut hub = setup();
    hub.create_wave("owner", "Sprint 2026.1", VOTING_PERIOD)
        .unwrap();
    hub
}
