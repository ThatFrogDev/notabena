use dialoguer::{Input, theme::ColorfulTheme};

fn input(prompt: &str, initial_text: String) -> String {
    match initial_text.as_str() {
        "" => Input::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .interact_text()
            .unwrap(),
        _ => Input::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .with_initial_text(initial_text)
            .interact_text()
            .unwrap(),
    }
}