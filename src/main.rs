pub mod api;

use promptly::{prompt, prompt_default};
use chrono::prelude::*;

pub struct Note {
    name: String,
    content: String,
    created: String
}

fn main() {
    api::init_db().expect("");

    loop {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        show_notes();

        let new_note_bool = prompt_default("Do you want to create a new note?", false);

        match new_note_bool {
            Ok(true) => {
                new_note().expect("");
                print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            },
            Ok(false) => {
                print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                return;
            },
            Err(e) => {
                println!("There was an error: {}", e);
            },
        }
    }
}


fn show_notes() {
    let mut saved_notes: Vec<Note> = Vec::new();
    saved_notes = api::get_notes().expect("");

    println!("Welcome to Notabena, your favorite note taking app.");
    println!("=======================");
    if saved_notes.is_empty() {
        println!("There are no notes yet.");
        println!("=======================");
    } else {
        for note in saved_notes {
            println!("{}", note.name);
            println!("{}", note.content);
            println!("Created at: {}", note.created);
            println!("=======================");
        }
    };
}

fn new_note() -> Result<(), Box<dyn std::error::Error>> {
    let inputted_note = Note {
        name: prompt("Name")?,
        content: prompt("Content")?,
        created: format!("{:02}:{:02}", Local::now().hour(), Local::now().minute()),
    };

    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("This is the note you just created:");
    println!("=======================");
    println!("Name: {}", inputted_note.name);
    println!("Content: {}", inputted_note.content);
    println!("Created at: {}", inputted_note.created);
    println!("=======================");

    let save_note_bool = prompt_default("Do you want to save this note?", true);

    return match save_note_bool {
        Ok(true) => {
            api::save_note(&inputted_note)?;
            Ok(())
        },
        Ok(false) => {
            Ok(())
        }

        Err(e) => {
            println!("There was an error: {}", e);
            Err(Box::new(e))
        }
    }
}
