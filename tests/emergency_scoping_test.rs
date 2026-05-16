use wave_scoping::emergency_scoping::EmergencyScoping;
use wave_scoping::errors::Error;

const EMERGENCY_TIMELOCK: u64 = 5;

fn setup() -> EmergencyScoping {
    EmergencyScoping::new("wave_scoping", EMERGENCY_TIMELOCK)
}

const ISSUE_URL_1: &str = "https://github.com/owner/repo/issues/1";
const ISSUE_URL_2: &str = "https://github.com/owner/repo/issues/2";
const EMERGENCY_REASON: &str = "Critical security vulnerability in auth module";

#[test]
fn test_propose_fast_track() {
    let mut es = setup();
    es.propose_fast_track(ISSUE_URL_1, EMERGENCY_REASON, "owner", "owner", 0)
        .unwrap();
    assert_eq!(es.proposal_count(), 1);
}

#[test]
fn test_execute_fast_track() {
    let mut es = setup();
    es.propose_fast_track(ISSUE_URL_1, EMERGENCY_REASON, "owner", "owner", 0)
        .unwrap();
    // advance blocks past timelock
    es.execute_fast_track(ISSUE_URL_1, "owner", "owner", EMERGENCY_TIMELOCK + 1)
        .unwrap();

    let prop = es.get_proposal(ISSUE_URL_1).unwrap();
    assert!(prop.executed);
}

#[test]
fn test_revert_double_propose() {
    let mut es = setup();
    es.propose_fast_track(ISSUE_URL_1, EMERGENCY_REASON, "owner", "owner", 0)
        .unwrap();
    let err = es
        .propose_fast_track(ISSUE_URL_1, EMERGENCY_REASON, "owner", "owner", 0)
        .unwrap_err();
    assert_eq!(err, Error::AlreadyProposed);
}

#[test]
fn test_revert_timelock_not_met() {
    let mut es = setup();
    es.propose_fast_track(ISSUE_URL_1, EMERGENCY_REASON, "owner", "owner", 0)
        .unwrap();
    let err = es
        .execute_fast_track(ISSUE_URL_1, "owner", "owner", EMERGENCY_TIMELOCK - 1)
        .unwrap_err();
    assert_eq!(err, Error::TimelockNotMet);
}

#[test]
fn test_revert_not_owner() {
    let mut es = setup();
    let err = es
        .propose_fast_track(ISSUE_URL_1, EMERGENCY_REASON, "owner", "alice", 0)
        .unwrap_err();
    assert_eq!(err, Error::NotAuthorized);
}

#[test]
fn test_get_proposal_count() {
    let mut es = setup();
    assert_eq!(es.proposal_count(), 0);

    es.propose_fast_track(ISSUE_URL_1, EMERGENCY_REASON, "owner", "owner", 0)
        .unwrap();
    assert_eq!(es.proposal_count(), 1);

    es.propose_fast_track(ISSUE_URL_2, "Another reason", "owner", "owner", 0)
        .unwrap();
    assert_eq!(es.proposal_count(), 2);
}
