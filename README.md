# WaveScoping — Decentralized Backlog Prioritization

> **A Rust library for collaborative GitHub issue prioritization through weighted voting, reputation-based influence, and conflict-resolution slashing.**

[![CI](https://github.com/your-org/wave-scoping/actions/workflows/ci.yml/badge.svg)](https://github.com/your-org/wave-scoping/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

---

## Problem Statement

Open-source maintainers face a fundamental coordination problem: **how to prioritize which issues to tackle in each development cycle.** This system decentralizes that process, allowing **proven contributors** to vote on which GitHub issues should be included in the next development "Wave."

## Key Features

### Weighted Voting
Each address can cast one vote per issue. Voting weight is boosted by the voter's accumulated reputation balance. Points are derived from the total weight using a linear divisor.

### Reputation System
- Voters earn reputation every time they vote (`weight / 10 + 1`).
- Reputation decays each epoch (configurable basis points).
- Decay ensures ongoing participation is rewarded over past activity.

### Emergency Scoping
- Maintainers can propose an issue for fast-track with a timelock.
- Once executed, the issue receives a weight boost of `2 * MAX_WEIGHT_PER_VOTE`.

### Slashing
- When a maintainer adjusts points after a contributor has started work (past a grace window), the maintainer is slashed.
- Penalty = `(newPoints - oldPoints) * slashMaxBps / 10000`.

## Quick Start

```bash
# Build
cargo build

# Run tests
cargo test

# Run demo
cargo run -- demo
```

## Project Structure

```
src/
├── main.rs                    # CLI entry point
├── lib.rs                     # Re-exports
├── errors.rs                  # Error types
├── types.rs                   # Struct definitions
├── math.rs                    # Math utilities
├── wave.rs                    # Wave validation
├── reputation_manager.rs      # Reputation tracking
├── emergency_scoping.rs       # Fast-track proposals
├── slashing_manager.rs        # Slash calculations
└── wave_scoping.rs            # Hub / orchestrator
tests/
├── common/mod.rs              # Shared test setup
├── wave_scoping_test.rs       # Hub unit tests
├── reputation_manager_test.rs # Reputation tests
├── emergency_scoping_test.rs  # Emergency tests
├── slashing_manager_test.rs   # Slashing tests
└── integration_test.rs        # End-to-end tests
```

## Usage

```bash
# Create a wave
cargo run -- create-wave "Sprint 1" 64800

# Register an issue
cargo run -- register "https://github.com/owner/repo/issues/1"

# Cast a vote
cargo run -- vote alice "https://github.com/owner/repo/issues/1" 30

# Fast-track an issue
cargo run -- fast-track owner "https://github.com/owner/repo/issues/1" "Critical bug"

# Start work
cargo run -- start-work alice "https://github.com/owner/repo/issues/1"

# Finalize a wave
cargo run -- finalize owner <wave_id>

# Run interactive demo
cargo run -- demo
```

## Running Tests

```bash
cargo test                    # All tests
cargo test wave_scoping       # Hub tests only
cargo test integration        # Integration tests
cargo test -- --nocapture     # With stdout
```

## License

MIT
