use wave_scoping::errors::Error;
use wave_scoping::slashing_manager::SlashingManager;

const SLASH_MAX_BPS: u64 = 2000;
const SLASH_BURN_RATE: u64 = 5000;

const ISSUE_URL_1: &str = "https://github.com/owner/repo/issues/1";
const ISSUE_URL_2: &str = "https://github.com/owner/repo/issues/2";

fn setup() -> SlashingManager {
    SlashingManager::new("wave_scoping", SLASH_MAX_BPS, SLASH_BURN_RATE).unwrap()
}

#[test]
fn test_slash() {
    let mut sm = setup();
    let penalty = sm
        .slash("maintainer", ISSUE_URL_1, 10, 50, "Unauthorized point change")
        .unwrap();
    assert!(penalty > 0);
    assert_eq!(sm.total_slashed("maintainer"), penalty);
}

#[test]
fn test_revert_duplicate_slash() {
    let mut sm = setup();
    sm.slash("maintainer", ISSUE_URL_1, 10, 50, "First slash")
        .unwrap();
    let err = sm
        .slash("maintainer", ISSUE_URL_1, 10, 50, "Second slash")
        .unwrap_err();
    assert_eq!(err, Error::DuplicateSlash);
}

#[test]
fn test_revert_no_change_detected() {
    let mut sm = setup();
    let err = sm
        .slash("maintainer", ISSUE_URL_1, 50, 30, "Decrease not allowed")
        .unwrap_err();
    assert_eq!(err, Error::NoChangeDetected);
}

#[test]
fn test_slash_records() {
    let mut sm = setup();
    sm.slash("maintainer", ISSUE_URL_1, 10, 50, "Reason")
        .unwrap();

    let record = sm.get_slash_record("maintainer", ISSUE_URL_1).unwrap();
    assert_eq!(record.maintainer, "maintainer");
    assert!(record.applied);
    assert_eq!(record.penalty, (50 - 10) * SLASH_MAX_BPS / 10000);
}

#[test]
fn test_total_slashed_aggregation() {
    let mut sm = setup();
    sm.slash("maintainer", ISSUE_URL_1, 10, 50, "First").unwrap();
    let first_penalty = sm.total_slashed("maintainer");

    sm.slash("maintainer", ISSUE_URL_2, 5, 25, "Second")
        .unwrap();
    let second_penalty = sm.total_slashed("maintainer");

    assert!(second_penalty > first_penalty);
}
