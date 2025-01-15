use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    event::{read, Event, KeyCode},
};
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::{env, time::SystemTime};

/// Get the directory where notes are stored
fn get_notes_dir() -> PathBuf {
    let home_dir = env::var("HOME").expect("Could not find HOME directory");
    let notes_dir = Path::new(&home_dir).join(".noobpad");
    if !notes_dir.exists() {
        fs::create_dir_all(&notes_dir).expect("Failed to create notes directory");
    }
    notes_dir
}

/// Save a note to a file
fn save_note(content: &str) -> io::Result<()> {
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    let notes_dir = get_notes_dir();
    let note_path = notes_dir.join(format!("note_{}.txt", timestamp));
    let mut file = File::create(note_path)?;
    file.write_all(content.as_bytes())?;
    println!("\nNote saved successfully!");
    Ok(())
}

/// List all notes
fn list_notes() -> io::Result<()> {
    let notes_dir = get_notes_dir();
    let entries = fs::read_dir(notes_dir)?;

    println!("Your notes:");
    for entry in entries {
        let entry = entry?;
        let file_name = entry.file_name();
        println!("{}", file_name.to_string_lossy());
    }
    Ok(())
}

/// Main editor logic
fn editor() -> io::Result<()> {
    let mut buffer = String::new();
    enable_raw_mode().unwrap();
    execute!(io::stdout(), EnterAlternateScreen).unwrap();

    loop {
        match read().unwrap() {
            Event::Key(key) => match key.code {
                KeyCode::Char(c) => {
                    print!("{}", c);
                    buffer.push(c);
                }
                KeyCode::Backspace => {
                    buffer.pop();
                    print!("\x08 \x08");
                }
                KeyCode::Enter => {
                    buffer.push('\n');
                    println!();
                }
                KeyCode::Esc => break, // Exit on ESC key
                _ => (),
            },
            _ => (),
        }
        io::stdout().flush().unwrap();
    }

    execute!(io::stdout(), LeaveAlternateScreen).unwrap();
    disable_raw_mode().unwrap();

    // Save the note
    if !buffer.trim().is_empty() {
        save_note(&buffer)?;
    } else {
        println!("\nNo content to save.");
    }

    Ok(())
}

/// Command-line interface
fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "list" => list_notes()?,
            _ => eprintln!("Unknown command: {}", args[1]),
        }
    } else {
        editor()?;
    }

    Ok(())
}
