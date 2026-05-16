pub const BASIS_POINTS: u64 = 10_000;

pub fn min(a: u64, b: u64) -> u64 {
    if a < b { a } else { b }
}

pub fn max(a: u64, b: u64) -> u64 {
    if a > b { a } else { b }
}

pub fn apply_basis_points(value: u64, bp: u64) -> u64 {
    (value * bp) / BASIS_POINTS
}

pub fn calculate_weighted_score(base_weight: u64, reputation_bonus: u64, reputation_divisor: u64) -> u64 {
    base_weight + (reputation_bonus / reputation_divisor)
}

pub fn calculate_points(weight: u64, divisor: u64) -> u64 {
    weight / divisor
}

pub fn decay(value: u64, decay_basis_points: u64) -> u64 {
    let retained = BASIS_POINTS - decay_basis_points;
    (value * retained) / BASIS_POINTS
}

pub fn is_in_range(value: u64, min_v: u64, max_v: u64) -> bool {
    value >= min_v && value <= max_v
}
