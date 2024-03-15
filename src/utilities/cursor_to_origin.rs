use std::process::Command;

#[cfg(target_os = "windows")]
pub fn cursor_to_origin() -> Result<(), Box<dyn std::error::Error>> {
    Command::new("cmd").args(["/c", "cls"]).spawn()?.wait()?;
    Ok(())
}

#[cfg(not(target_os = "windows"))]
pub fn cursor_to_origin() -> Result<(), Box<dyn std::error::Error>> {
    Command::new("clear").spawn()?.wait()?;
    Ok(())
}
