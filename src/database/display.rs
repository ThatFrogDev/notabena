use crate::note::Note;

pub fn display(note: &Note) -> Result<(), Box<dyn std::error::Error>> {
    println!("=======================");
    println!("Name: {}", note.name);
    println!("Content: {}", note.content);
    println!("Created at: {}", note.created);
    println!("=======================");

    Ok(())
}
