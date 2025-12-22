# SIGTERM to shell wrapper, not child process

## Problem

When running `jb run "source .env && ./app"`, jb spawns a shell to execute the command. When `jb stop` sends SIGTERM, it goes to the shell process, not the app. The shell may not forward SIGTERM to children.

This causes:

- App keeps running after `jb stop`
- Stale locks (e.g., DuckDB) from unclean shutdown
- Resource leaks

## Workaround

Use `exec` to replace the shell with the app:

```bash
jb run "source .env && exec ./app"
```

## Potential Fixes

1. **Auto-prepend `exec`** to final command in shell string
2. **Use process groups** (`setpgid`) and signal entire group with `killpg`
3. **Document the exec pattern** in help/README

## Discovered

2024-12-22 in pacabot - graceful shutdown wasn't working because SIGTERM went to shell wrapper.
