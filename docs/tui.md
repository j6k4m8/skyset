# TUI layout & interaction

## Layout

**Left (Preview panel)**

**Right (Editor panel)**

-   Metadata fields: origin, message, submessage
-   Toggle for `theme.mode` (dark/light)
-   Color pickers for accent + palette (hex input + visual picker + sliders)
-   Gradient stop editors (hex input per stop)

**Bottom status line**

-   File path
-   `updated_at`
-   `source_will_update`
-   Refresh countdown (polling every 30s)
-   Last read/parse status

## Interaction model

-   Read on launch; missing/empty file does not create a file.
-   Save explicitly with a “Save” action (no auto-write on edit).
-   Reload from disk on demand with `r` (unless actively editing a field).
-   Reset to defaults with `x`.

## **Key bindings**

-   `Tab` / `Shift+Tab`: move between fields
-   `Enter`: apply the current field value
-   `s`: save to `latest.yml`
-   `r`: reload from disk (unless you are actively editing)
-   `x`: reset to defaults
-   `q`: quit

## **Run with the default path**

```zsh
cargo run
```

## **Normalize a directory**

```zsh
cargo run -- ~/.config/skyset
```

## **Explicit config file**

```zsh
cargo run -- --config-file ~/.config/skyset/latest.yml
```

## **Apply overrides from the CLI**

```zsh
cargo run -- --message "hi there" --accent "#dedede"
```
