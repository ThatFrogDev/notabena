// TODO: Organize the code into different files
pub mod api;

use std::process::Command;

use chrono::prelude::*;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, MultiSelect, Select};

pub struct Note {
    id: usize,
    name: String,
    content: String,
    created: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    api::init_db()?;
    cursor_to_origin()?;
    println!("Welcome to Notabena, the FOSS note-taking app.");

    loop {
        let options = vec!["New note", "View note", "Edit note", "Delete note", "Exit"];
        let select = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("What do you want to do?")
            .items(&options)
            .interact()
            .unwrap();

        match select {
            0 => {
                cursor_to_origin()?;
                new_note().expect("Creating a new note failed");
            }
            1 => {
                cursor_to_origin()?;
                show_notes().expect("Failed fetching the notes");
            }
            2 => {
                cursor_to_origin()?;
                edit_notes().expect("Editing the note failed");
            }
            3 => {
                cursor_to_origin()?;
                delete_notes().expect("Deleting the note failed");
            }
            _ => {
                cursor_to_origin()?;
                return Ok(());
            }
        }
    }
}

fn new_note() -> Result<(), Box<dyn std::error::Error>> {
    let saved_notes = api::get_notes()?;
    let inputted_note = Note {
        id: saved_notes.len(),
        name: Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Name:")
            .interact_text()
            .unwrap(),
        content: Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Content:")
            .interact_text()
            .unwrap(),
        created: format!("{}", Local::now().format("%A %e %B, %H:%M")),
    };

    cursor_to_origin()?;

    println!("This is the note you're about to create:");
    display_note(&inputted_note)?;

    let save_note_bool = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to save this note?")
        .default(true)
        .interact()
        .unwrap();

    return match save_note_bool {
        true => {
            api::save_note(&inputted_note)?;
            cursor_to_origin()?;
            println!("Note created successfully.");
            Ok(())
        }
        false => Ok(()),
    };
}

fn show_notes() -> Result<(), Box<dyn std::error::Error>> {
    let saved_notes = api::get_notes()?;
    if saved_notes.is_empty() {
        println!("You don't have any notes.");
        return Ok(());
    } else {
        let mut options: Vec<String> = Vec::new();
        truncated_note(&mut options)?;
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select the note that you want to view:")
            .items(&options)
            .interact()
            .unwrap();

        let selected_note = &saved_notes[selection];
        display_note(&selected_note)?;
        return Ok(());
    }
}

fn edit_notes() -> Result<(), Box<dyn std::error::Error>> {
    let saved_notes = api::get_notes()?;
    let mut options: Vec<String> = Vec::new();
    truncated_note(&mut options)?;
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select the note that you want to edit:")
        .items(&options)
        .interact()
        .unwrap();

    if api::get_notes()?.is_empty() {
        cursor_to_origin()?;
        println!("You can't edit notes, because there are none.");
        return Ok(());
    } else {
        let selected_note = &saved_notes[selection];
        let updated_note = Note {
            id: selection,
            name: Input::with_theme(&ColorfulTheme::default())
                .with_prompt("New name:")
                .with_initial_text(selected_note.name.clone())
                .interact_text()
                .unwrap(),
            content: Input::with_theme(&ColorfulTheme::default())
                .with_prompt("New content:")
                .with_initial_text(selected_note.content.clone())
                .interact_text()
                .unwrap(),
            created: selected_note.created.clone(),
        };

        let edit_note_bool = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Are you sure that you want to edit this note?")
            .default(true)
            .interact()
            .unwrap();

        return match edit_note_bool {
            true => {
                api::edit_note(&updated_note, selection)?;
                cursor_to_origin()?;
                println!("Note updated successfully.");
                Ok(())
            }
            false => Ok(()),
        };
    }
}

// TODO: make an if statement to check if there are multiple or only one note selected
fn delete_notes() -> Result<(), Box<dyn std::error::Error>> {
    let mut options: Vec<String> = Vec::new();
    truncated_note(&mut options)?;
    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt(
            "Select the note(s) that you want to delete:\nSpace to select, Enter to confirm.",
        )
        .items(&options)
        .interact()
        .unwrap();

    let delete_note_bool = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Are you sure that you want to delete these note(s)?")
        .default(true)
        .interact()
        .unwrap();

    if api::get_notes()?.is_empty() {
        cursor_to_origin()?;
        println!("You can't delete notes, because there are none.");
        return Ok(());
    } else {
        for selection in selections {
            if delete_note_bool {
                api::delete_note(selection)?;
            }
        }
        cursor_to_origin()?;
        println!("Notes deleted successfully.");
        return Ok(());
    }
}

fn display_note(note: &Note) -> Result<(), Box<dyn std::error::Error>> {
    println!("=======================");
    println!("Name: {}", note.name);
    println!("Content: {}", note.content);
    println!("Created at: {}", note.created);
    println!("=======================");

    Ok(())
}

fn truncated_note(options: &mut Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    Ok(for note in &api::get_notes()? {
        let mut truncated_content: String = note.content.chars().take(10).collect();

        if truncated_content.chars().count() == 10 {
            truncated_content = truncated_content + "...";
        }

        options.push(format!(
            "{} | {} | {}",
            note.name, truncated_content, note.created
        ));
    })
}

fn cursor_to_origin() -> Result<(), Box<dyn std::error::Error>> {
    if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/c", "cls"]).spawn()?.wait()?;
        Ok(())
    } else {
        Command::new("clear").spawn()?.wait()?;
        Ok(())
    }
}
