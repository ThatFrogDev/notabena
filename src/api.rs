use crate::note::Note;
use async_std::path::PathBuf;
use rusqlite::{params, Connection, Result};
use std::fs::{self, File};

pub fn init_db(
    data_directory: &PathBuf,
    db_file: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    if File::open(db_file).is_err() {
        let notabena_directory = data_directory
            .parent()
            .unwrap()
            .to_path_buf()
            .join("Notabena");

        println!("{:?}", &notabena_directory);
        fs::create_dir_all(&notabena_directory)?;
        File::create(db_file)?;
    }

    Connection::open(db_file)?.execute(
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

pub fn save_note(note: &Note, db_file: &PathBuf) -> Result<()> {
    Connection::open(db_file)?.execute(
        "INSERT INTO saved_notes (id, name, content, created) VALUES (?1, ?2, ?3, ?4);",
        params![&note.id, &note.name, &note.content, &note.created],
    )?;

    Ok(())
}

pub fn edit_note(note: &Note, idx: usize, db_file: &PathBuf) -> Result<()> {
    Connection::open(db_file)?.execute(
        "UPDATE saved_notes
            SET (name) = ?1, (content) = ?2
            WHERE id = ?3;",
        params![&note.name, &note.content, &idx],
    )?;

    Ok(())
}

pub fn delete_notes(idx: Vec<usize>, db_file: &PathBuf) -> Result<()> {
    let sqlite = Connection::open(db_file)?;
    for identifier in idx {
        sqlite.execute(
            "DELETE FROM saved_notes WHERE id = ?1;",
            params![&identifier],
        )?;
    }

    Ok(())
}

pub fn get_notes(db_file: &PathBuf) -> Result<Vec<Note>> {
    let sqlite = Connection::open(db_file)?;

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
