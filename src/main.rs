// TODO: Organize the code into different files
pub mod api;

use chrono::prelude::*;
use inquire::{Confirm, Select, MultiSelect, Text};

pub struct Note {
    id: usize,
    name: String,
    content: String,
    created: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    api::init_db()?;
    println!("Welcome to Notabena, the FOSS note-taking app.");

    loop {
        cursor_to_origin();

        let options = vec!["New note", "View note", "Edit note", "Delete note", "Exit"];
        let select = Select::new("What do you want to do?", options).prompt();

        match select {
            Ok("New note") => {
                new_note().expect("Creating a new note failed");
                cursor_to_origin();
            },
            Ok("View note") => {
                show_notes()?;
                cursor_to_origin();
            },
            Ok("Edit note") => {
                edit_notes().expect("Editing the note failed");
                cursor_to_origin();
            },
            Ok("Delete note") => {
                delete_notes().expect("Deleting the note failed");
                cursor_to_origin();
            },
            Ok(_) => {
                return Ok(());
            },
            Err(e) => {
                println!("There was an error: {}", e);
            },
        }
    }
}

fn new_note() -> Result<(), Box<dyn std::error::Error>> {
    let saved_notes = api::get_notes()?;
    let inputted_note = Note {
        id: saved_notes.len(),
        name: Text::new("Name:").prompt()?,
        content: Text::new("Content:").prompt()?,
        created: format!("{}", Local::now().format("%A %e %B, %H:%M").to_string()),
    };

    cursor_to_origin();
    println!("This is the note you're about to create:");
    display_note(&inputted_note);

    let save_note_bool = Confirm::new("Do you want to save this note?")
        .with_default(true)
        .prompt();

    return match save_note_bool {
        Ok(true) => {
            api::save_note(&inputted_note)?;
            println!("Note created successfully.");
            Ok(())
        }
        Ok(false) => Ok(()),
        Err(e) => {
            println!("There was an error: {}", e);
            Err(Box::new(e))
        }
    };
}

fn show_notes() -> Result<(), Box<dyn std::error::Error>> {
    let saved_notes = api::get_notes()?;
    let mut options: Vec<String> = Vec::new();
    truncated_note(&mut options)?;
    let selection = Select::new("Select the note that you want to view:", options.clone()).prompt();
    let selection_index = options.iter().position(|n| n == selection.as_ref().unwrap()).unwrap();

    if api::get_notes()?.is_empty() {
        println!("{}", "You don't have any notes.");
        return Ok(());
    } else {
        let selected_note = &saved_notes[selection_index];

        display_note(selected_note);
        return Ok(());
    }
}

fn edit_notes() -> Result<(), Box<dyn std::error::Error>> {
    let saved_notes = api::get_notes()?;
    let mut options: Vec<String> = Vec::new();
    truncated_note(&mut options)?;
    let selection = Select::new("Select the note that you want to edit:", options.clone()).prompt();
    let selection_index = options.iter().position(|n| n == selection.as_ref().unwrap()).unwrap();

    if api::get_notes()?.is_empty() {
        println!("{}", "You can't edit notes, because there are none.");
        return Ok(());
    } else {
        let selected_note = &saved_notes[selection_index];
        let updated_note = Note {
            id: selection_index,
            name: Text::new("New name:").with_initial_value(&selected_note.name).prompt()?,
            content: Text::new("New content:").with_initial_value(&selected_note.content).prompt()?,
            created: selected_note.created.clone(),
        };
    
        let edit_note_bool = Confirm::new("Are you sure that you want to edit this note?")
            .with_default(true)
            .prompt();
            
        return match edit_note_bool {
            Ok(true) => {
                api::edit_note(&updated_note, selection_index)?;
                println!("Note updated successfully.");
                Ok(())
            }
            Ok(false) => Ok(()),
            Err(e) => {
                println!("There was an error: {}", e);
                Err(Box::new(e))
            }
        };
    }
}

fn delete_notes() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Implement select multiple
    let mut options: Vec<String> = Vec::new();
    truncated_note(&mut options)?;
    let selections = MultiSelect::new("Select the notes that you want to delete:", options.clone()).prompt();
    let delete_note_bool = Confirm::new("Are you sure that you want to delete these notes?")
        .with_default(true)
        .prompt();

    if api::get_notes()?.is_empty() {
        println!("{}", "You can't delete notes, because there are none.");
        return Ok(());
    } else {
        return Ok(for selection in selections.unwrap() {
            let selection_index = options.iter().position(|n| n == &selection.as_ref()).unwrap(); 
            
            return match delete_note_bool {
                Ok(true) => {
                    api::delete_note(selection_index)?;
                    println!("Notes deleted successfully.");
                    Ok(())
                }
                Ok(false) => Ok(()),
                Err(e) => {
                    println!("There was an error: {}", e);
                    Err(Box::new(e))
                }
            };
        });
    }
}

fn display_note(note: &Note) {
    println!("=======================");
    println!("Name: {}", note.name);
    println!("Content: {}", note.content);
    println!("Created at: {}", note.created);
    println!("=======================");
}

fn truncated_note(options: &mut Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    Ok(for note in &api::get_notes()? {
        let mut truncated_content: String = note.content.chars().take(10).collect();

        if truncated_content.chars().count() == 10 {
            truncated_content = truncated_content + "...";
        }

        options.push(format!("{} | {} | {}", note.name, truncated_content, note.created));
    })
}

fn cursor_to_origin() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}
