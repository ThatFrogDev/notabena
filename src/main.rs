mod api;
mod database;
mod note;
mod prompts;
mod return_to_main;

use crate::{
    note::Note,
    prompts::{multiselect::multiselect, select::select},
    /* return_to_main::return_to_main, */
};
use async_std::path::PathBuf;
use directories::BaseDirs;
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data_directory: PathBuf = BaseDirs::new().unwrap().config_dir().into();
    let db_file = data_directory.join("Notabena").join("notes.db");
    api::init_db(&data_directory, &db_file)?;
    cursor_to_origin()?;
    println!("Welcome to Notabena!");

    loop {
        let options = vec![
            "New note",
            "View note",
            "Edit note",
            "Delete note",
            "About",
            "Exit",
        ];

        match select("What do you want to do?", &options) {
            0 => Note::create(&db_file).expect("Creating a new note failed"),
            1 => Note::show(&db_file).expect("Failed fetching the notes"),
            2 => Note::edit(&db_file).expect("Editing the note failed"),
            3 => Note::delete(&db_file).expect("Deleting the note failed"),
            4 => display_about().expect("Viewing about failed"),
            _ => {
                cursor_to_origin()?;
                return Ok(());
            }
        }

        /*if return_to_main().is_ok() {
            main()?;
        }*/
    }
}

fn display_about() -> Result<(), Box<dyn std::error::Error>> {
    cursor_to_origin()?;
    println!("Notabena is a FOSS note-taking CLI tool, written in Rust.");
    println!("License: GPL v3\n");
    println!("COPYRIGHT (c) 2024 NOTABENA ORGANISATION\nPROJECT LEADS @ThatFrogDev, @MrSerge01, GITHUB CONTRIBUTORS\n");

    Ok(())
}

fn truncate_note(
    options: &mut Vec<String>,
    db_file: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    for note in &api::get_notes(db_file)? {
        let mut truncated_content: String = note.content.chars().take(10).collect();
        if truncated_content.chars().count() == 10 {
            truncated_content += "...";
        }

        options.push(format!(
            "{} | {} | {}",
            note.name, truncated_content, note.created
        ));
    }
    Ok(())
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
