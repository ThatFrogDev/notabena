use std::io::{self, Write};
use termimad::*;
use crate::note::Note;
use crate::utilities::format_md::{paragraph, inline};

pub fn display(note: &Note) -> Result<(), Box<dyn std::error::Error>> {
    let skin: MadSkin = MadSkin::default();
    let formatted_note = format!(
        "=======================\n\
        {}\
        {}\n\
        {}\
        =======================\n",
        paragraph(&skin, &format!("# {}", inline(&skin, &note.name))),
        paragraph(&skin, &note.content),
        paragraph(&skin, &format!("*{}*", note.created))
    );

    print!("{}", formatted_note);
    io::stdout().flush()?;

    Ok(())
}