# branchgen

A TUI application for generating Git branch names, commit messages, and PR titles from user inputs. Built with Rust and ratatui.

## Features

- **Dynamic form** — fields are fully configurable via a TOML config file
- **Smart generation** — generates branch name, commit message, and PR title from your inputs
- **History** — every generation is saved to a JSON file and browsable in the app
- **Git integration** — create or checkout branches directly from the app
- **Clipboard** — copy any generated value with a single keystroke
- **Persistent fields** — frequently used values (team, PI, IT) are remembered between sessions
- **Cross-platform** — works on Windows, macOS, and Linux

## Installation

### From source

```bash
git clone <your-repo-url>
cd branchgen
cargo install --path .
```

### Manual (pre-built binary)

Download the binary for your platform, place it in a folder in your `PATH`, and run `branchgen`.

## Setup

On first launch, a default config file will be created automatically at:

- **Linux/macOS**: `~/.config/branchgen/config.toml`
- **Windows**: `C:\Users\<name>\AppData\Roaming\branchgen\config.toml`

You can also generate the config manually:

```bash
branchgen --init
```

Edit the config file to match your team's conventions before launching the app.

## Configuration

The config file defines your form fields and output formats.

```toml
# Available field types: text, number, select
# Keys defined here must match the variables in your formats: {my_key}

[[fields]]
key        = "team"
label      = "Team"
type       = "text"
required   = true
persistent = true      # value is remembered between sessions
normalize  = "uppercase" # automatically uppercased

[[fields]]
key        = "pi"
label      = "PI"
type       = "number"
required   = true
persistent = true

[[fields]]
key        = "story_type"
label      = "Story Type"
type       = "select"
values     = ["feature", "bugfix", "hotfix", "release"]
required   = true

[[fields]]
key        = "story_title"
label      = "Story Title"
type       = "text"
required   = true
normalize  = "spaces"  # spaces replaced by hyphens

[formats]
branch   = "{story_type}/{pi}_{team}_{story_title}"
commit   = "{team} [{pi}] - {story_type}: details to update"
pr_title = "{story_type}: {pi}_{team}_{story_title}"
```

### Field options

| Option | Type | Description |
|--------|------|-------------|
| `key` | string | Unique identifier, used in format templates as `{key}` |
| `label` | string | Display name shown in the form |
| `type` | `text` \| `number` \| `select` | Field type |
| `required` | bool | Whether the field must be filled before generating |
| `persistent` | bool | Whether the value is saved between sessions |
| `normalize` | string | Value transformation (see below) |
| `values` | array | List of options for `select` fields |

### Normalize options

| Value | Description |
|-------|-------------|
| `none` | No transformation (default) |
| `spaces` | Replace spaces with hyphens |
| `uppercase` | Force uppercase |
| `lowercase` | Force lowercase |
| `lcfirst` | First letter lowercase |
| `ucfirst` | First letter uppercase |

## Usage

```bash
# Launch in the current directory (must be a git repo)
branchgen

# Launch in a specific directory
branchgen /path/to/repo

# Generate a default config file
branchgen --init
```

## Keyboard shortcuts

### Form

| Key | Action |
|-----|--------|
| `↑↓` | Navigate between fields |
| `←→` | Navigate select options |
| `Enter` | Next field / Generate (on the Generate button) |
| `Ctrl+r` | Reset form |
| `Tab` | Next screen |
| `BackTab` | Previous screen |
| `q` | Quit |

### Results

| Key | Action |
|-----|--------|
| `↑↓` | Select a line |
| `c` | Copy selected line to clipboard |
| `b` | Create git branch |
| `Tab` | Next screen |
| `BackTab` | Previous screen |
| `q` | Quit |

### History

| Key | Action |
|-----|--------|
| `↑↓` | Navigate lines |
| `c` | Copy selected line to clipboard |
| `b` | Create branch from history |
| `Enter` | Checkout branch from history |
| `Tab` | Next screen |
| `BackTab` | Previous screen |
| `q` | Quit |

## Data storage

| File | Location | Description |
|------|----------|-------------|
| `config.toml` | `~/.config/branchgen/` | App configuration |
| `history.json` | `~/.config/branchgen/` | Generation history |
| `persistent.json` | `~/.config/branchgen/` | Persistent field values |

## Built with

- [ratatui](https://github.com/ratatui-org/ratatui) — TUI framework
- [crossterm](https://github.com/crossterm-rs/crossterm) — Cross-platform terminal manipulation
- [serde](https://serde.rs/) — Serialization/deserialization
- [clap](https://github.com/clap-rs/clap) — CLI argument parsing
- [chrono](https://github.com/chronotope/chrono) — Date formatting
- [indexmap](https://github.com/indexmap-rs/indexmap) — Ordered map for consistent field ordering
