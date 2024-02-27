use crate::return_to_main::{continue_event, return_event};
use crate::{main, return_to_main};
use dialoguer::{theme::ColorfulTheme, Input};

pub fn input(prompt: &str, initial_text: String) -> Result<String, Box<dyn std::error::Error>> {
    let result = match initial_text.as_str() {
        "" => Input::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .interact_text()
            .unwrap(),
        _ => Input::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .with_initial_text(initial_text)
            .interact_text()
            .unwrap(),
    };

    match return_to_main() {
        Ok(value) => {
            if value == return_event() {
                main()?;
            } else if value == continue_event() {
                return Ok(result);
            }
        }
        _ => return Ok(result),
    }

    Ok(result)
}
