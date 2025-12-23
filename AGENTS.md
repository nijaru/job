# Agent Instructions

## Project

Background job manager for AI agents. Rust CLI tool.

## Development

```bash
cargo build              # Dev build
cargo clippy -- -D warnings  # Lint
cargo fmt                # Format
cargo test               # Tests (none yet)
```

## Release Process

Releases are automated via GitHub Actions on tag push. **Do not run `cargo publish` manually.**

### Pre-flight (REQUIRED before tagging)

```bash
cargo fmt && cargo clippy -- -D warnings && cargo build
```

### Steps

1. Bump version in `Cargo.toml`
2. Update `CHANGELOG.md` and `ai/STATUS.md`
3. Commit: `git commit -m "vX.Y.Z: <summary>"`
4. Tag: `git tag -a vX.Y.Z -m "vX.Y.Z: <summary>"`
5. Push: `git push && git push --tags`
6. Wait for CI to pass

### NEVER force-push after release

Once a tag is pushed, the homebrew-tap autobump runs immediately and caches checksums.
If you force-push the tag with different binaries, homebrew will have stale checksums.

**If release fails**: Create a new patch version (v0.0.11 → v0.0.12), don't force-push.

The workflow:

- Verifies version isn't already on crates.io
- Runs fmt, clippy, build
- Builds binaries for linux (x86_64, aarch64) and macos (aarch64)
- Publishes to crates.io via OIDC
- Autobumps homebrew-tap formula

## Testing

Manual testing only currently. Key scenarios:

- `jb run "cmd"` - basic job start
- `jb run "cmd" --follow` - stream output
- `jb logs <id> --follow` - attach to running job
- `jb list` - verify EXIT column shows codes
- Exit code propagation (run job that exits non-zero)
