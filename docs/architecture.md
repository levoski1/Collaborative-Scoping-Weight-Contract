# Architecture

## Overview

The WaveScoping system follows a **hub-and-spoke** modular architecture. The core `WaveScoping` contract acts as the orchestrator (hub), delegating specialized concerns to satellite modules (spokes): `ReputationManager`, `EmergencyScoping`, and `SlashingManager`.

## Architecture Diagram

```
┌──────────────────────────────────────────────────────────────┐
│                        WaveScoping (Hub)                      │
│  ┌──────────────┐  ┌──────────────────┐  ┌───────────────┐   │
│  │ Reputation    │  │ EmergencyScoping │  │ Slashing      │   │
|  | Manager       │  │                  │  │ Manager       │   │
│  └──────────────┘  └──────────────────┘  └───────────────┘   │
│  ┌──────────────┐  ┌──────────────────┐                       │
│  │ MathLib      │  │ WaveLib          │                       │
│  └──────────────┘  └──────────────────┘                       │
└──────────────────────────────────────────────────────────────┘
```

## Separation of Concerns

- **WaveScoping** — Orchestration, access control, issue/wave state, voting logic.
- **ReputationManager** — Tracks reputation balances, handles decay, enforces reputation-based weight boosting.
- **EmergencyScoping** — Manages fast-track proposals with timelock enforcement.
- **SlashingManager** — Handles penalty calculation, slash recording, and burn logic.
- **MathLib** — Pure math utilities (basis points, decay, weighted scores).
- **WaveLib** — Structs and pure functions for wave lifecycle validation.

## Design Decisions

### Why separate modules instead of inheritance?
- **Composability** — Modules can be upgraded/replaced independently.
- **Testability** — Each module can be tested in isolation.
- **Bytecode size** — Avoids the 24KB contract size limit by splitting logic.

### Why timelocks for emergency scoping?
- Prevents single-maintainer abuse.
- Gives the community a window to detect and respond to controversial fast-tracks.

### Why reputation decay?
- Encourages ongoing participation.
- Prevents hoarding of voting power by early contributors.
- Keeps governance weighted toward currently-active contributors.

### Why burn slashed tokens?
- Aligns with deflationary tokenomics.
- Prevents governance capture by malicious maintainers who might otherwise reclaim slashed reputation.
