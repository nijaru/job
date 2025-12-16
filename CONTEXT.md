# Context

Background job manager for AI agents. Allows agents to spawn tasks that survive session end.

## Quick Orientation

| What                  | Where             |
| --------------------- | ----------------- |
| Architecture & design | `ai/DESIGN.md`    |
| Current status        | `ai/STATUS.md`    |
| Design decisions      | `ai/DECISIONS.md` |
| Tasks                 | `bd list`         |
| Skills for Claude     | `skills/skill.md` |

## Project Structure

```
jb/
├── crates/
│   ├── job-cli/      # CLI binary (jb)
│   ├── job-daemon/   # Daemon binary (jbd) - NOT YET IMPLEMENTED
│   └── job-core/     # Shared library (types, DB, IPC protocol)
├── skills/           # Claude skills file
└── ai/               # Design docs
```

## Current State

**CLI works, daemon doesn't.**

`jb run "cmd"` creates a job in `~/.jb/job.db` but the job stays `pending` because the daemon (`jbd`) isn't implemented to execute it.

## What Needs to Be Built

The daemon (`crates/job-daemon/src/main.rs`) needs to:

1. Listen on Unix socket (`~/.jb/daemon.sock`)
2. Receive job requests from CLI
3. Spawn processes with `setsid()` (detached)
4. Monitor processes, capture output to `~/.jb/logs/<id>.log`
5. Update DB with exit codes

See `bd list` for tracked tasks.

## Key Files

| File                            | Purpose                            |
| ------------------------------- | ---------------------------------- |
| `crates/job-core/src/job.rs`    | Job struct and Status enum         |
| `crates/job-core/src/db.rs`     | SQLite operations                  |
| `crates/job-core/src/ipc.rs`    | Request/Response protocol          |
| `crates/job-cli/src/main.rs`    | CLI entry point                    |
| `crates/job-daemon/src/main.rs` | Daemon stub (needs implementation) |

## Commands

```bash
cargo build --release       # Build
./target/release/jb --help  # CLI help
bd list                     # View tasks
```
