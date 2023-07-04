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

        match select("What do you want to do?", &options) {
            0 => new_note().expect("Creating a new note failed"),
            1 => show_notes().expect("Failed fetching the notes"),
            2 => edit_notes().expect("Editing the note failed"),
            3 => delete_notes().expect("Deleting the note failed"),
            _ => return Ok(()),
        }
    }
}

fn new_note() -> Result<(), Box<dyn std::error::Error>> {
    let inputted_note = Note {
        id: api::get_notes()?.len(),
        name: input("Name:", "".to_string()),
        content: input("Content:", "".to_string()),
        created: format!("{}", Local::now().format("%A %e %B, %H:%M")),
    };

    cursor_to_origin()?;
    println!("This is the note you're about to create:");
    display_note(&inputted_note)?;

    return match confirm_prompt("Do you want to save this note?") {
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
        let selection = select("Select the note that you want to view:", &options);
        let selected_note = &saved_notes[selection];
        display_note(&selected_note)?;

        return Ok(());
    }
}

fn edit_notes() -> Result<(), Box<dyn std::error::Error>> {
    let saved_notes = api::get_notes()?;
    let mut options: Vec<String> = Vec::new();
    truncated_note(&mut options)?;
    let selection = select("Select the note that you want to edit:", &options);

    if saved_notes.is_empty() {
        cursor_to_origin()?;
        println!("You can't edit notes, because there are none.");
        return Ok(());
    } else {
        let selected_note = &saved_notes[selection];
        let updated_note = Note {
            id: selection,
            name: input("Name:", selected_note.name.clone()),
            content: input("Content:", selected_note.content.clone()),
            created: selected_note.created.clone(),
        };

        return match confirm_prompt("Are you sure that you want to edit this note?") {
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

fn delete_notes() -> Result<(), Box<dyn std::error::Error>> {
    let mut options: Vec<String> = Vec::new();
    truncated_note(&mut options)?;
    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select the note(s) that you want to delete:\nSpace to select, Enter to confirm.")
        .items(&options)
        .interact()
        .unwrap();

    let mut prompt = "Are you sure that you want to delete these notes?";
    if selections.len() == 1 {
        prompt = "Are you sure that you want to delete this note?";
    }

    cursor_to_origin()?;
    if api::get_notes()?.is_empty() {
        println!("You can't delete notes, because there are none.");
        return Ok(());
    } else {
        if confirm_prompt(prompt) {
            api::delete_notes(selections)?;
        }
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

        options.push(format!("{} | {} | {}", note.name, truncated_content, note.created));
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

fn confirm_prompt(prompt: &str) -> bool {
    Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt).default(true).interact().unwrap()
}

fn select<T: AsRef<str> + std::fmt::Display>(prompt: &str, options: &[T]) -> usize {
    Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt).items(&options).interact().unwrap()
}

fn input(prompt: &str, initial_text: String) -> String {
    match initial_text.as_str() {
        "" => Input::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt).interact_text().unwrap(),
        _ => Input::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt).with_initial_text(initial_text).interact_text().unwrap()
    }
}
