mod api;
mod note;
mod prompts;
mod return_to_main;
mod tests;
mod utilities;

use std::process::exit;

use crate::{
    note::Note,
    prompts::{multiselect::multiselect, select::select},
    return_to_main::return_to_main,
    utilities::{
        cursor_to_origin::cursor_to_origin, format_md::paragraph, truncate_note::truncate_note,
    },
};
use async_std::path::PathBuf;
use directories::BaseDirs;
use termimad::MadSkin;

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
            5 => exit(0),
            _ => (),
        }
    }
}

fn display_about() -> Result<(), Box<dyn std::error::Error>> {
    let skin: MadSkin = MadSkin::default();

    cursor_to_origin()?;
    println!("{}", paragraph(&skin, "# About Notabena"));
    println!(
        "{}",
        paragraph(
            &skin,
            "**Notabena** is a FOSS note-taking CLI tool, written in Rust.\nDonations are always a great way to help us keeping the project alive. It can be done here: https://paypal.me/Notabena (ctrl+click to follow link)."
        )
    );
    println!(
        "version: v{}, licensed under: GPL v3",
        env!("CARGO_PKG_VERSION")
    );
    println!("COPYRIGHT (c) 2023-PRESENT NOTABENA ORGANISATION\nPROJECT LEADS @ThatFrogDev, @MrSerge01, GITHUB CONTRIBUTORS\n\n(scroll up if you can't read everything)");

    Ok(())
}
