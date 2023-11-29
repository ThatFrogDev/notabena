use crate::note::Note;
use async_std::path::PathBuf;
use rusqlite::Connection;

pub fn get_notes(db_file: &PathBuf) -> Result<Vec<Note>, Box<dyn std::error::Error>> {
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
