pub(crate) mod console_render {
    use std::io::{stdout, Write};

    use crossterm::{cursor, execute, queue, style, terminal};

    use crate::color::ENABLE_COLOR;
    use crate::style::ContentStyle;

    // use crossterm::style::ContentStyle;

    pub fn render_frame(frame: &str) {
        execute!(stdout(), cursor::RestorePosition, style::Print(frame),).unwrap();
    }

    pub fn render_styled_line(lines: &[String], style: ContentStyle) {
        if *ENABLE_COLOR {
            let mut stdout = stdout();
            for (index, line) in lines.iter().enumerate() {
                queue!(
                    stdout,
                    cursor::RestorePosition,
                    cursor::MoveToNextLine(index as u16 + 1), // move to next line based on index +1
                    style::SetStyle(style),                   // set animation color
                    style::Print(line),
                    style::ResetColor, // reset colors
                )
                .unwrap();
                stdout.flush().unwrap();
            }
        } else {
            render_frame(&lines.join("\n"));
        }
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
