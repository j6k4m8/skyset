use crate::{app::App, color::parse_hex_rgb};

pub fn print_oneline(app: &App) {
    let accent_block = color_block(&app.skyset().theme.accent);
    let primary_block = color_block(&app.skyset().palette.primary);
    let secondary_block = color_block(&app.skyset().palette.secondary);
    let tertiary_block = color_block(&app.skyset().palette.tertiary);
    let background_blocks = gradient_blocks(&app.skyset().gradients.background);
    let hero_blocks = gradient_blocks(&app.skyset().gradients.hero);
    let palette_blocks = format!("{}{}{}", primary_block, secondary_block, tertiary_block);
    println!(
        "{} {} | msg=\"{}\" | palette: {} | background: {} | hero: {}",
        app.path().display(),
        accent_block,
        app.skyset().message,
        palette_blocks,
        background_blocks,
        hero_blocks
    );
}

pub fn print_json(app: &App) -> Result<(), String> {
    serde_json::to_string_pretty(app.skyset())
        .map(|payload| {
            println!("{}", payload);
        })
        .map_err(|err| err.to_string())
}

fn color_block(value: &str) -> String {
    if let Some((r, g, b)) = parse_hex_rgb(value) {
        format!("\x1b[48;2;{r};{g};{b}m  \x1b[0m")
    } else {
        "??".to_string()
    }
}

fn gradient_blocks(values: &[String]) -> String {
    values
        .iter()
        .map(|value| color_block(value))
        .collect::<Vec<_>>()
        .join("")
}
