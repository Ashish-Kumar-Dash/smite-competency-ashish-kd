# Smite Codebase Understanding Notes
After going through the codebase, this is my understanding of the codebase so far...
## 1. Runtime architecture

Core execution path:

1. Scenario binary starts in `smite-scenarios/src/bin/*`.
2. Binary calls `smite::scenarios::smite_run::<ScenarioType>()`.
3. `smite_run` initializes runner first, then scenario (`smite/src/scenarios.rs`).
4. Runner dispatch:
   - Nyx mode when `SMITE_NYX` is set.
   - Local mode otherwise (`smite/src/runners.rs`).
5. Scenario executes fuzz input and reports `Ok`, `Skip`, or `Fail`.

## 2. Process and crash model

Smite wraps child processes in `ManagedProcess` (`smite/src/process.rs`):

- Child is spawned into its own process group.
- Graceful shutdown sends `SIGTERM` to process group.
- On timeout, escalates to `SIGKILL`.
- `Drop` auto-cleans live processes.

This is central to avoiding orphaned grandchildren and zombie trees.

## 3. Target orchestration

Each target implementation in `smite-scenarios/src/targets` starts bitcoind + LN node and exposes:

- `pubkey()` and `addr()` for Noise handshake setup.
- `check_alive()` for crash detection.

Target-specific behavior:

- LND: coverage sync via pipes because Go cannot directly write AFL shared memory.
- CLN: C AFL instrumentation writes directly; liveness + crash log checks.
- LDK: Rust AFL instrumentation writes directly; liveness + crash log checks.
- Eclair: Java agent/JNI path; liveness + crash log checks.

## 4. Scenario semantics

Scenarios in `smite-scenarios/src/scenarios`:

- `encrypted_bytes`: send raw encrypted payloads.
- `init`: fuzz BOLT init message with valid encryption context.
- `noise`: fuzz BOLT 8 handshake at different stages.

Common strategy:

- Warm up paths before snapshot.
- Use ping/pong sync to reduce false negatives when deciding target liveness.

## 5. IR and structured fuzzing

`smite-ir` defines typed operations, generators, and mutators.

- Generator (`generators/open_channel.rs`) builds realistic `open_channel -> accept_channel` flows.
- Mutators preserve structural validity while perturbing literals and operation parameters.
- Executor (`smite-scenarios/src/executor.rs`) enforces runtime type checks and operation input arity.

## 6. Existing operational tooling

Scripts already provide key automation primitives:

- `scripts/setup-nyx.sh`: sharedir creation.
- `scripts/coverage-report.sh`: per-target coverage merge/report generation.
- `scripts/symbolize-crash.sh`: CLN crash symbolization workflow.

These scripts are natural foundations for a future `smitebot` command set.

## 7. What this means for smitebot design

A strong `smitebot` design can build on existing seams rather than replacing internals:

1. Reuse current Docker build matrix per target/scenario.
2. Use `ManagedProcess` process-group cleanup strategy for robust start/stop.
3. Wrap existing coverage and crash scripts as first-class CLI subcommands.
4. Normalize campaign metadata and artifact layout for triage and dashboarding.
5. Keep runner abstraction intact so Nyx/local behavior remains transparent to users.
