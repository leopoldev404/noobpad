use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    event::{read, Event, KeyCode, KeyModifiers},
};
use copypasta::{ClipboardContext, ClipboardProvider};
use std::io::{self, Write};

fn editor() -> io::Result<()> {
    let mut buffer = String::new();
    let mut clipboard = ClipboardContext::new().unwrap();

    enable_raw_mode().unwrap();
    execute!(io::stdout(), EnterAlternateScreen).unwrap();

    loop {
        match read().unwrap() {
            Event::Key(key) => match key.code {
                KeyCode::Char(c) if key.modifiers.contains(KeyModifiers::CONTROL) => match c {
                    'c' => {
                        // Copy buffer to clipboard
                        clipboard.set_contents(buffer.clone()).unwrap();
                        println!("\nCopied to clipboard!");
                    }
                    'v' => {
                        // Paste from clipboard
                        if let Ok(paste) = clipboard.get_contents() {
                            print!("{}", paste);
                            buffer.push_str(&paste);
                        }
                    }
                    _ => (),
                },
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

    if !buffer.trim().is_empty() {
        println!("\nYour note: {}", buffer);
    }

    Ok(())
}
