pub(crate) mod console_render {
    use std::io::{stdout, Write};

    use crossterm::{cursor, execute, queue, style, terminal};

    use crate::color::ENABLE_COLOR;
    use crate::style::StyledString;

    pub fn render_line(frame: &Vec<String>, row: u16) {
        let mut stdout = stdout();
        queue!(
            stdout,
            cursor::RestorePosition,
            cursor::MoveToNextLine(row + 1),
        )
        .unwrap();

        for content in frame {
            queue!(stdout, style::Print(content),).unwrap();
        }

        stdout.flush().unwrap();
    }

    pub fn render_styled_line(row: u16, content: &[StyledString]) {
        if *ENABLE_COLOR {
            render_styled(row, content);
        } else {
            render_unstyled(row, content);
        }
    }

    pub fn render_unstyled(row: u16, content: &[StyledString]) {
        render_line(
            &content
                .iter()
                .map(|styled_string| styled_string.string.clone())
                .collect::<Vec<_>>(),
            row,
        );
    }

    pub fn render_styled(row: u16, content: &[StyledString]) {
        let mut stdout = stdout();
        queue!(
            stdout,
            cursor::RestorePosition,
            cursor::MoveToNextLine(row + 1), // move to the next line based on index +1
            terminal::Clear(terminal::ClearType::CurrentLine),
        )
        .unwrap();
        for content in content {
            queue!(
                stdout,
                style::SetStyle(content.style), // set animation color
                style::Print(&content.string),
                style::ResetColor, // reset colors
            )
            .unwrap();
        }

        stdout.flush().unwrap();
    }

    pub fn cleanup() {
        execute!(
            stdout(),
            cursor::RestorePosition,
            cursor::MoveToNextLine(2),
            terminal::Clear(terminal::ClearType::FromCursorDown),
        )
        .unwrap();
    }
}

pub(crate) mod console_cursor {
    use std::io::stdout;

    use crossterm::{cursor, execute, terminal};

    /// sets the cursor to be hidden, moves it to the next line,saves its current position,
    /// and clears the terminal screen from the cursor position down
    pub fn save_hide_cursor() {
        execute!(
            stdout(),
            cursor::MoveTo(0, 1),
            cursor::Hide,
            cursor::SavePosition,
            terminal::Clear(terminal::ClearType::FromCursorDown),
        )
        .unwrap();
    }

    /// resets the cursor to be shown and restores its saved position
    pub fn reset_cursor() {
        execute!(stdout(), cursor::RestorePosition, cursor::Show).unwrap();
    }

    /// resets the cursor to be shown and restores its saved position
    pub fn next_line(num: u16) {
        execute!(stdout(), cursor::MoveToNextLine(num)).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use crate::style::Color;

    use crate::style::StyledString;

    use super::*;

    #[test]
    fn test_render_unstyled() {
        let content = vec![
            StyledString::simple("Hello, ", Some(Color::Red), None, None),
            StyledString::simple(" world", Some(Color::Green), None, None),
        ];

        console_render::render_unstyled(4, &content);

        assert!(true);
    }

    #[test]
    fn test_render_styled() {
        let content = vec![
            StyledString::simple("Hello, ", Some(Color::Red), None, None),
            StyledString::simple(" world", Some(Color::Green), None, None),
            StyledString::default(),
        ];

        console_render::render_styled(4, &content);

        assert!(true);
    }
}
