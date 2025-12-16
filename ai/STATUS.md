# Status

**Version**: 0.0.1-alpha1
**Phase**: Core complete, ready for alpha testing

## What Works

| Command         | Status | Notes                     |
| --------------- | ------ | ------------------------- |
| `jb run "cmd"`  | Tested | Auto-starts daemon        |
| `jb run --wait` | Tested | Blocks, returns exit code |
| `jb run -t 5s`  | Tested | Timeout kills job         |
| `jb run -k key` | Tested | Idempotency works         |
| `jb list`       | Tested |                           |
| `jb status`     | Tested | System + job detail       |
| `jb logs`       | Tested |                           |
| `jb stop`       | Tested | Via daemon IPC            |
| `jb wait`       | Tested | Via daemon IPC            |
| `jb retry`      | Tested | Via daemon IPC            |
| `jb clean`      | Tested |                           |
| `--json`        | Tested | Valid JSON output         |

## Not Yet Tested

- Daemon crash recovery
- System reboot behavior
- Concurrent job submission
- Very long-running jobs (hours+)
- Large output files (GB+)
- Actual agent session survival

## Known Limitations

- No automated tests
- 41 clippy pedantic warnings (all docs/style)
- `spawn_job` has too many arguments (should use struct)

## Platforms

| Platform       | Build | Manual Tests |
| -------------- | ----- | ------------ |
| macOS (arm64)  | Pass  | Pass         |
| Linux (Fedora) | Pass  | Pass         |

## Next Steps

1. Real-world testing with Claude Code
2. Add integration tests
3. Daemon crash recovery
4. Consider systemd/launchd integration
