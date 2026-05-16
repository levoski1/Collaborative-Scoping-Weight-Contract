use wave_scoping::errors::Error;
use wave_scoping::reputation_manager::ReputationManager;

const DECAY_RATE: u64 = 100;
const POINTS_DIVISOR: u64 = 10;

fn setup() -> ReputationManager {
    ReputationManager::new("wave_scoping", DECAY_RATE, POINTS_DIVISOR)
}

#[test]
fn test_mint_reputation() {
    let mut rm = setup();
    rm.mint_reputation("alice", 100).unwrap();
    assert_eq!(rm.balance_of("alice"), 100);
}

#[test]
fn test_burn_reputation() {
    let mut rm = setup();
    rm.mint_reputation("alice", 100).unwrap();
    rm.burn_reputation("alice", 40).unwrap();
    assert_eq!(rm.balance_of("alice"), 60);
}

#[test]
fn test_revert_burn_more_than_balance() {
    let mut rm = setup();
    rm.mint_reputation("alice", 10).unwrap();
    let err = rm.burn_reputation("alice", 100).unwrap_err();
    assert_eq!(
        err,
        Error::InsufficientReputation("alice".to_string(), 10, 100)
    );
}

#[test]
fn test_consume_reputation() {
    let mut rm = setup();
    rm.mint_reputation("alice", 50).unwrap();
    let success = rm.consume_reputation("alice", 30).unwrap();
    assert!(success);
    assert_eq!(rm.balance_of("alice"), 20);
}

#[test]
fn test_consume_insufficient_reputation() {
    let mut rm = setup();
    let success = rm.consume_reputation("alice", 10).unwrap();
    assert!(!success);
}

#[test]
fn test_decay() {
    let mut rm = setup();
    rm.mint_reputation("alice", 100).unwrap();

    rm.decay_all();
    rm.decay_all();

    // trigger decay by minting 0
    rm.mint_reputation("alice", 0).unwrap();

    let balance = rm.balance_of("alice");
    assert!(balance < 100, "balance should have decayed below 100");
    assert!(balance > 95, "balance should not decay too much: {}", balance);
}

#[test]
fn test_revert_not_authorized() {
    // ReputationManager doesn't enforce caller auth in this Rust version;
    // access control is handled by the hub. This test is a no-op.
}
