# jb

Modern nohup. Background jobs with tracking.

## Install

```bash
cargo install jb
```

## Quick Start

```bash
$ jb run "cargo build --release"
a3x9

$ jb list
ID    STATUS     COMMAND
a3x9  running    cargo build --release

$ jb logs a3x9
   Compiling foo v0.1.0
   ...

$ jb status a3x9
Status: completed
Exit: 0
```

## vs nohup

```bash
# nohup way
nohup cmd > /tmp/log-$$.txt 2>&1 &
echo $!  # remember this somehow
# later: where was that log? what was the PID?

# jb way
jb run "cmd"   # returns: a3x9
jb logs a3x9   # output is here
jb status a3x9 # status is here
```

## Commands

| Command          | Purpose                     |
| ---------------- | --------------------------- |
| `jb run <cmd>`   | Start background job        |
| `jb list`        | List jobs (current project) |
| `jb logs <id>`   | View output                 |
| `jb status <id>` | Job details                 |
| `jb stop <id>`   | Stop job                    |
| `jb wait <id>`   | Block until done            |
| `jb retry <id>`  | Re-run job                  |
| `jb clean`       | Remove old jobs             |

## Features

- Short memorable IDs (`a3x9`)
- Per-project job tracking (via git root)
- JSON output (`--json`)
- Survives terminal disconnect
- Auto-starts daemon

## For AI Agents

```bash
jb skills install  # installs to ~/.claude/skills/jb/
```

## License

MIT
