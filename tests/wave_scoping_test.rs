mod common;

use wave_scoping::errors::Error;

const ISSUE_URL_1: &str = "https://github.com/owner/repo/issues/1";
const VOTING_PERIOD: u64 = 1000;

#[test]
fn test_constructor() {
    let hub = common::setup();
    assert_eq!(hub.owner, "owner");
    let wave = hub.get_current_wave();
    assert!(wave.is_none());
}

#[test]
fn test_register_issue() {
    let mut hub = common::setup_with_wave();
    hub.register_issue("owner", ISSUE_URL_1).unwrap();
    let (weight, points, emergency, contributor) = hub.get_issue(ISSUE_URL_1).unwrap();
    assert_eq!(weight, 0);
    assert_eq!(points, 0);
    assert!(!emergency);
    assert_eq!(contributor, None);
}

#[test]
fn test_revert_double_register() {
    let mut hub = common::setup_with_wave();
    hub.register_issue("owner", ISSUE_URL_1).unwrap();
    let err = hub.register_issue("owner", ISSUE_URL_1).unwrap_err();
    assert_eq!(err, Error::AlreadyRegistered);
}

#[test]
fn test_vote() {
    let mut hub = common::setup_with_wave();
    hub.register_issue("owner", ISSUE_URL_1).unwrap();
    hub.vote_on_issue("alice", ISSUE_URL_1, 10).unwrap();
    let (weight, points, _, _) = hub.get_issue(ISSUE_URL_1).unwrap();
    assert_eq!(weight, 10);
    assert_eq!(points, 1);
}

#[test]
fn test_revert_double_vote() {
    let mut hub = common::setup_with_wave();
    hub.register_issue("owner", ISSUE_URL_1).unwrap();
    hub.vote_on_issue("alice", ISSUE_URL_1, 10).unwrap();
    let err = hub.vote_on_issue("alice", ISSUE_URL_1, 5).unwrap_err();
    assert_eq!(err, Error::AlreadyVoted);
}

#[test]
fn test_revert_vote_unregistered_issue() {
    let mut hub = common::setup_with_wave();
    let err = hub.vote_on_issue("alice", ISSUE_URL_1, 10).unwrap_err();
    assert_eq!(err, Error::NotRegistered);
}

#[test]
fn test_revert_invalid_weight() {
    let mut hub = common::setup_with_wave();
    hub.register_issue("owner", ISSUE_URL_1).unwrap();

    let err = hub.vote_on_issue("alice", ISSUE_URL_1, 0).unwrap_err();
    assert_eq!(err, Error::InvalidWeight);

    let err = hub.vote_on_issue("alice", ISSUE_URL_1, 101).unwrap_err();
    assert_eq!(err, Error::InvalidWeight);
}

#[test]
fn test_owner_transfer() {
    let mut hub = common::setup_with_wave();
    hub.set_owner("owner", "alice").unwrap();
    assert_eq!(hub.owner, "alice");

    hub.set_owner("alice", "owner").unwrap();
    assert_eq!(hub.owner, "owner");
}

#[test]
fn test_revert_not_owner() {
    let mut hub = common::setup_with_wave();
    let err = hub.register_issue("alice", ISSUE_URL_1).unwrap_err();
    assert_eq!(err, Error::NotOwner);
}

#[test]
fn test_start_work() {
    let mut hub = common::setup_with_wave();
    hub.register_issue("owner", ISSUE_URL_1).unwrap();

    hub.start_work("alice", ISSUE_URL_1).unwrap();

    let (_, _, _, contributor) = hub.get_issue(ISSUE_URL_1).unwrap();
    assert_eq!(contributor, Some("alice".to_string()));
}

#[test]
fn test_revert_start_work_twice() {
    let mut hub = common::setup_with_wave();
    hub.register_issue("owner", ISSUE_URL_1).unwrap();

    hub.start_work("alice", ISSUE_URL_1).unwrap();

    let err = hub.start_work("bob", ISSUE_URL_1).unwrap_err();
    assert_eq!(err, Error::PointsAlreadyAssigned);
}

#[test]
fn test_vote_out_of_period() {
    let mut hub = common::setup_with_wave();
    hub.register_issue("owner", ISSUE_URL_1).unwrap();

    hub.advance_blocks(VOTING_PERIOD + 1);

    let err = hub.vote_on_issue("alice", ISSUE_URL_1, 10).unwrap_err();
    assert_eq!(err, Error::WaveNotActive);
}

#[test]
fn test_finalize_wave() {
    let mut hub = common::setup_with_wave();
    let wave_id = hub.current_wave_id;

    hub.finalize_wave("owner", wave_id).unwrap();

    let wave = hub.get_wave(wave_id).unwrap();
    assert!(wave.is_finalized);
}

#[test]
fn test_adjust_points_without_contributor() {
    let mut hub = common::setup_with_wave();
    hub.register_issue("owner", ISSUE_URL_1).unwrap();

    hub.adjust_points("owner", ISSUE_URL_1, 50, "Initial adjustment")
        .unwrap();

    let (_, points, _, _) = hub.get_issue(ISSUE_URL_1).unwrap();
    assert_eq!(points, 50);
}

#[test]
fn test_user_cannot_register_issue() {
    let mut hub = common::setup_with_wave();
    let err = hub.register_issue("alice", ISSUE_URL_1).unwrap_err();
    assert_eq!(err, Error::NotOwner);
}
