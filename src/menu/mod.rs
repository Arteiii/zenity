//! implementation for menus
//! (work in progress checkout: [issue#20](https://github.com/Arteiii/zenity/issues/20))
use crossterm::event::{Event, KeyCode, KeyEvent};

pub mod input;

#[cfg(unix)]
pub(crate) fn handle_key_input(buffer: &mut String) -> bool {
    handle_key_input_unix(buffer, crossterm::event::read().unwrap())
}

#[cfg(windows)]
pub(crate) fn handle_key_input(buffer: &mut String) -> bool {
    handle_key_input_windows(buffer, crossterm::event::read().unwrap())
}

#[cfg(unix)]
fn handle_key_input_unix(buffer: &mut String, event: Event) -> bool {
    let event = crossterm::event::read().unwrap();

    // Handle events
    if let Event::Key(key_event) = event {
        let KeyEvent { code, .. } = key_event;

        match code {
            KeyCode::Enter => {
                return true; // signal to validate the input
            }
            KeyCode::Backspace => {
                buffer.pop();
            }
            KeyCode::Char(c) => {
                buffer.push(c);
            }
            _ => {}
        }
    }

    false
}

#[cfg(windows)]
fn handle_key_input_windows(buffer: &mut String, event: Event) -> bool {
    static mut SKIP_NEXT: bool = false;
    static mut SKIP_NEXT_BACK: bool = false;

    if let Event::Key(key_event) = event {
        let KeyEvent { code, .. } = key_event;

        // TODO!: fix unsafe usage!!!
        match code {
            KeyCode::Enter => {
                return true; // signal to validate the input
            }
            KeyCode::Backspace => unsafe {
                if !SKIP_NEXT_BACK {
                    buffer.pop();
                    SKIP_NEXT_BACK = true
                } else {
                    SKIP_NEXT_BACK = false
                }
            },
            KeyCode::Char(c) => unsafe {
                if !SKIP_NEXT {
                    buffer.push(c);
                    SKIP_NEXT = true
                } else {
                    SKIP_NEXT = false
                }
            },
            _ => {}
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyEventKind, KeyEventState, KeyModifiers};

    #[cfg(unix)]
    #[test]
    fn test_handle_key_input_unix_enter() {
        let mut buffer = String::new();
        let event = Event::Key(KeyEvent {
            code: KeyCode::Enter,
            modifiers: Default::default(),
        });
        assert_eq!(handle_key_input_unix(&mut buffer, event), true);
    }

    #[cfg(unix)]
    #[test]
    fn test_handle_key_input_unix_backspace() {
        let mut buffer = String::from("test");
        let event = Event::Key(KeyEvent {
            code: KeyCode::Backspace,
            modifiers: Default::default(),
        });
        handle_key_input_unix(&mut buffer, event);
        assert_eq!(buffer, "tes");
    }

    #[cfg(unix)]
    #[test]
    fn test_handle_key_input_unix_char() {
        let mut buffer = String::new();
        let event = Event::Key(KeyEvent {
            code: KeyCode::Char('a'),
            modifiers: Default::default(),
        });
        handle_key_input_unix(&mut buffer, event);
        assert_eq!(buffer, "a");
    }

    #[test]
    fn test_handle_key_input_windows_enter() {
        let mut buffer = String::new();
        let event = Event::Key(KeyEvent {
            code: KeyCode::Enter,
            modifiers: KeyModifiers::empty(),
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        });
        assert!(handle_key_input_windows(&mut buffer, event));
    }

    #[test]
    fn test_handle_key_input_windows_backspace() {
        let mut buffer = String::from("test");
        let event = Event::Key(KeyEvent {
            code: KeyCode::Backspace,
            modifiers: KeyModifiers::empty(),
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        });
        handle_key_input_windows(&mut buffer, event);
        assert_eq!(buffer, "tes");
    }

    #[test]
    fn test_handle_key_input_windows_char() {
        let mut buffer = String::new();
        let event = Event::Key(KeyEvent {
            code: KeyCode::Char('a'),
            modifiers: KeyModifiers::empty(),
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        });
        handle_key_input_windows(&mut buffer, event);
        assert_eq!(buffer, "a");
    }
}
