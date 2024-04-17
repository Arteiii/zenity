//! implementation for menus
//! (work in progress checkout: [issue#20](https://github.com/Arteiii/zenity/issues/20))
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

pub mod input;

#[cfg(unix)]
pub(crate) fn handle_key_input(buffer: &mut String, force: &mut bool) -> bool {
    handle_key_input_unix(buffer, crossterm::event::read().unwrap(), force)
}

#[cfg(windows)]
pub(crate) fn handle_key_input(buffer: &mut String, force: &mut bool) -> bool {
    handle_key_input_windows(buffer, crossterm::event::read().unwrap(), force)
}

#[cfg(unix)]
fn handle_key_input_unix(buffer: &mut String, event: Event, force: &mut bool) -> bool {
    if let Event::Key(key_event) = event {
        let KeyEvent {
            code, modifiers, ..
        } = key_event;

        match code {
            KeyCode::Enter => {
                if modifiers.contains(KeyModifiers::SHIFT) {
                    *force = true;
                }
                return true;
            }
            KeyCode::Backspace => {
                if modifiers.contains(KeyModifiers::SHIFT) {
                    buffer.clear();
                } else {
                    buffer.pop();
                }
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
fn handle_key_input_windows(buffer: &mut String, event: Event, force: &mut bool) -> bool {
    static mut SKIP_NEXT: bool = false;
    // true to fix execute keypress-release to be used as keypress
    static mut SKIP_NEXT_BACK: bool = false;
    static mut SKIP_NEXT_ENTER: bool = false;

    if let Event::Key(key_event) = event {
        let KeyEvent {
            code, modifiers, ..
        } = key_event;
        

        // TODO!: fix unsafe usage!!!
        match code {
            KeyCode::Enter => unsafe {
                if !SKIP_NEXT_ENTER {
                    SKIP_NEXT_ENTER = true;
                    if modifiers.contains(KeyModifiers::SHIFT) {
                        *force = true;
                    }
                    return true;
                } else {
                    SKIP_NEXT_ENTER = false
                }
            },
            KeyCode::Backspace => unsafe {
                if !SKIP_NEXT_BACK {
                    if modifiers.contains(KeyModifiers::SHIFT) {
                        buffer.clear();
                    } else {
                        buffer.pop();
                    }
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
    use crossterm::event::{KeyEventKind, KeyEventState};

    use super::*;

    #[cfg(unix)]
    #[test]
    fn test_handle_key_input_unix_enter() {
        let mut buffer = String::new();
        let event = Event::Key(KeyEvent {
            code: KeyCode::Enter,
            modifiers: KeyModifiers::empty(),
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        });
        assert!(handle_key_input_unix(&mut buffer, event, &mut false));
    }

    #[cfg(unix)]
    #[test]
    fn test_handle_key_input_unix_backspace() {
        let mut buffer = String::from("test");
        let event = Event::Key(KeyEvent {
            code: KeyCode::Backspace,
            modifiers: KeyModifiers::empty(),
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        });
        handle_key_input_unix(&mut buffer, event, &mut false);
        assert_eq!(buffer, "tes");
    }

    #[cfg(unix)]
    #[test]
    fn test_handle_key_input_unix_char() {
        let mut buffer = String::new();
        let event = Event::Key(KeyEvent {
            code: KeyCode::Char('a'),
            modifiers: KeyModifiers::empty(),
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        });
        handle_key_input_unix(&mut buffer, event, &mut false);
        assert_eq!(buffer, "a");
    }

    #[cfg(windows)]
    #[test]
    fn test_handle_key_input_windows_enter() {
        let mut buffer = String::new();
        let event = Event::Key(KeyEvent {
            code: KeyCode::Enter,
            modifiers: KeyModifiers::empty(),
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        });
        assert!(handle_key_input_windows(&mut buffer, event, &mut false));
    }

    #[cfg(windows)]
    #[test]
    fn test_handle_key_input_windows_backspace() {
        let mut buffer = String::from("test");
        let event = Event::Key(KeyEvent {
            code: KeyCode::Backspace,
            modifiers: KeyModifiers::empty(),
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        });
        handle_key_input_windows(&mut buffer, event, &mut false);
        assert_eq!(buffer, "tes");
    }

    #[cfg(windows)]
    #[test]
    fn test_handle_key_input_windows_char() {
        let mut buffer = String::new();
        let event = Event::Key(KeyEvent {
            code: KeyCode::Char('a'),
            modifiers: KeyModifiers::empty(),
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        });
        handle_key_input_windows(&mut buffer, event, &mut false);
        assert_eq!(buffer, "a");
    }
}
