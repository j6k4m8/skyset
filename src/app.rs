use std::{path::PathBuf, time::Duration};

use chrono::{SecondsFormat, Utc};

use crate::{
    cli::CliOverrides,
    editor::{self, EditorState, FieldId},
    io::{self, ReadOutcome},
    model::{SCHEMA_VERSION, Skyset},
};

pub struct App {
    path: PathBuf,
    skyset: Skyset,
    editor: EditorState,
    last_raw: String,
    last_poll: std::time::Instant,
    poll_interval: Duration,
}

impl App {
    pub fn new(path: PathBuf) -> Self {
        let outcome = io::load_initial_state(&path);
        let editor = EditorState::new(&outcome.skyset);
        Self {
            path,
            skyset: outcome.skyset,
            editor,
            last_raw: outcome.raw,
            last_poll: std::time::Instant::now(),
            poll_interval: Duration::from_secs(30),
        }
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn skyset(&self) -> &Skyset {
        &self.skyset
    }

    pub fn editor(&self) -> &EditorState {
        &self.editor
    }

    pub fn poll_interval(&self) -> Duration {
        self.poll_interval
    }

    pub fn last_poll_elapsed(&self) -> Duration {
        self.last_poll.elapsed()
    }

    pub fn apply_outcome(&mut self, outcome: ReadOutcome) {
        if outcome.raw != self.last_raw {
            self.skyset = outcome.skyset;
            self.sync_editor_input();
            self.last_raw = outcome.raw;
        }
    }

    pub fn reload_from_disk(&mut self) {
        self.last_poll = std::time::Instant::now();
        match io::read_skyset(&self.path) {
            Ok(outcome) => self.apply_outcome(outcome),
            Err(_) => {
                self.skyset = Skyset::default();
                self.last_raw = String::new();
            }
        }
    }

    pub fn editor_next(&mut self) {
        self.editor.next(&self.skyset);
    }

    pub fn editor_previous(&mut self) {
        self.editor.previous(&self.skyset);
    }

    pub fn push_char(&mut self, value: char) {
        self.editor.push_char(value);
    }

    pub fn pop_char(&mut self) {
        self.editor.pop_char();
    }

    pub fn is_editing(&self) -> bool {
        editor::is_editing(&self.editor, &self.skyset)
    }

    pub fn apply_editor_input(&mut self) {
        let field = self.editor.current_field();
        if field.is_toggle() {
            editor::toggle_field(&mut self.skyset, field);
            self.sync_editor_input();
            return;
        }

        let value = self.editor.input().trim().to_string();
        let _ = editor::set_field_value(&mut self.skyset, field, value);
        self.sync_editor_input();
    }

    pub fn save(&mut self) {
        self.skyset.updated_at = Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true);
        self.skyset.version = SCHEMA_VERSION;

        let serialized = match serde_yaml::to_string(&self.skyset) {
            Ok(value) => value,
            Err(_) => return,
        };

        if io::write_skyset(&self.path, &serialized).is_err() {
            return;
        }

        self.last_raw = serialized;
    }

    pub fn reset(&mut self) {
        self.skyset = Skyset::default();
        self.sync_editor_input();
    }

    pub fn apply_overrides(&mut self, overrides: &CliOverrides) {
        set_string_override(&mut self.skyset.origin, &overrides.origin);
        set_string_override(&mut self.skyset.message, &overrides.message);
        set_string_override(&mut self.skyset.submessage, &overrides.submessage);

        for (field, value) in [
            (FieldId::ThemeMode, &overrides.theme_mode),
            (FieldId::Accent, &overrides.accent),
            (FieldId::Primary, &overrides.primary),
            (FieldId::Secondary, &overrides.secondary),
            (FieldId::Tertiary, &overrides.tertiary),
            (FieldId::Background1, &overrides.background1),
            (FieldId::Background2, &overrides.background2),
            (FieldId::Background3, &overrides.background3),
            (FieldId::Hero1, &overrides.hero1),
            (FieldId::Hero2, &overrides.hero2),
            (FieldId::SourceWillUpdate, &overrides.source_will_update),
        ] {
            apply_override(&mut self.skyset, field, value);
        }

        self.sync_editor_input();
    }

    fn sync_editor_input(&mut self) {
        self.editor.set_input(editor::field_value(
            &self.skyset,
            self.editor.current_field(),
        ));
    }
}

fn apply_override(skyset: &mut Skyset, field: FieldId, value: &Option<String>) {
    if let Some(value) = value {
        let _ = editor::set_field_value(skyset, field, value.clone());
    }
}

fn set_string_override(target: &mut String, value: &Option<String>) {
    if let Some(value) = value {
        *target = value.clone();
    }
}
