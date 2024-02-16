use async_std::path::PathBuf;
use crate::api::get_notes;

pub fn truncate_note(
  options: &mut Vec<String>,
  db_file: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
  for note in &get_notes(db_file)? {
      let mut truncated_content: String = note.content.chars().take(10).collect();
      if truncated_content.chars().count() == 10 {
          truncated_content += "..."; // this is cursed and awesome at the same time
      }

      options.push(format!(
          "{} | {} | {}",
          note.name, truncated_content, note.created
      ));
  }
  Ok(())
}
