use crate::model::{Skyset, ThemeMode, theme_mode_label};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldId {
    Origin,
    Message,
    Submessage,
    ThemeMode,
    Accent,
    Primary,
    Secondary,
    Tertiary,
    Background1,
    Background2,
    Background3,
    Hero1,
    Hero2,
    SourceWillUpdate,
}

const FIELD_ORDER: [FieldId; 14] = [
    FieldId::Origin,
    FieldId::Message,
    FieldId::Submessage,
    FieldId::ThemeMode,
    FieldId::Accent,
    FieldId::Primary,
    FieldId::Secondary,
    FieldId::Tertiary,
    FieldId::Background1,
    FieldId::Background2,
    FieldId::Background3,
    FieldId::Hero1,
    FieldId::Hero2,
    FieldId::SourceWillUpdate,
];

impl FieldId {
    pub fn label(self) -> &'static str {
        match self {
            FieldId::Origin => "Origin",
            FieldId::Message => "Message",
            FieldId::Submessage => "Submessage",
            FieldId::ThemeMode => "Theme mode",
            FieldId::Accent => "Accent",
            FieldId::Primary => "Primary",
            FieldId::Secondary => "Secondary",
            FieldId::Tertiary => "Tertiary",
            FieldId::Background1 => "Background #1",
            FieldId::Background2 => "Background #2",
            FieldId::Background3 => "Background #3",
            FieldId::Hero1 => "Hero #1",
            FieldId::Hero2 => "Hero #2",
            FieldId::SourceWillUpdate => "Source will update",
        }
    }

    pub fn is_toggle(self) -> bool {
        matches!(self, FieldId::ThemeMode | FieldId::SourceWillUpdate)
    }
}

pub struct EditorState {
    selected: usize,
    input: String,
}

impl EditorState {
    pub fn new(skyset: &Skyset) -> Self {
        let selected = 0;
        let input = field_value(skyset, FIELD_ORDER[selected]);
        Self { selected, input }
    }

    pub fn fields(&self) -> &[FieldId] {
        &FIELD_ORDER
    }

    pub fn selected(&self) -> usize {
        self.selected
    }

    pub fn input(&self) -> &str {
        &self.input
    }

    pub fn set_input(&mut self, value: String) {
        self.input = value;
    }

    pub fn push_char(&mut self, value: char) {
        self.input.push(value);
    }

    pub fn pop_char(&mut self) {
        self.input.pop();
    }

    pub fn current_field(&self) -> FieldId {
        FIELD_ORDER[self.selected]
    }

    pub fn next(&mut self, skyset: &Skyset) {
        self.selected = (self.selected + 1) % FIELD_ORDER.len();
        self.input = field_value(skyset, self.current_field());
    }

    pub fn previous(&mut self, skyset: &Skyset) {
        self.selected = (self.selected + FIELD_ORDER.len() - 1) % FIELD_ORDER.len();
        self.input = field_value(skyset, self.current_field());
    }
}

pub fn is_editing(editor: &EditorState, skyset: &Skyset) -> bool {
    editor.input != field_value(skyset, editor.current_field())
}

pub fn field_value(skyset: &Skyset, field: FieldId) -> String {
    match field {
        FieldId::Origin => skyset.origin.clone(),
        FieldId::Message => skyset.message.clone(),
        FieldId::Submessage => skyset.submessage.clone(),
        FieldId::ThemeMode => theme_mode_label(skyset.theme.mode).to_string(),
        FieldId::Accent => skyset.theme.accent.clone(),
        FieldId::Primary => skyset.palette.primary.clone(),
        FieldId::Secondary => skyset.palette.secondary.clone(),
        FieldId::Tertiary => skyset.palette.tertiary.clone(),
        FieldId::Background1 => skyset
            .gradients
            .background
            .get(0)
            .cloned()
            .unwrap_or_default(),
        FieldId::Background2 => skyset
            .gradients
            .background
            .get(1)
            .cloned()
            .unwrap_or_default(),
        FieldId::Background3 => skyset
            .gradients
            .background
            .get(2)
            .cloned()
            .unwrap_or_default(),
        FieldId::Hero1 => skyset.gradients.hero.get(0).cloned().unwrap_or_default(),
        FieldId::Hero2 => skyset.gradients.hero.get(1).cloned().unwrap_or_default(),
        FieldId::SourceWillUpdate => skyset.source_will_update.to_string(),
    }
}

pub fn set_field_value(skyset: &mut Skyset, field: FieldId, value: String) -> Result<(), String> {
    match field {
        FieldId::Origin => skyset.origin = value,
        FieldId::Message => skyset.message = value,
        FieldId::Submessage => skyset.submessage = value,
        FieldId::Accent => skyset.theme.accent = normalize_hex(&value)?,
        FieldId::Primary => skyset.palette.primary = normalize_hex(&value)?,
        FieldId::Secondary => skyset.palette.secondary = normalize_hex(&value)?,
        FieldId::Tertiary => skyset.palette.tertiary = normalize_hex(&value)?,
        FieldId::Background1 => set_gradient(&mut skyset.gradients.background, 0, value)?,
        FieldId::Background2 => set_gradient(&mut skyset.gradients.background, 1, value)?,
        FieldId::Background3 => set_gradient(&mut skyset.gradients.background, 2, value)?,
        FieldId::Hero1 => set_gradient(&mut skyset.gradients.hero, 0, value)?,
        FieldId::Hero2 => set_gradient(&mut skyset.gradients.hero, 1, value)?,
        FieldId::ThemeMode => {
            skyset.theme.mode = match value.to_lowercase().as_str() {
                "dark" => ThemeMode::Dark,
                "light" => ThemeMode::Light,
                "system" => ThemeMode::System,
                _ => return Err("Theme mode must be dark, light, or system".to_string()),
            };
        }
        FieldId::SourceWillUpdate => {
            skyset.source_will_update =
                matches!(value.to_lowercase().as_str(), "true" | "1" | "yes");
        }
    }
    Ok(())
}

pub fn toggle_field(skyset: &mut Skyset, field: FieldId) {
    match field {
        FieldId::ThemeMode => {
            skyset.theme.mode = match skyset.theme.mode {
                ThemeMode::Dark => ThemeMode::Light,
                ThemeMode::Light => ThemeMode::System,
                ThemeMode::System | ThemeMode::Unknown => ThemeMode::Dark,
            };
        }
        FieldId::SourceWillUpdate => {
            skyset.source_will_update = !skyset.source_will_update;
        }
        _ => {}
    }
}

pub fn normalize_hex(value: &str) -> Result<String, String> {
    let trimmed = value.trim().trim_start_matches('#');
    if trimmed.len() != 6 || !trimmed.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err("Invalid hex color".to_string());
    }
    Ok(format!("#{}", trimmed.to_uppercase()))
}

fn set_gradient(gradient: &mut Vec<String>, index: usize, value: String) -> Result<(), String> {
    let normalized = normalize_hex(&value)?;
    if gradient.len() <= index {
        gradient.resize(index + 1, "#000000".to_string());
    }
    gradient[index] = normalized;
    Ok(())
}
