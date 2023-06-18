use std::fs::File;
use rusqlite::{Connection, Result};
use crate::Note;

pub fn init_db() -> Result<()> {
    if !File::open("notes.db").is_ok() {
        File::create("notes.db").expect("Failed to initiate the database.");
    }
    let sqlite = Connection::open("notes.db")?;

    sqlite.execute(
        "CREATE TABLE IF NOT EXISTS saved_notes (
                name TEXT NOT NULL,
                content TEXT NOT NULL,
                created TEXT NOT NULL
            )",
        (),
    )?;

    Ok(())
}

pub fn save_note(note: &Note) -> Result<()> {
    let sqlite = Connection::open("notes.db")?;

    sqlite.execute(
            "INSERT INTO saved_notes (name, content, created) VALUES (?1, ?2, ?3);",
            (&note.name, &note.content, &note.created),
        )?;

    Ok(())
}

pub fn get_notes() -> Result<Vec<Note>> {
    let sqlite = Connection::open("notes.db")?;

    let mut stmt = sqlite.prepare("SELECT name, content, created FROM saved_notes;")?;
    let note_iter = stmt.query_map((), |row| {
        Ok(Note {
            name: row.get(0)?,
            content: row.get(1)?,
            created: row.get(2)?,
        })
    })?;

    let mut notes: Vec<Note> = Vec::new();

    for note in note_iter {
        notes.push(note?);
    }

    Ok(notes)
}