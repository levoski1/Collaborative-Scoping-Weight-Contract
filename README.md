# WaveScoping — Decentralized Backlog Prioritization

> **A governance-lite smart contract system that lets communities collaboratively prioritize GitHub issues through weighted voting, reputation-based influence, and conflict-resolution slashing.**

[![CI](https://github.com/your-org/wave-scoping/actions/workflows/ci.yml/badge.svg)](https://github.com/your-org/wave-scoping/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Foundry](https://img.shields.io/badge/Built%20with-Foundry-000000.svg)](https://book.getfoundry.sh/)
[![Solidity ^0.8.20](https://img.shields.io/badge/Solidity-^0.8.20-blue)](https://soliditylang.org/)

---

## Table of Contents

- [Problem Statement](#problem-statement)
- [Vision & Goals](#vision--goals)
- [Key Features](#key-features)
- [Technical Architecture](#technical-architecture)
- [Technology Stack](#technology-stack)
- [Folder Structure](#folder-structure)
- [Quick Start](#quick-start)
- [Installation](#installation)
- [Configuration](#configuration)
- [Usage](#usage)
- [API Reference](#api-reference)
- [Testing Strategy](#testing-strategy)
- [Deployment](#deployment)
- [Security](#security)
- [CI/CD](#cicd)
- [Docker](#docker)
- [Contributing](#contributing)
- [Coding Standards](#coding-standards)
- [Roadmap](#roadmap)
- [FAQ](#faq)
- [License](#license)

---

## Problem Statement

Open-source maintainers face a fundamental coordination problem: **how to prioritize which issues to tackle in each development cycle.** Traditionally, a single maintainer or small core team assigns point values to issues, creating a bottleneck and potential for bias. This system decentralizes that process, allowing **token holders and proven contributors** to vote on which GitHub issues should be included in the next development "Wave."

---

## Vision & Goals

- **Decentralize the backlog** — Let the community, not just maintainers, influence prioritization.
- **Reward participation** — Voters earn reputation points (used as voting weight in future waves).
- **Prevent capture** — Reputation decays over time so power concentrates in active, not historical, contributors.
- **Handle emergencies** — Maintainers retain a fast-track mechanism for critical security fixes.
- **Ensure fairness** — Slashing penalizes arbitrary point changes after work has started.

---

## Key Features

### Weighted Voting
Each address can cast one vote per issue. Voting weight is boosted by the voter's accumulated reputation balance. Raw weight plus reputation bonus combine to produce an effective weight (capped by `MAX_WEIGHT_PER_VOTE`). Points are derived from the total weight using a linear divisor.

### Reputation System (`ReputationManager`)
- Voters earn reputation every time they vote (`weight / 10 + 1`).
- Reputation decays each epoch (configurable basis points).
- Reputation serves as voting weight in subsequent waves.
- Decay ensures ongoing participation is rewarded over past activity.

### Emergency Scoping (`EmergencyScoping`)
- Maintainers can propose an issue for fast-track.
- A timelock (configurable blocks) prevents instant execution.
- Once executed, the issue receives a weight boost of `2 * MAX_WEIGHT_PER_VOTE`.
- Emits `IssueFastTracked` for off-chain monitoring.

### Conflict Resolution / Slashing (`SlashingManager`)
- When a maintainer adjusts points after a contributor has started work (past a grace window), the maintainer is slashed.
- Penalty = `(newPoints - oldPoints) * slashMaxBps / 10000`.
- 50% of the penalty is burned (configurable), preventing wasteful redistribution.
- Duplicate slashes on the same issue are prevented.

### Wave Lifecycle
1. **Create Wave** — Owner starts a new wave with a name and duration (in blocks).
2. **Register Issues** — Owner adds GitHub issue URLs to the wave.
3. **Voting Period** — Community votes on issues (weighted by reputation).
4. **Work Start** — Contributors signal they have begun work on an issue.
5. **Finalize** — Owner locks the wave, preventing further changes.
6. **Reputation Settlement** — Voters receive reputation for their participation.

---

## Technical Architecture

```
┌──────────────────────────────────────────────────────────────────┐
│                       WaveScoping (Hub)                           │
│  ┌──────────────┐  ┌──────────────────┐  ┌───────────────────┐   │
│  │ Reputation   │  │ EmergencyScoping │  │ SlashingManager   │   │
│  │ Manager      │  │                  │  │                   │   │
│  └──────────────┘  └──────────────────┘  └───────────────────┘   │
│  ┌──────────────┐  ┌──────────────────┐                           │
│  │ MathLib      │  │ WaveLib          │                           │
│  └──────────────┘  └──────────────────┘                           │
└──────────────────────────────────────────────────────────────────┘
```

### Hub-and-Spoke Pattern
The `WaveScoping` contract is the single entry point. It delegates specialized concerns to three satellite modules:

| Module | Responsibility | Upgrade Strategy |
|--------|---------------|------------------|
| `ReputationManager` | Tracks balances, mints/burns, decays | Replace via proxy |
| `EmergencyScoping` | Fast-track proposals, timelock enforcement | Independent |
| `SlashingManager` | Penalty calculation, burn logic | Independent |

**Why not inheritance?**
1. **Bytecode size** — Solidity's 24KB limit is hit quickly. Modules keep the hub lean.
2. **Testability** — Each module is a standalone contract.
3. **Upgradeability** — Modules can be swapped without redeploying the hub (future: UUPS proxy).

---

## Technology Stack

| Layer | Technology | Rationale |
|-------|-----------|-----------|
| Smart Contracts | Solidity ^0.8.20 | Industry standard, safe math by default |
| Development Framework | [Foundry](https://book.getfoundry.sh/) | Blazing fast, Solidity-native testing |
| Testing | Forge (built-in) | Gas-aware, fuzz testing, console logging |
| Linting | `forge fmt` | Opinionated Solidity formatter |
| Static Analysis | Slither | Detects common vulnerability patterns |
| Deployment | Forge Scripts | Multi-chain friendly, Etherscan verification |
| Containerization | Docker + Foundry image | Reproducible builds |
| CI/CD | GitHub Actions | Foundry-native actions available |

---

## Folder Structure

```
wave-scoping/
├── src/                          # Smart contracts
│   ├── WaveScoping.sol           # Hub contract (orchestrator)
│   ├── interfaces/               # Type definitions & event signatures
│   │   ├── IWaveScoping.sol
│   │   ├── IReputationManager.sol
│   │   └── ISlashingManager.sol
│   ├── modules/                  # Logic satellites
│   │   ├── ReputationManager.sol
│   │   ├── EmergencyScoping.sol
│   │   └── SlashingManager.sol
│   └── libraries/                # Pure utility functions
│       ├── MathLib.sol           # Basis points, decay, weighted scores
│       └── WaveLib.sol           # Wave structs, validation helpers
├── test/                         # Forge test suite
│   ├── helpers/
│   │   └── TestHelpers.sol       # Shared setup & constants
│   ├── WaveScoping.t.sol         # Hub contract tests
│   ├── ReputationManager.t.sol
│   ├── EmergencyScoping.t.sol
│   ├── SlashingManager.t.sol
│   └── integration/
│       └── FullWaveFlow.t.sol    # End-to-end wave lifecycle
├── script/                       # Deployment & interaction scripts
│   ├── Deploy.s.sol
│   └── Interactions.s.sol
├── docs/
│   ├── architecture.md
│   ├── api.md
│   └── threats.md
├── .github/workflows/
│   ├── ci.yml                    # Lint, test, coverage, Slither
│   └── deploy.yml                # Forge script deployment
├── foundry.toml                  # Solidity compiler config
├── remappings.txt                # Import aliases
├── Dockerfile                    # Reproducible build image
├── docker-compose.yml            # Local Anvil + deploy + test
├── .env.example                  # Environment template
├── .gitignore
└── README.md
```

---

## Quick Start

```bash
# 1. Install Foundry
curl -L https://foundry.paradigm.xyz | bash
foundryup

# 2. Clone & enter
git clone <repo-url> wave-scoping && cd wave-scoping

# 3. Install submodules (forge-std)
forge install

# 4. Build
forge build

# 5. Test
forge test -vvv

# 6. Run gas snapshot
forge snapshot
```

---

## Installation

### Prerequisites

- **Foundry** (>= nightly): `curl -L https://foundry.paradigm.xyz | bash && foundryup`
- **Git**: For submodule management.
- **Make** (optional): If using our `Makefile` shortcuts.

### Step-by-step

```bash
# 1. Fork or clone the repository
git clone https://github.com/your-org/wave-scoping.git
cd wave-scoping

# 2. Install Forge submodules
forge install

# 3. Compile contracts
forge build

# 4. (Optional) Copy and edit environment
cp .env.example .env
# Edit .env with your RPC URLs and private key
```

---

## Configuration

### Environment Variables (`.env`)

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `RPC_MAINNET` | For mainnet deploy | — | Alchemy/Infura mainnet URL |
| `RPC_SEPOLIA` | For testnet deploy | — | Alchemy/Infura Sepolia URL |
| `PRIVATE_KEY` | For deployment | — | Deployer wallet private key (0x-prefixed) |
| `ETHERSCAN_API_KEY` | For verification | — | Etherscan API key |
| `FOUNDRY_PROFILE` | No | `default` | Compiler profile (`default` / `optimized` / `ci`) |

### Contract Parameters (in `Deploy.s.sol`)

| Parameter | Description | Value |
|-----------|-------------|-------|
| `VOTING_PERIOD_BLOCKS` | Voting window (~7 days at 12s/block) | 64800 |
| `EMERGENCY_TIMELOCK` | Blocks before fast-track executes | 5 |
| `MAX_WEIGHT_PER_VOTE` | Maximum vote weight per address | 100 |
| `POINTS_DIVISOR` | Weight → Points divisor | 10 |
| `DECAY_RATE_BPS` | Reputation decay per epoch (1%) | 100 |
| `SLASH_MAX_BPS` | Max slash penalty (20%) | 2000 |
| `SLASH_BURN_RATE_BPS` | Portion of penalty burned (50%) | 5000 |

---

## Usage

### Local Development with Anvil

```bash
# Terminal 1: Start local chain
anvil

# Terminal 2: Deploy
forge script script/Deploy.s.sol --rpc-url http://127.0.0.1:8545 \
  --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 --broadcast

# Terminal 2: Create a wave
WAVE_SCOPING_ADDRESS=<deployed-address> \
WAVE_NAME="Sprint 1" \
WAVE_DURATION_BLOCKS=64800 \
  forge script script/Interactions.s.sol:CreateWave --rpc-url ... --broadcast
```

### Common Workflows

**1. Register an Issue**
```bash
ISSUE_URL="https://github.com/owner/repo/issues/42" \
  forge script script/Interactions.s.sol:RegisterIssue --rpc-url <rpc> --broadcast
```

**2. Vote**
```bash
ISSUE_URL="https://github.com/owner/repo/issues/42" \
VOTE_WEIGHT=50 \
  forge script script/Interactions.s.sol:VoteOnIssue --rpc-url <rpc> --broadcast
```

**3. Fast-Track (emergency)**
```bash
ISSUE_URL="https://github.com/owner/repo/issues/42" \
REASON="Critical auth vulnerability" \
  forge script script/Interactions.s.sol:FastTrackIssue --rpc-url <rpc> --broadcast
```

**4. Start work**
```bash
ISSUE_URL="https://github.com/owner/repo/issues/42" \
  forge script script/Interactions.s.sol:StartWork --rpc-url <rpc> --broadcast
```

**5. Adjust Points (triggers slashing if after work start)**
```bash
ISSUE_URL="https://github.com/owner/repo/issues/42" \
NEW_POINTS=80 \
REASON="Scope increased" \
  forge script script/Interactions.s.sol:AdjustPoints --rpc-url <rpc> --broadcast
```

**6. Finalize Wave**
```bash
WAVE_ID=<wave-id> \
  forge script script/Interactions.s.sol:FinalizeWave --rpc-url <rpc> --broadcast
```

---

## API Reference

Full API documentation is in [`docs/api.md`](docs/api.md). Key functions:

### State-Changing

| Function | Access | Description |
|----------|--------|-------------|
| `createWave(name, duration)` | Owner | Opens a new voting wave |
| `registerIssue(url)` | Owner | Adds an issue to current wave |
| `voteOnIssue(url, weight)` | Anyone | Cast weighted vote (one per address) |
| `fastTrackIssue(url, reason)` | Owner | Emergency bypass |
| `startWork(url)` | Anyone | Signal work begun |
| `finalizeWave(waveId)` | Owner | Lock wave state |
| `adjustPoints(url, newPts, reason)` | Owner | Update points (may trigger slash) |
| `setOwner(newOwner)` | Owner | Transfer ownership |

### View

| Function | Returns |
|----------|---------|
| `getIssue(url)` | `(weight, points, isEmergency, contributor)` |
| `getWave(waveId)` | Full `WaveData` struct |
| `getCurrentWave()` | Active wave data |
| `getWaveConfig()` | `WaveConfig` parameters |
| `getVoterWeight(voter, url)` | Cast vote weight |

---

## Testing Strategy

### Unit Tests
- **WaveScoping.t.sol** — 15+ tests covering registration, voting, ownership, wave lifecycle, error cases.
- **ReputationManager.t.sol** — Mint, burn, consume, decay, auth checks.
- **EmergencyScoping.t.sol** — Propose, execute, timelock, duplicate prevention.
- **SlashingManager.t.sol** — Slash calculation, duplicate detection, no-change guard.

### Integration Tests
- **FullWaveFlow.t.sol** — End-to-end wave lifecycle (create → register → vote → work → finalize → reputation settlement).

### Coverage
```bash
forge coverage --report lcov
# View: genhtml lcov.info -o coverage/ && open coverage/index.html
```

### Gas Snapshot
```bash
forge snapshot
# Creates .gas-snapshot; diff against baseline in CI.
```

Performance benchmarks (reference):
| Operation | Gas Cost |
|-----------|----------|
| `registerIssue` | ~45,000 |
| `voteOnIssue` | ~65,000 |
| `fastTrackIssue` | ~120,000 |
| `startWork` | ~35,000 |
| `finalizeWave` | ~28,000 |

---

## Deployment

### Testnet (Sepolia)

```bash
source .env
forge script script/Deploy.s.sol:DeployScript \
  --rpc-url $RPC_SEPOLIA \
  --private-key $PRIVATE_KEY \
  --broadcast \
  --verify \
  --etherscan-api-key $ETHERSCAN_API_KEY \
  -vvvv
```

### Mainnet

> ⚠️ Run a full audit before mainnet deployment. Use a multisig as the owner.

```bash
forge script script/Deploy.s.sol:DeployScript \
  --rpc-url $RPC_MAINNET \
  --private-key $PRIVATE_KEY \
  --broadcast \
  --verify \
  --etherscan-api-key $ETHERSCAN_API_KEY \
  --slow
```

### Verification
If `--verify` fails, rerun manually:
```bash
forge verify-contract <address> src/WaveScoping.sol:WaveScoping \
  --chain sepolia \
  --etherscan-api-key $ETHERSCAN_API_KEY
```

---

## Security

### Access Control
- **Owner** controls: `registerIssue`, `createWave`, `finalizeWave`, `fastTrackIssue`, `adjustPoints`, `setOwner`.
- **Anyone** can: `voteOnIssue`, `startWork`.
- **Modules** are only callable by `WaveScoping` hub.

### Key Safeguards
| Risk | Mitigation |
|------|-----------|
| Owner abuse | Ownership transferrable to multisig. Post-work point changes trigger slashing. |
| Sybil voting | Unique address tracked, reputation-gated influence. |
| Emergency abuse | Timelock enforced before fast-track execution. |
| Reentrancy | Checks-Effects-Interactions pattern. No untrusted external calls. |
| Integer overflow | Solidity ^0.8.20 has built-in overflow checking. |

See [docs/threats.md](docs/threats.md) for the full threat model.

---

## CI/CD

### CI Pipeline (`.github/workflows/ci.yml`)
Runs on every push/PR to `main`:
1. **Lint** — `forge fmt --check`
2. **Test** — `forge test -vvv` + gas snapshot
3. **Coverage** — `forge coverage --report lcov` → Codecov
4. **Static Analysis** — Slither vulnerability scan

### Deploy Pipeline (`.github/workflows/deploy.yml`)
Manual trigger (`workflow_dispatch`):
1. Select network (`sepolia` / `mainnet`)
2. Toggle Etherscan verification
3. Deploys via `forge script`, saves broadcast artifacts

---

## Docker

### Build
```bash
docker build -t wave-scoping .
```

### Run full local stack
```bash
docker compose up --build
```

This starts:
- **Anvil** — Local Ethereum node on `:8545`
- **Deploy** — Deploys contracts to Anvil
- **Test** — Runs full test suite against the deployment

---

## Contributing

We welcome contributions! Please follow these steps:

1. **Fork** the repository.
2. **Create a feature branch**: `git checkout -b feat/my-feature`
3. **Commit** your changes: `git commit -m "feat: add new feature"`
4. **Push** to your fork: `git push origin feat/my-feature`
5. **Open a Pull Request** against `main`.

### PR Checklist
- [ ] `forge build` passes without warnings
- [ ] `forge fmt --check` passes
- [ ] `forge test -vvv` passes
- [ ] New tests cover added/modified code
- [ ] Gas snapshot is updated (`forge snapshot`)
- [ ] Documentation updated if API changes
- [ ] No new Slither warnings

### Commit Convention
We use [Conventional Commits](https://www.conventionalcommits.org/):
```
feat: add reputation decay
fix: prevent double voting in same wave
docs: update API reference
test: add slashing edge cases
refactor: extract math to library
```

---

## Coding Standards

### Solidity
- **Compiler**: ^0.8.20 (explicitly check for overflow safety)
- **Style**: Follow `forge fmt` (no manual formatting exceptions)
- **Naming**:
  - `camelCase` for functions and variables
  - `CamelCase` for contracts, enums, structs
  - `UPPER_SNAKE_CASE` for constants
  - `_prefixed` for internal/private
- **Ordering**:
  1. Type declarations
  2. State variables
  3. Events
  4. Errors
  5. Constructor
  6. External functions
  7. Public functions
  8. Internal functions
  9. Private functions
- **Comments**: Use NatSpec `///` for all public/external functions. Use `//` for internal comments only when logic is non-obvious.
- **Imports**:
  ```solidity
  import {IWaveScoping} from "@interfaces/IWaveScoping.sol";
  import {MathLib} from "@libraries/MathLib.sol";
  ```
- **Access modifiers**: Explicitly label `external`, `public`, `internal`, `private`.
- **Custom errors**: Use `error` declarations instead of `require` with strings (cheaper gas, better DX).

### Git
- No direct pushes to `main`. PRs require CI green + review.
- Rebase before merging: `git rebase main`

---

## Roadmap

### Phase 1 (Current) — MVP
- [x] Core voting with reputation-weighted influence
- [x] Emergency fast-track with timelock
- [x] Slashing for post-work point changes
- [x] Foundry test suite with 90%+ coverage
- [x] CI/CD pipeline

### Phase 2 — Governance
- [ ] UUPS upgradeable proxy for hub contract
- [ ] DAO-owned contract (replace single owner with governor)
- [ ] Reputation delegation (delegate voting power)
- [ ] Quadratic voting for more Sybil resistance

### Phase 3 — Integrations
- [ ] GitHub App (auto-sync issues, PRs, labels)
- [ ] Off-chain indexer (Graph Protocol subgraph)
- [ ] Frontend dApp (React + Wagmi + Viem)
- [ ] Webhook notifications on wave finalization

### Phase 4 — Advanced
- [ ] Cross-chain waves (LayerZero/Wormhole)
- [ ] Programmable incentives (liquidity mining for voters)
- [ ] ZK-proof based private voting
- [ ] On-chain dispute resolution (Kleros integration)

---

## FAQ

### How is reputation earned?
Each vote mints `weight / 10 + 1` reputation. No other minting mechanism exists.

### Can I lose reputation?
Yes. Reputation decays each epoch (configurable rate, 1% per epoch by default). The owner can also burn reputation (no current use, reserved for future slashing).

### What prevents the owner from manipulating everything?
1. Post-work point changes trigger slashing (the owner is penalized).
2. The owner can transfer ownership to a multisig or DAO.
3. Fast-track has a timelock, allowing community reaction.
4. All owner actions emit public events.

### Can I change my vote?
No. Each address gets one vote per issue. This prevents last-minute swing-voting attacks.

### What happens after a wave is finalized?
The wave is locked. Points and weights become immutable. Off-chain systems (future: GitHub integration) read finalized state to drive the actual issue tracker.

### How do I deploy on my own chain?
Set `RPC_LOCALHOST` in `.env` or pass `--rpc-url <your-rpc>`. Works with any EVM-compatible chain (Optimism, Arbitrum, Polygon, etc.).

---

## License

This project is licensed under the **MIT License**. See the [`LICENSE`](LICENSE) file for details.

```
MIT License

Copyright (c) 2026 WaveScoping Contributors

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files...
```

---

## Acknowledgments

- [Foundry Book](https://book.getfoundry.sh/) — The best Solidity development experience.
- [OpenZeppelin](https://www.openzeppelin.com/contracts) — Inspiration for access control patterns.
- [Solmate](https://github.com/transmissions11/solmate) — Gas-efficient library patterns referenced in MathLib.

---

<p align="center">
  Built with ❤️ by the WaveScoping Contributors
</p>
# Collaborative-Scoping-Weight-Contract
