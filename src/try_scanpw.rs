use std::io::{stdout, Write};

use crossterm::{
    cursor::{position, MoveLeft, MoveToNextLine},
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    style::Print,
    terminal,
};

/// Attempts to read a password from standard input
///
/// `echo` controls whether a replacement character should be printed each time
/// the user enters a character, and if so, which character. The result is
/// either a [`String`] or a [`crossterm::ErrorKind`]. Input begins wherever the
/// cursor was before calling this function, which is likely to be on its own
/// empty line.
pub fn try_scanpw(echo: Option<char>) -> crossterm::Result<String> {
    // Enter raw mode so we can control character echoing
    terminal::enable_raw_mode()?;

    // In case anything was printed prior to the beginning of the input on the
    // same line, store the column the cursor started at
    let (max_left, _height) = position()?;

    // The password
    let mut pw = String::new();

    loop {
        if let Event::Key(k) = event::read()? {
            match k {
                // Normal character input
                KeyEvent {
                    code: KeyCode::Char(c),
                    modifiers,
                } if modifiers.is_empty() => {
                    let c = echo.unwrap_or(c);
                    execute!(stdout(), Print(c))?;

                    // Add the character to the password
                    pw.push(c);
                }

                // Password input completed
                KeyEvent {
                    code: KeyCode::Enter,
                    ..
                } => {
                    execute!(stdout(), Print('\n'))?;
                    execute!(stdout(), MoveToNextLine(1))?;
                    break;
                }

                // Handle backspace
                KeyEvent {
                    code: KeyCode::Backspace,
                    ..
                } => {
                    // If echo characters are enabled and any exist, remove the
                    // rightmost one
                    let (cur_left, _height) = position()?;

                    // True if the next position isn't past the left of the
                    // column where the cursor started
                    let not_too_far = cur_left
                        .checked_sub(1)
                        .map(|np| np >= max_left)
                        .unwrap_or(false);

                    if not_too_far {
                        execute!(stdout(), MoveLeft(1), Print(" "), MoveLeft(1))?;
                    }

                    // Delete the character from the password
                    pw.pop();
                }

                // Pass Ctrl+C through as a signal like normal
                KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers,
                } if modifiers == KeyModifiers::CONTROL => {
                    // This is a bit silly
                    execute!(stdout(), Print("^C"),)?;

                    // Reset the terminal back to normal and exit
                    terminal::disable_raw_mode()?;

                    die();
                }

                // Ignore other cases
                _ => (),
            }
        }
    }

    // Reset the terminal back to normal
    terminal::disable_raw_mode()?;

    Ok(pw)
}

fn die() {
    cfg_if::cfg_if! {
        if #[cfg(unix)] {
            use nix::sys::signal::{raise, Signal::SIGINT};

            raise(SIGINT).unwrap();
        } else if #[cfg(windows)] {
            use winapi::um::{
                wincon::{
                    GenerateConsoleCtrlEvent,
                    CTRL_C_EVENT,
                },
                processthreadsapi::GetCurrentProcessId,
            };

            unsafe {
                let res = GenerateConsoleCtrlEvent(
                    CTRL_C_EVENT,
                    GetCurrentProcessId(),
                );

                if res == 0 {
                    panic!("failed to generate CTRL_C_EVENT");
                }
            }
        } else {
            std::process::exit(1);
        }
    }
}
