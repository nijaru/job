# Context

Background job manager for AI agents. Allows agents to spawn tasks that survive session end.

## Quick Orientation

| What                  | Where                            |
| --------------------- | -------------------------------- |
| Architecture & design | `ai/DESIGN.md`                   |
| Current status        | `ai/STATUS.md`                   |
| Design decisions      | `ai/DECISIONS.md`                |
| Tasks                 | `bd list`                        |
| Skills for Claude     | `crates/job-cli/skills/SKILL.md` |

## Project Structure

```
jb/
├── crates/
│   ├── job-cli/      # CLI binary (jb)
│   ├── job-daemon/   # Daemon binary (jbd)
│   └── job-core/     # Shared library (types, DB, IPC protocol)
└── ai/               # Design docs
```

## Key Files

| File                            | Purpose                    |
| ------------------------------- | -------------------------- |
| `crates/job-core/src/job.rs`    | Job struct and Status enum |
| `crates/job-core/src/db.rs`     | SQLite operations          |
| `crates/job-core/src/ipc.rs`    | Request/Response protocol  |
| `crates/job-cli/src/main.rs`    | CLI entry point            |
| `crates/job-daemon/src/main.rs` | Daemon entry point         |

## Commands

```bash
cargo build --release       # Build
./target/release/jb --help  # CLI help
bd list                     # View tasks
```

## Releasing

**Always use the release workflow - never publish manually.**

```bash
# 1. Bump version in root Cargo.toml (workspace.package.version and workspace.dependencies.jb-core)
# 2. Commit and push
# 3. Wait for CI to pass
gh run list --limit 1

# 4. Trigger release workflow
gh workflow run release.yml

# 5. Watch release
gh run watch
```

The release workflow:

- Verifies version isn't already published
- Runs fmt/clippy checks
- Builds for linux (x86_64, aarch64) and macos (x86_64, aarch64)
- Publishes via trusted publishing (OIDC)
