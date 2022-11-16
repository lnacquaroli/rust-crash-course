// Run cargo watch with
// > cargo-watch -qc -x 'run -- "YOUR_NOTE"' -i "notes.txt" -x clippy
// -i ignores to keep loading the notes.txt indefinitely by cargo-watch

#![deny(clippy::all)]

use std::env;
use std::fs::OpenOptions;
use std::io::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Collect input args
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        Err("Usage: notes your_note_goes_here")?;
        std::process::exit(1);
    }

    // Grab note
    let note = args[1].clone();
    let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    // Write in a file
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("notes.txt")
        .unwrap();

    // Write the current time to the file
    file.write_all(b"<!-- ")?;
    file.write_all(now.as_bytes())?;
    file.write_all(b" -->\n")?;

    // Store the note into the file
    file.write_all(note.as_bytes())?;
    file.write_all(b"\n\n")?;

    Ok(())
}
