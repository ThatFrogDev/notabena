#[cfg(test)]
use crate::{api, note::Note};
use chrono::prelude::Local;
use async_std::path::PathBuf;
use tempfile::tempdir;

#[test]
fn test_db_init() -> Result<(), Box<dyn std::error::Error>> {
  let dir = tempdir()?;
  let data_directory = PathBuf::from(dir.path());
  let db_file = data_directory.join("notes.db");
  assert!(api::init_db(&data_directory, &db_file).is_ok());
  Ok(())
}

#[test]
fn test_save_note() -> Result<(), Box<dyn std::error::Error>> {
  let dir = tempdir()?;
  let data_directory = PathBuf::from(dir.path());
  let db_file: PathBuf = data_directory.join("notes.db");
  api::init_db(&data_directory, &db_file)?;

  let note = Note {
    id: 0,
    name: "Test".to_string(),
    content: "Test".to_string(),
    created: format!("{}", Local::now().format("%A %e %B, %H:%M")),
  };

  assert!(api::save_note(&note, &db_file).is_ok());
  Ok(())
}

#[test]
fn test_delete_notes() {
  let dir = tempdir().unwrap();
  let data_directory = PathBuf::from(dir.path());
  let db_file = data_directory.join("notes.db");
  api::init_db(&data_directory, &db_file).unwrap();

  let note = Note {
    id: 0,
    name: "Test".to_string(),
    content: "Test".to_string(),
    created: format!("{}", Local::now().format("%A %e %B, %H:%M")),
  };

  api::save_note(&note, &db_file).unwrap();
  let notes = api::get_notes(&db_file).unwrap();
  let ids: Vec<usize> = notes.iter().map(|note| note.id).collect();
  api::delete_notes(ids, &db_file).unwrap();
  assert!(api::get_notes(&db_file).unwrap().is_empty());
}

#[test]
fn test_get_notes() -> Result<(), Box<dyn std::error::Error>> {
  let dir = tempdir()?;
  let data_directory = PathBuf::from(dir.path());
  let db_file = data_directory.join("notes.db");
  api::init_db(&data_directory, &db_file)?;

  let note = Note {
    id: 0,
    name: "Test".to_string(),
    content: "Test".to_string(),
    created: format!("{}", Local::now().format("%A %e %B, %H:%M")),
  };

  api::save_note(&note, &db_file)?;
  assert!(!api::get_notes(&db_file)?.is_empty());
  Ok(())
}