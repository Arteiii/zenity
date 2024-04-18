//! implementation for menus
//! (work in progress checkout: [issue#20](https://github.com/Arteiii/zenity/issues/20))
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

pub mod input;

pub(crate) fn handle_key_input(buffer: &mut String, force: &mut bool) -> bool {
    _handle_key_input(buffer, crossterm::event::read().unwrap(), force)
}

#[inline]
fn _handle_key_input(buffer: &mut String, event: Event, force: &mut bool) -> bool {
    if let Event::Key(key_event) = event {
        let KeyEvent {
            code,
            modifiers,
            kind,
            ..
        } = key_event;

        if kind != KeyEventKind::Press {
            return false;
        }

        return match code {
            KeyCode::Enter => {
                if modifiers.contains(KeyModifiers::SHIFT) {
                    *force = true;
                } else {
                    return true;
                }
                false
            }
            KeyCode::Backspace => {
                if modifiers.contains(KeyModifiers::SHIFT) {
                    buffer.clear();
                } else {
                    buffer.pop();
                }
                false
            }
            KeyCode::Char(c) => {
                buffer.push(c);
                false
            }
            _ => false,
        };
    }

    false
}

#[cfg(test)]
mod tests {
    use crossterm::event::KeyEventState;

    use super::*;

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
        assert!(_handle_key_input(&mut buffer, event, &mut false));
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
        _handle_key_input(&mut buffer, event, &mut false);
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
        _handle_key_input(&mut buffer, event, &mut false);
        assert_eq!(buffer, "a");
    }
}
