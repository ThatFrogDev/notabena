use crossterm::terminal::disable_raw_mode;

struct Clean;

impl Drop for Clean {
    fn drop(&mut self) {
        disable_raw_mode().expect("Unable to disable raw mode")
    }
}
