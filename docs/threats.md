# Threat Model & Security Analysis

## Asset Classification

| Asset | Sensitivity | Description |
|-------|-------------|-------------|
| Ownership | Critical | Controls issue registration, point adjustment, wave creation |
| Reputation balances | High | Determines voting weight |
| Issue state | Medium | Affects contributor assignments and point values |
| Wave state | Medium | Governs voting periods and finalization |

## Threat Scenarios

### T1: Malicious Owner (Most Critical)
**Risk:** The owner can register spam issues, arbitrarily adjust points, and fast-track without oversight.
**Mitigation:**
- `setOwner()` allows transfer (e.g., to a multisig or timelock-controlled contract).
- `adjustPoints` is guarded by slashing when work has started.
- Recommended: deploy with a governance multisig as owner (e.g., Gnosis Safe).

### T2: Sybil Voting
**Risk:** An attacker creates many addresses to inflate vote weight.
**Mitigation:**
- Reputation system ties weight to past contributions.
- One vote per address per issue (cannot re-vote).
- Reputation decay reduces power of dormant accounts.

### T3: Emergency Fast-Track Abuse
**Risk:** Owner fast-tracks low-priority issues to bypass voting.
**Mitigation:**
- `EmergencyScoping` enforces a timelock (configurable at construction).
- The event `IssueFastTracked` is public, allowing off-chain monitoring.

### T4: Point Manipulation After Work Starts
**Risk:** Owner reduces a contributor's points after they've begun work.
**Mitigation:**
- `SlashingManager` calculates penalty proportional to the point difference.
- The slashing only applies when work has started and the grace window has passed.
- 50% of slashed penalty is burned (configurable `burnRateBps`), creating disincentive.

### T5: Front-Running Votes
**Risk:** A watcher sees a pending vote and submits their own first.
**Mitigation:**
- Since each address can only vote once per issue, front-running only affects order, not outcome.
- Weighted scoring means early and late votes have equal influence.

### T6: Reentrancy
**Risk:** A malicious contract calls back into WaveScoping during a vote.
**Mitigation:**
- No external calls are made during state-changing operations (Checks-Effects-Interactions pattern).
- `mintReputation` is an internal-only call to a trusted module.

## Parameter Safety Bounds

| Parameter | Safe Range | Rationale |
|-----------|------------|-----------|
| `maxWeightPerVote` | 1–1000 | Linear scaling prevents extreme outliers |
| `pointsDivisor` | 1–100 | Controls granularity of point assignment |
| `decayRateBps` | 0–5000 | 0%–50% decay per epoch |
| `slashMaxBps` | 0–10000 | 0%–100% of point diff |
| `emergencyTimelock` | 1–100 | Blocks until execution allowed |
