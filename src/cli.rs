use std::path::PathBuf;

#[derive(Default)]
pub struct CliOverrides {
    pub origin: Option<String>,
    pub message: Option<String>,
    pub submessage: Option<String>,
    pub theme_mode: Option<String>,
    pub accent: Option<String>,
    pub primary: Option<String>,
    pub secondary: Option<String>,
    pub tertiary: Option<String>,
    pub background1: Option<String>,
    pub background2: Option<String>,
    pub background3: Option<String>,
    pub hero1: Option<String>,
    pub hero2: Option<String>,
    pub source_will_update: Option<String>,
}

pub enum OutputMode {
    Tui,
    Oneline,
    Json,
}

pub struct CliArgs {
    pub config_path: Option<PathBuf>,
    pub show_help: bool,
    pub overrides: CliOverrides,
    pub output_mode: OutputMode,
}

pub fn parse_cli<I>(args: I) -> CliArgs
where
    I: IntoIterator<Item = String>,
{
    let mut iter = args.into_iter();
    let mut positional: Option<PathBuf> = None;
    let mut show_help = false;
    let mut overrides = CliOverrides::default();
    let mut output_mode = OutputMode::Tui;
    let override_flags: [(&str, fn(&mut CliOverrides, String)); 13] = [
        ("--message", set_message),
        ("--submessage", set_submessage),
        ("--origin", set_origin),
        ("--mode", set_theme_mode),
        ("--accent", set_accent),
        ("--primary", set_primary),
        ("--secondary", set_secondary),
        ("--tertiary", set_tertiary),
        ("--background1", set_background1),
        ("--background2", set_background2),
        ("--background3", set_background3),
        ("--hero1", set_hero1),
        ("--hero2", set_hero2),
    ];

    while let Some(arg) = iter.next() {
        if arg == "--help" || arg == "-h" {
            show_help = true;
            continue;
        }
        if arg == "--oneline" {
            output_mode = OutputMode::Oneline;
            continue;
        }
        if arg == "--json" {
            output_mode = OutputMode::Json;
            continue;
        }
        if let Some(value) = take_value(&arg, "--config-file", &mut iter) {
            return CliArgs {
                config_path: Some(PathBuf::from(value)),
                show_help,
                overrides,
                output_mode,
            };
        }
        let mut matched_override = false;
        for (flag, setter) in override_flags {
            if let Some(value) = take_value(&arg, flag, &mut iter) {
                setter(&mut overrides, value);
                matched_override = true;
                break;
            }
        }
        if matched_override {
            continue;
        }
        if let Some(value) = take_value(&arg, "--source-will-update", &mut iter) {
            overrides.source_will_update = Some(value);
            continue;
        }
        if positional.is_none() && !arg.starts_with('-') {
            positional = Some(PathBuf::from(arg));
        }
    }

    CliArgs {
        config_path: positional,
        show_help,
        overrides,
        output_mode,
    }
}

fn take_value<I>(arg: &str, flag: &str, iter: &mut I) -> Option<String>
where
    I: Iterator<Item = String>,
{
    if arg == flag {
        return iter.next();
    }
    let prefix = format!("{flag}=");
    arg.strip_prefix(&prefix).map(|value| value.to_string())
}

fn set_message(overrides: &mut CliOverrides, value: String) {
    overrides.message = Some(value);
}

fn set_submessage(overrides: &mut CliOverrides, value: String) {
    overrides.submessage = Some(value);
}

fn set_origin(overrides: &mut CliOverrides, value: String) {
    overrides.origin = Some(value);
}

fn set_theme_mode(overrides: &mut CliOverrides, value: String) {
    overrides.theme_mode = Some(value);
}

fn set_accent(overrides: &mut CliOverrides, value: String) {
    overrides.accent = Some(value);
}

fn set_primary(overrides: &mut CliOverrides, value: String) {
    overrides.primary = Some(value);
}

fn set_secondary(overrides: &mut CliOverrides, value: String) {
    overrides.secondary = Some(value);
}

fn set_tertiary(overrides: &mut CliOverrides, value: String) {
    overrides.tertiary = Some(value);
}

fn set_background1(overrides: &mut CliOverrides, value: String) {
    overrides.background1 = Some(value);
}

fn set_background2(overrides: &mut CliOverrides, value: String) {
    overrides.background2 = Some(value);
}

fn set_background3(overrides: &mut CliOverrides, value: String) {
    overrides.background3 = Some(value);
}

fn set_hero1(overrides: &mut CliOverrides, value: String) {
    overrides.hero1 = Some(value);
}

fn set_hero2(overrides: &mut CliOverrides, value: String) {
    overrides.hero2 = Some(value);
}

pub fn print_help() {
    println!(
        "Skyset (TUI prototype)\n\nUSAGE:\n  skyset [PATH]\n  skyset --config-file <PATH>\n\nOPTIONS:\n  --config-file <PATH>       Use an explicit config file\n  --message <TEXT>           Override message text\n  --submessage <TEXT>        Override submessage text\n  --origin <ID>              Override origin identifier\n  --mode <dark|light|system> Override theme mode\n  --accent <HEX>             Override accent color\n  --primary <HEX>            Override palette primary color\n  --secondary <HEX>          Override palette secondary color\n  --tertiary <HEX>           Override palette tertiary color\n  --background1 <HEX>        Override background gradient stop #1\n  --background2 <HEX>        Override background gradient stop #2\n  --background3 <HEX>        Override background gradient stop #3\n  --hero1 <HEX>              Override hero gradient stop #1\n  --hero2 <HEX>              Override hero gradient stop #2\n  --source-will-update <BOOL>Override source_will_update\n  --oneline                  Print a one-line summary and exit\n  --json                     Print JSON status and exit\n  -h, --help                 Show this help message\n\nIf PATH is a directory, skyset will normalize it to latest.yml."
    );
}
