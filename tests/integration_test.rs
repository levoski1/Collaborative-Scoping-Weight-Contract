mod common;

const ISSUE_URL_1: &str = "https://github.com/owner/repo/issues/1";
const ISSUE_URL_2: &str = "https://github.com/owner/repo/issues/2";
const VOTING_PERIOD: u64 = 1000;
const EMERGENCY_REASON: &str = "Critical security vulnerability";
const MAX_WEIGHT: u64 = 100;

#[test]
fn test_full_wave_lifecycle() {
    let mut hub = common::setup();

    hub.create_wave("owner", "Sprint 2026.2", VOTING_PERIOD)
        .unwrap();
    let wave_id = hub.current_wave_id;

    hub.register_issue("owner", ISSUE_URL_1).unwrap();
    hub.register_issue("owner", ISSUE_URL_2).unwrap();

    hub.vote_on_issue("alice", ISSUE_URL_1, 30).unwrap();
    hub.vote_on_issue("bob", ISSUE_URL_1, 20).unwrap();
    hub.vote_on_issue("charlie", ISSUE_URL_2, 50).unwrap();

    let (w1, p1, _, _) = hub.get_issue(ISSUE_URL_1).unwrap();
    assert_eq!(w1, 50);
    assert_eq!(p1, 5);

    let (w2, p2, _, _) = hub.get_issue(ISSUE_URL_2).unwrap();
    assert_eq!(w2, 50);
    assert_eq!(p2, 5);

    hub.start_work("alice", ISSUE_URL_1).unwrap();

    hub.finalize_wave("owner", wave_id).unwrap();

    let wave = hub.get_wave(wave_id).unwrap();
    assert!(wave.is_finalized);

    assert!(hub.reputation_manager.balance_of("alice") > 0);
    assert!(hub.reputation_manager.balance_of("bob") > 0);
    assert!(hub.reputation_manager.balance_of("charlie") > 0);
}

#[test]
fn test_fast_track_during_wave() {
    let mut hub = common::setup_with_wave();
    hub.register_issue("owner", ISSUE_URL_1).unwrap();

    hub.vote_on_issue("alice", ISSUE_URL_1, 10).unwrap();

    let (weight_before, _, _, _) = hub.get_issue(ISSUE_URL_1).unwrap();

    hub.fast_track_issue("owner", ISSUE_URL_1, EMERGENCY_REASON)
        .unwrap();

    let (weight_after, _, is_emergency, _) = hub.get_issue(ISSUE_URL_1).unwrap();
    assert!(is_emergency);
    assert!(weight_after > weight_before);
}

#[test]
fn test_reputation_weighted_voting() {
    let mut hub = common::setup_with_wave();
    hub.register_issue("owner", ISSUE_URL_1).unwrap();

    hub.reputation_manager
        .mint_reputation("alice", 100)
        .unwrap();

    hub.vote_on_issue("alice", ISSUE_URL_1, 30).unwrap();

    let (weight, _, _, _) = hub.get_issue(ISSUE_URL_1).unwrap();
    assert!(weight >= 30);

    let alice_vote = hub.get_voter_weight("alice", ISSUE_URL_1);
    assert!(alice_vote >= 30);
    assert!(alice_vote <= MAX_WEIGHT);
}

#[test]
fn test_multiple_waves() {
    let mut hub = common::setup();
    let url1 = "https://github.com/owner/repo/issues/1";
    let url2 = "https://github.com/owner/repo/issues/2";

    hub.create_wave("owner", "Wave 1", VOTING_PERIOD).unwrap();
    hub.register_issue("owner", url1).unwrap();

    hub.vote_on_issue("alice", url1, 50).unwrap();

    let (w1, _, _, _) = hub.get_issue(url1).unwrap();
    assert_eq!(w1, 50);

    hub.create_wave("owner", "Wave 2", VOTING_PERIOD).unwrap();
    hub.register_issue("owner", url2).unwrap();

    hub.vote_on_issue("bob", url2, 75).unwrap();

    hub.finalize_wave("owner", hub.current_wave_id).unwrap();
}

#[test]
fn test_fast_track_edge_cases() {
    let mut hub = common::setup_with_wave();
    hub.register_issue("owner", ISSUE_URL_1).unwrap();

    // fast track without any votes
    hub.fast_track_issue("owner", ISSUE_URL_1, EMERGENCY_REASON)
        .unwrap();

    let (weight, _, is_emergency, _) = hub.get_issue(ISSUE_URL_1).unwrap();
    assert!(is_emergency);
    assert_eq!(weight, MAX_WEIGHT * 2);
}
