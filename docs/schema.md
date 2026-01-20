# Schema (latest.yml)

This is the canonical schema for `latest.yml`. Fields are optional unless stated otherwise. Missing fields should fall back to defaults; unknown fields should be ignored.

```yaml
_version: 1
origin: com.example.producer
updated_at: "2026-01-19T03:12:00Z"
message: "Album art for Purple Songs"
submessage: "My Fun Song - Now Playing"
source_will_update: true

theme:
    mode: system # or dark/light
    accent: "#7C4DFF"

palette:
    primary: "#0E0E10"
    secondary: "#1F1F23"
    tertiary: "#2E2E35"

gradients:
    background: ["#0B0B12", "#141424", "#1D1D32"]
    hero: ["#7C4DFF", "#5E35B1"]
```

Recall you can also encode nested arrays like this:

```yml
# ...
gradients:
    background:
        - "#0B0B12"
        - "#141424"
```

# `_version` (integer, required)

The schema version. Current version is `1`.

# `origin` (string, required)

An identifier string for the producing application (e.g., `com.matelsky.coppelia`). Can also be, say, a hostname or URL, or any other string that helps consumers identify the source of the data (`"manual"` or whatever).

# `updated_at` (string, required)

The UTC timestamp (RFC3339) of when the file was last saved by the producer.

# `message` (string)

A message to communicate if the consumer app supports it.

# `submessage` (string)

A secondary message to communicate if the consumer app supports it.

# `source_will_update` (boolean)

Whether the producer app expects to update the file again soon. Consumers can use this to decide whether to poll for updates more frequently.

Note that this is just a hint; consumers should not rely on it for correctness, since producers may misreport it or the user may stop the producer app before it updates skyset again.

# `theme`, `palette`, `gradients` (objects)

An object defining theme-related properties. These colors can be used by consumer apps to style their UIs in a way that matches the producer app; there are no guarantees made about contrast, lightness, or other properties, so consumers should apply their own logic to ensure usability.

In general, `accent` is a highlight color, `primary`/`secondary`/`tertiary` are background or surface colors, and `background`/`hero` colors or gradients can be used for large areas or headers.
