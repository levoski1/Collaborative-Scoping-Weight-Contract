# API Reference

## WaveScoping (Hub)

### State-Changing Functions

#### `WaveScoping::new(owner, voting_period_blocks, emergency_timelock, max_weight_per_vote, points_divisor, decay_rate_bps, slash_max_bps, slash_burn_rate_bps)`
Creates a new WaveScoping instance with all three module managers.

#### `set_owner(caller, new_owner)`
Transfers ownership. OnlyOwner.

#### `create_wave(caller, name, duration_blocks)` → `u64`
Creates a new wave. Returns the wave ID (hash of name+block). OnlyOwner.

#### `register_issue(caller, url)`
Registers an issue for the current wave. Reverts if already registered. OnlyOwner.

#### `vote_on_issue(caller, url, weight)`
Casts a weighted vote. Each address gets one vote per issue. Weight is boosted by voter's reputation balance. Multi-call reverts.

#### `fast_track_issue(caller, url, reason)`
Proposes and executes an emergency fast-track, doubling the max weight on the issue. OnlyOwner.

#### `start_work(caller, url)`
Marks an issue as having work started by the caller. Prevents later point changes without slashing.

#### `finalize_wave(caller, wave_id)`
Locks a wave, preventing further voting. OnlyOwner.

#### `adjust_points(caller, url, new_points, reason)`
Adjusts issue points. If work has started beyond the grace window, the caller is slashed. OnlyOwner.

### View Functions

| Function | Returns | Description |
|----------|---------|-------------|
| `get_issue(url)` | `(weight, points, is_emergency, contributor)` | Full issue state |
| `get_wave(wave_id)` | `WaveData` | Full wave state |
| `get_current_wave()` | `WaveData` | Active wave |
| `get_wave_config()` | `WaveConfig` | Contract parameters |
| `get_voter_weight(voter, url)` | `u64` | Vote weight cast by user |

## ReputationManager

| Function | Description |
|----------|-------------|
| `balance_of(user)` | Returns reputation balance |
| `mint_reputation(user, amount)` | Called by hub |
| `burn_reputation(user, amount)` | Called by hub. Reverts if insufficient |
| `consume_reputation(user, amount)` | Called by hub. Returns bool |
| `decay_all()` | Advances epoch, decaying all balances |

## EmergencyScoping

| Function | Description |
|----------|-------------|
| `propose_fast_track(url, reason, owner, caller, block_number)` | Creates a proposal. OnlyOwner |
| `execute_fast_track(url, owner, caller, block_number)` | Executes after timelock. OnlyOwner |
| `get_proposal(url)` | Returns proposal details |
| `is_fast_tracked(url)` | Returns whether executed |

## SlashingManager

| Function | Description |
|----------|-------------|
| `slash(maintainer, url, prev_points, new_points, reason)` | Records a slash. Called by hub |
| `get_slash_record(maintainer, url)` | Returns slash record |
| `total_slashed(maintainer)` | Aggregated penalty for maintainer |

## Events

| Event | Parameters |
|-------|------------|
| `IssueRegistered` | `url`, `wave_id` |
| `Voted` | `voter`, `url`, `weight` |
| `IssueFastTracked` | `url`, `maintainer` |
| `ContributionStarted` | `url`, `contributor` |
| `ReputationEarned` | `voter`, `amount` |
| `SlashApplied` | `maintainer`, `penalty` |
| `WaveCreated` | `id`, `name`, `end_block` |
| `WaveFinalized` | `id` |
| `PointsAdjusted` | `url`, `old_points`, `new_points` |
