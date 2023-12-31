use dialoguer::{theme::ColorfulTheme, Select};

pub fn select<T: AsRef<str> + std::fmt::Display>(prompt: &str, options: &[T]) -> usize {
    Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .items(options)
        .interact()
        .unwrap()
}
