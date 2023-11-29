use dialoguer::{theme::ColorfulTheme, MultiSelect};

pub fn multiselect(prompt: &str, options: Vec<String>) -> Vec<usize> {
    MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .items(&options)
        .interact()
        .unwrap()
}
