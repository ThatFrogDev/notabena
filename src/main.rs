mod api;
use promptly::{prompt, prompt_opt, prompt_default};
use chrono::prelude::*;

fn main() {
    show_notes();

    let prompt_user_create_note = prompt_default("Do you want to create a new note?", false);

    match prompt_user_create_note {
        Ok(true) => {
            create_new_note();
        }
        Ok(false) => {
            println!("Nevermind, see you next time :)");
        }

        Err(e) => {
            println!("There was an error: {}", e);
        
        }
    }   
}


fn show_notes() {
    // The API for storing the notes is not yet implemented, for now this is fine:
    println!("Hey, welcome to the official AvdanOS Notes app. Here are your notes:\n");
    println!("=======================");
    println!("There are no notes yet.");
    println!("=======================\n");

}

fn create_new_note() -> Result<(), Box<dyn std::error::Error>> {
    println!("== Create a new note: ==");
    
    pub struct Note {
        name: String,
        content: String,
        created: DateTime<Local>
    }
 
    let inputted_note = Note {
        name: prompt("What is the name of the note?")?,
        content: prompt("What is the content of the note?")?,
        created: Local::now()
    };

    println!("\nThis is the note you just created:");
    println!("=======================");
    println!("Name: {}", inputted_note.name);
    println!("Content: {}", inputted_note.content);
    println!("Created at: {:02}:{:02}", inputted_note.created.hour(), inputted_note.created.minute());
    println!("=======================");

    let prompt_user_save_note = prompt_default("Do you want to save this note?", true);

    match prompt_user_save_note {
        Ok(true) => {
            api::save_note();
            return Ok(());
        }

        Ok(false) => {
            return Ok(());
        }

        Err(e) => {
            println!("There was an error: {}", e);
            return Err(Box::new(e));
        }
    }    
}