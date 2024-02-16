use std::process::Command;

pub fn cursor_to_origin() -> Result<(), Box<dyn std::error::Error>> {
  if cfg!(target_os = "windows") {
      Command::new("cmd").args(["/c", "cls"]).spawn()?.wait()?;
      Ok(())
  } else {
      Command::new("clear").spawn()?.wait()?;
      Ok(())
  }
}