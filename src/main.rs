pub mod api;

use promptly::{prompt, prompt_default};
use chrono::prelude::*;

#[derive(Clone)]
pub struct Note {
    name: String,
    content: String,
    created: DateTime<Local>
}

// #[derive(Default)]
// struct Notes(Vec<Notes>);

impl Note {
    fn save_note(&self, saved_notes: &mut Vec<Note>) {
        // The API for storing the notes is not yet implemented, for now this is fine:
        saved_notes.push(self.clone());
        println!("Your note is saved.");
        show_notes(saved_notes);
    }
}

// TODO: make smamy stuff work
// impl Notes {
//     fn save_notes(&mut self, saved_notes: &mut Vec<Note>) {
//         
//     }
// 
//     fn iter(&self) -> impl Iterator<Item = &Note> {
//         
//     }
// }

fn main() {
    let mut saved_notes: Vec<Note> = Vec::new();

    loop {
        cursor_to_origin();
        show_notes(&saved_notes);

        let new_note_bool = prompt_default("Do you want to create a new note?", false);

        match new_note_bool {
            Ok(true) => {
                new_note(&mut saved_notes).expect("");
                cursor_to_origin();
            },
            Ok(false) => {
                cursor_to_origin();
                return;
            },
            Err(e) => {
                println!("There was an error: {}", e);
            },
        }
    }
}   

fn show_notes(saved_notes: &[Note]) {
    println!("Welcome to Notabena, your favorite note taking app.");
    println!("=======================");
    if saved_notes.is_empty() {
        println!("There are no notes yet.");
        println!("=======================");
    } else {
        for note in saved_notes {
            println!("{}", note.name);
            println!("{}", note.content);
            println!("Created at: {:02}:{:02}", note.created.hour(), note.created.minute());
            println!("=======================");
        }
    };
}

fn new_note(saved_notes: &mut Vec<Note>) -> Result<(), Box<dyn std::error::Error>> {
    let inputted_note = Note {
        name: prompt("Name")?,
        content: prompt("Content")?,
        created: Local::now()
    };

    cursor_to_origin();
    println!("This is the note you just created:");
    println!("=======================");
    println!("Name: {}", inputted_note.name);
    println!("Content: {}", inputted_note.content);
    println!("Created at: {:02}:{:02}", inputted_note.created.hour(), inputted_note.created.minute());
    println!("=======================");

    let save_note_bool = prompt_default("Do you want to save this note?", true);

    return match save_note_bool {
        Ok(true) => {
            inputted_note.save_note(saved_notes);
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

fn cursor_to_origin() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}
