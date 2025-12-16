# Status

**Phase**: Core complete, ready for use

## What Works

| Command          | Status   | Notes                                       |
| ---------------- | -------- | ------------------------------------------- |
| `jb run "cmd"`   | Working  | Auto-starts daemon, returns job ID          |
| `jb run --wait`  | Working  | Waits for completion, exits with job's code |
| `jb list`        | Working  | Shows jobs for current project              |
| `jb status`      | Working  | System status or job details                |
| `jb status <id>` | Working  | Detailed job info                           |
| `jb clean`       | Working  | Removes old completed jobs                  |
| `jb logs <id>`   | Partial  | Reads log file, no `--follow` yet           |
| `jb stop <id>`   | Untested | Sends kill signal                           |
| `jb wait <id>`   | Untested | Polls until complete                        |

## Architecture

```
jb run "cmd"
    │
    ├─► daemon not running? spawn jbd
    │
    └─► connect to ~/.jb/daemon.sock
            │
            └─► daemon spawns process (setsid)
                    │
                    ├─► stdout/stderr → ~/.jb/logs/<id>.log
                    └─► exit code → SQLite DB
```

## Polish Items (not tracked)

- `jb logs --follow` - tail -f style
- `jb retry <id>` - re-run failed job
- Orphan recovery on daemon start
- Graceful shutdown (finish running jobs)
- Log rotation

## Build & Test

```bash
cargo build --release
./target/release/jb run "sleep 2 && echo done" --name test
./target/release/jb list
./target/release/jb logs <id>
```
