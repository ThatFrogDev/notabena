use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::time::Duration;

pub fn return_to_main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;

    loop {
        if event::poll(Duration::from_millis(1000))? {
            if let Event::Key(event) = event::read()? {
                match event {
                    KeyEvent {
                        code: KeyCode::Char('r'),
                        modifiers: event::KeyModifiers::CONTROL,
                        kind: _,
                        state: _,
                    } => return Ok(()),
                    _ => (),
                }
            }
        }

        disable_raw_mode()?;
    }
}
