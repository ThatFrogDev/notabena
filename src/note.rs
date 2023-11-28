use crate::{
    cursor_to_origin,
    database::{display::display, get_notes::get_notes},
    prompts::{confirm::confirm, input::input, select::select},
    truncated_note,
};
use async_std::path::PathBuf;
use chrono::prelude::Local;
use rusqlite::{params, Connection};

#[derive(Clone)]
pub struct Note {
    pub id: usize,
    pub name: String,
    pub content: String,
    pub created: String,
}

impl Note {
    pub fn create(db_file: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let mut inputted_note = Note {
            id: get_notes(db_file)?.len(),
            name: input("Name:", "".to_string()),
            content: input("Content:", "".to_string()),
            created: format!("{}", Local::now().format("%A %e %B, %H:%M")),
        };

        cursor_to_origin()?;
        println!("This is the note you're about to create:");
        display(&mut inputted_note)?;

        match confirm("Do you want to save this note?") {
            true => {
                Connection::open(db_file)?.execute(
                    "INSERT INTO saved_notes (id, name, content, created) VALUES (?1, ?2, ?3, ?4);",
                    params![
                        &inputted_note.id,
                        &inputted_note.name,
                        &inputted_note.content,
                        &inputted_note.created
                    ],
                )?;

                cursor_to_origin()?;
                println!("Note created successfully.");
                Ok(())
            }
            false => {
                cursor_to_origin()?;
                Ok(())
            }
        }
    }

    fn show(db_file: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let saved_notes = get_notes(db_file)?;

        if saved_notes.is_empty() {
            println!("You don't have any notes.");
            return Ok(());
        }

        let mut options: Vec<String> = Vec::new();
        truncated_note(&mut options, db_file)?;
        let selection = select("Select the note that you want to view:", &options);
        let mut selected_note = &saved_notes[selection];

        display(&mut selected_note)?;
        Ok(())
    }

    pub fn edit(db_file: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let saved_notes = get_notes(db_file)?;
        let mut options: Vec<String> = Vec::new();
        truncated_note(&mut options, db_file)?;
        let selection = select("Select the note that you want to edit:", &options);
        let selected_note = &saved_notes[selection];
        let updated_note = Note {
            id: selection,
            name: input("Name:", selected_note.name.clone()),
            content: input("Content:", selected_note.content.clone()),
            created: selected_note.created.clone(),
        };

        if saved_notes.is_empty() {
            cursor_to_origin()?;
            println!("You can't edit notes, because there are none.");
        }

        match confirm("Are you sure that you want to edit this note?") {
            true => {
                Connection::open(db_file)?.execute(
                    "UPDATE saved_notes
                        SET (name) = ?1, (content) = ?2
                        WHERE id = ?3;",
                    params![&selected_note.name, &selected_note.content, &selection],
                )?;

                cursor_to_origin()?;
                println!("Note updated successfully.");
                Ok(())
            }
            false => Ok(()),
        }
    }
}
