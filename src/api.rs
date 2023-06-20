use std::fs::File;
use rusqlite::{Connection, params, Result};
use crate::Note;

pub fn init_db() -> Result<()> {
    if !File::open("notes.db").is_ok() {
        File::create("notes.db").expect("Failed to initiate the database.");
    }
    let sqlite = Connection::open("notes.db")?;

    sqlite.execute(
        "CREATE TABLE IF NOT EXISTS saved_notes (
                id INTEGER NOT NULL,
                name TEXT NOT NULL,
                content TEXT NOT NULL,
                created TEXT NOT NULL
            )",
        params![],
    )?;

    Ok(())
}

pub fn save_note(note: &Note) -> Result<()> {
    let sqlite = Connection::open("notes.db")?;

    sqlite.execute(
        "INSERT INTO saved_notes (id, name, content, created) VALUES (?1, ?2, ?3, ?4);",
        params![&note.id, &note.name, &note.content, &note.created],
    )?;

    Ok(())
}

pub fn edit_note(note: &Note, idx: usize) -> Result<()> {
    let sqlite = Connection::open("notes.db")?;

    sqlite.execute(
        "UPDATE saved_notes
            SET (name) = ?1, (content) = ?2
            WHERE id = ?3;
            ",
        params![&note.name, &note.content, &idx],
    )?;

    Ok(())
}

pub fn delete_note(idx: usize) -> Result<()> {
    let sqlite = Connection::open("notes.db")?;

    sqlite.execute(
        "DELETE FROM saved_notes WHERE id = ?1;",
        params![&idx],
    )?;

    Ok(())
}

pub fn get_notes() -> Result<Vec<Note>> {
    let sqlite = Connection::open("notes.db")?;

    let mut stmt = sqlite.prepare("SELECT id, name, content, created FROM saved_notes;")?;
    let note_iter = stmt.query_map((), |row| {
        Ok(Note {
            id: row.get(0)?,
            name: row.get(1)?,
            content: row.get(2)?,
            created: row.get(3)?,
        })
    })?;

    let mut notes: Vec<Note> = Vec::new();

    for note in note_iter {
        notes.push(note?);
    }

    Ok(notes)
}