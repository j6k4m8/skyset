# Skyset Overview

Skyset is a tiny, file-based contract for sharing a live theme between apps. One producer writes `latest.yml`; any number of consumers read it.

## Behaviors

-   Read on launch; missing/empty file means “no signal.”
-   Only save on explicit user action (no auto-write on edit).
-   Set `updated_at` on save (UTC RFC3339).
-   Parse leniently: missing fields fall back to defaults; unknown fields are ignored.

## File location

Canonical path: `~/.config/skyset/latest.yml`.

Directory normalization:

-   `~/` → `~/.config/skyset/latest.yml`
-   `~/.config` → `~/.config/skyset/latest.yml`
-   `~/.config/skyset` → `~/.config/skyset/latest.yml`
