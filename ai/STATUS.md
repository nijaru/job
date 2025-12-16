# Status

**Phase**: CLI scaffolding complete, daemon not implemented

## Completed

- [x] Project structure (Cargo workspace)
- [x] Core types (Job, Status, Database, IPC protocol)
- [x] CLI skeleton with all 9 commands
- [x] Skills file for Claude integration
- [x] Design documentation
- [x] Renamed from `job` to `jb` (crate name conflict)
- [x] GitHub repo: github.com/nijaru/jb
- [x] Beads initialized with tasks

## What Works

- `jb run "cmd"` - creates job in DB, returns ID
- `jb list` - shows jobs for current project
- `jb status <id>` - shows job details
- `jb status` - shows system status
- `jb clean` - removes old jobs from DB

## What Doesn't Work Yet

- Jobs stay `pending` forever (daemon not executing them)
- `jb stop` - can't kill without daemon tracking PID
- `jb logs` - no logs since jobs don't run
- `jb wait` - polls DB but job never completes
- `jb run --wait` - not wired up

## Next Steps

See `bd list` for tasks:

1. **job-390**: Implement daemon IPC listener (P1)
2. **job-pa2**: Implement job spawning with process groups (P1)
3. **job-2nv**: Implement process monitoring loop (P2)
4. **job-a2m**: Wire CLI to daemon communication (P2)

## Build & Test

```bash
cargo build --release
./target/release/jb --help
./target/release/jb run "echo test" --name test
./target/release/jb list
```
