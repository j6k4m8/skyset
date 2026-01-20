# Skyset

Skyset is a tiny, file-based contract for sharing a live visual theme between apps. One **producer** writes `latest.yml`; many **consumers** read it to sync their look. It keeps coupling low: no IPC, just a shared file.

This repo is a TUI editor + preview tool and a reference implementation for the format.

<img width="829" height="505" alt="Image" src="https://github.com/user-attachments/assets/a2c412bc-5df0-47a4-afd9-53b2f0680f7c" />

## Example Use-Cases

-   The [Coppelia music player](https://blog.jordan.matelsky.com/coppelia/) sets a system-wide theme based on the currently playing album art
-   Get the current weather conditions outside and set your terminal colors and wallpaper accordingly
-   Coordinate your apps' themes to match images on your desktop
-   Shift your whole environment gradually to match the time of day

## Docs

-   [`docs/overview.md`](docs/overview.md) – Concise overview
-   [`docs/schema.md`](docs/schema.md) – Canonical schema for `latest.yml`
-   [`docs/tui.md`](docs/tui.md) – TUI layout and interaction model

## Quick start

```zsh
cargo run
```

## Usage

Skyset defaults to `~/.config/skyset/latest.yml` and polls the file every 30 seconds (the UI refreshes every second).

**TUI Editor**

```zsh
cargo run
```

**One-line status output**

```zsh
cargo run -- --oneline
```

**JSON status output**

```zsh
cargo run -- --json
```

**Help**

```zsh
cargo run -- --help
```
