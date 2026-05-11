# API Reference

## WaveScoping (Hub Contract)

### State-Changing Functions

#### `constructor(uint256 _votingPeriodBlocks, uint256 _emergencyTimelock, uint256 _maxWeightPerVote, uint256 _pointsDivisor, uint256 _decayRateBps, uint256 _slashMaxBps, uint256 _slashBurnRateBps)`
Deploys the hub contract and all three module contracts atomically.

#### `setOwner(address _newOwner)`
Transfers ownership. OnlyOwner.

#### `createWave(string calldata _name, uint256 _durationBlocks)` → `uint256`
Creates a new wave. Returns the wave ID (keccak256 hash of name+block). OnlyOwner.

#### `registerIssue(string calldata _url)`
Registers an issue for the current wave. Reverts if already registered. OnlyOwner.

#### `voteOnIssue(string calldata _url, uint256 _weight)`
Casts a weighted vote. Each address gets one vote per issue. Weight is boosted by voter's reputation balance. Multi-call reverts.

#### `fastTrackIssue(string calldata _url, string calldata _reason)`
Proposes and immediately executes an emergency fast-track, doubling the max weight on the issue. OnlyOwner.

#### `startWork(string calldata _url)`
Marks an issue as having work started by the caller. Prevents later point changes without slashing.

#### `finalizeWave(uint256 _waveId)`
Locks a wave, preventing further voting. OnlyOwner.

#### `adjustPoints(string calldata _url, uint256 _newPoints, string calldata _reason)`
Adjusts issue points. If work has started beyond the grace window, the caller is slashed. OnlyOwner.

### View Functions

| Function | Returns | Description |
|----------|---------|-------------|
| `getIssue(string)` | `(weight, points, isEmergency, contributor)` | Full issue state |
| `getWave(uint256)` | `WaveData` | Full wave state |
| `getCurrentWave()` | `WaveData` | Active wave |
| `getWaveConfig()` | `WaveConfig` | Contract parameters |
| `getVoterWeight(address, string)` | `uint256` | Vote weight cast by user |

## ReputationManager

| Function | Description |
|----------|-------------|
| `balanceOf(address)` | Returns reputation balance |
| `mintReputation(address, uint256)` | Only WaveScoping |
| `burnReputation(address, uint256)` | Only WaveScoping. Reverts if insufficient |
| `consumeReputation(address, uint256)` | Only WaveScoping. Returns bool |
| `decayAll()` | Advances epoch, decaying all balances |

## EmergencyScoping

| Function | Description |
|----------|-------------|
| `proposeFastTrack(string, string, address)` | Creates a proposal. OnlyOwner |
| `executeFastTrack(string, address)` | Executes after timelock. OnlyOwner |
| `getProposal(string)` | Returns proposal details |
| `isFastTracked(string)` | Returns whether executed |

## SlashingManager

| Function | Description |
|----------|-------------|
| `slash(address,string,uint256,uint256,string)` | Records a slash. Only WaveScoping |
| `getSlashRecord(address,string)` | Returns slash record |
| `totalSlashed(address)` | Aggregated penalty for maintainer |

## Events

| Event | Parameters |
|-------|------------|
| `IssueRegistered` | `url`, `waveId` |
| `Voted` | `voter`, `url`, `weight` |
| `IssueFastTracked` | `url`, `maintainer` |
| `ContributionStarted` | `url`, `contributor` |
| `ReputationEarned` | `voter`, `amount` |
| `SlashApplied` | `maintainer`, `penalty` |
| `WaveCreated` | `id`, `name`, `endBlock` |
| `WaveFinalized` | `id` |
| `PointsAdjusted` | `url`, `oldPoints`, `newPoints` |
