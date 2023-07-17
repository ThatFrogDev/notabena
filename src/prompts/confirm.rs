use dialoguer::{theme::ColorfulTheme, Confirm};

pub fn confirm(prompt: &str) -> bool {
    Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .default(true)
        .interact()
        .unwrap()
}
