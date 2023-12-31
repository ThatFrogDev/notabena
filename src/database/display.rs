use termimad::*;

use crate::database::format_md::format_md;
use crate::note::Note;

pub fn display(note: &Note) -> Result<(), Box<dyn std::error::Error>> {
    let skin: MadSkin = MadSkin::default();

    println!("=======================");
    println!("Name: {}", format_md(&skin, &note.name));
    println!("Content: {}", format_md(&skin, &note.content));
    println!("Created at: {}", note.created);
    println!("=======================");

    Ok(())
}
