# smite-competency-ashish-kd

> **Smite Summer of Bitcoin 2026 — Competency Submission**

---

## Candidate Information

| Field | Details |
|-------|---------|
| **Name** | Ashish Kumar Dash |
| **Email** | ashishdash2410@gmail.com |
| **Discord** | pablochocobar__ |
| **GitHub** | [github.com/Ashish-Kumar-Dash](https://github.com/Ashish-Kumar-Dash) |
| **College** | Indian Institute of Technology Bhilai — B.Tech CSE (2024–2028) |
| **Country** | India |
| **Time Zone** | IST (UTC+5:30) |

---

## Objective

This repository contains my completed competency work for the Smite project proposal: **Smitebot: Automated Fuzzing Campaign Manager**.

The competency required:

1. Running Smite Quick Start in AFL++ Nyx mode for one target and scenario.
2. Generating a coverage report from the fuzzing corpus.
3. Running a multi-core campaign using AFL Runner, then merging and minimizing corpus with `afl-cmin`.
4. Writing a Rust subprocess cleanup demo that terminates child and grandchild processes correctly.

---

## Repository Contents

| File | Description |
|------|-------------|
| `codebase_understanding.md` | Architecture and codebase understanding notes |
| `aflr_cfg_smite_lnd_encrypted_bytes.toml` | AFL Runner config used for multi-core orchestration |
| `merge_and_minimize_corpus.sh` | Helper script to merge queue files and run `afl-cmin` in Nyx mode |
| `Cargo.toml` | Cargo manifest for process cleanup demo |
| `main.rs` | Rust implementation of signal-aware subprocess group cleanup |
| `task2/` | Screenshot of working dashboard (required for Task 2) |
| `coverage.txt` | Coverage text report artifact from competency run |

---

## Competency Task 1: Quick Start + Coverage

**Target:** `lnd`  
**Scenario:** `encrypted_bytes`  
**Mode:** AFL++ Nyx

### Steps

- Built workload image for target + scenario.
- Enabled VMware backdoor for Nyx.
- Created Nyx sharedir using setup script.
- Started AFL++ fuzzing in Nyx mode.
- Generated HTML/text coverage report from corpus.

### Key Output

- Coverage report generated for `lnd` + `encrypted_bytes`.
- Included artifact: [`coverage.txt`](./coverage.txt)

---

## Competency Task 2: Multi-core Campaign + Corpus Minimize

**Tool used:** AFL Runner

### Steps

- Configured 8-runner campaign (1 main + secondaries) via TOML config.
- Ran campaign and monitored status.
- Merged queue corpus from all runners.
- Minimized merged corpus via `afl-cmin` in Nyx mode.

### Key Output

- Working AFL Runner config for Smite Nyx workflow: [`aflr_cfg_smite_lnd_encrypted_bytes.toml`](./aflr_cfg_smite_lnd_encrypted_bytes.toml)
- Merge and minimize automation script: [`merge_and_minimize_corpus.sh`](./merge_and_minimize_corpus.sh)
- Task.png Screenshot showing status dashboard

---

## Competency Task 3: Rust Child/Grandchild Cleanup

### What was implemented

- Rust program spawns child process in a dedicated process group.
- Program listens for `SIGINT`/`SIGTERM`.
- On signal, sends `SIGTERM` to the full process group.
- Escalates to `SIGKILL` after timeout if needed.
- Ensures child and grandchildren are fully cleaned up.

**Implementation:** [`main.rs`](./main.rs)

---

## Related Upstream Contribution

I also submitted and merged the following Smite PR, directly relevant to campaign lifecycle reliability:

> **PR: smite: Clean up managed subprocess groups on shutdown**  
> 🔗 [github.com/morehouse/smite/pull/32](https://github.com/morehouse/smite/pull/32)

### Summary

- Added process-group-based managed subprocess cleanup.
- Improved shutdown semantics for child and descendant process trees.
- Added regression coverage for child + grandchild cleanup behavior.
- Manually validated across CLN, LND, LDK, and Eclair coverage workflows.

---

## Reproducibility Notes

**Environment assumptions:**

- Linux x86\_64
- Docker
- AFL++ built from source with Nyx mode support
- KVM enabled
- Access to Smite repository and workload Dockerfiles

---

## Proposal Context

The accompanying proposal focuses on **Smitebot** as a Rust CLI orchestration layer to automate:

- Campaign start / stop / status
- Build and prerequisite checks
- Corpus merge and minimize
- Crash triage and deduplication
- Coverage snapshots and diffs
- Optional daemon mode for unattended campaigns
