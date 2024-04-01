use crossterm::style::SetStyle;
use crossterm::{
    cursor, execute,
    style::{ContentStyle, Print, ResetColor},
    terminal,
};
use std::io::stdout;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use super::frames::spinner::Frames;

/// a loading animation that runs in a separate thread.
pub fn spinner_animation(
    frames: &Frames,
    should_stop: Arc<Mutex<bool>>,
    text: Arc<Mutex<Option<String>>>,
    animation_style: Arc<Mutex<ContentStyle>>,
    text_style: Arc<Mutex<ContentStyle>>,
    end_sequence: Arc<Mutex<Option<String>>>,
    cleanup_on_exit: Arc<Mutex<bool>>,
) {
    let mut frame_index = 0;
    let longest_frame_len = frames
        .frames
        .iter()
        .map(|frame| frame.len())
        .max()
        .unwrap_or(0);

    execute!(
        stdout(),
        terminal::Clear(terminal::ClearType::CurrentLine),
        cursor::MoveToColumn(0),
        cursor::Hide
    )
    .unwrap(); // hide cursor

    while !*should_stop.lock().unwrap() {
        let frame = &frames.frames[frame_index];

        // clear the current line, move the cursor to the beginning of the line, print the frame, and flush stdout
        execute!(
            stdout(),
            terminal::Clear(terminal::ClearType::CurrentLine),
            cursor::MoveToColumn(0),
            SetStyle(*animation_style.lock().unwrap()), // set animation color
            Print(frame),
            Print("  "),
            SetStyle(*text_style.lock().unwrap()), // set text color
            Print(text.lock().unwrap().as_ref().unwrap_or(&"".to_string())),
            ResetColor, // reset colors
        )
        .unwrap();

        thread::sleep(Duration::from_millis(frames.speed_ms));

        frame_index = (frame_index + 1) % frames.frames.len();
    }

    // print end sequence if provided
    if let Some(end_seq) = &*end_sequence.lock().unwrap() {
        execute!(
            stdout(),
            terminal::Clear(terminal::ClearType::CurrentLine),
            cursor::MoveToColumn(0),
            ResetColor,
            Print(end_seq),
            Print("  "),
            SetStyle(*text_style.lock().unwrap()),
            Print(text.lock().unwrap().as_ref().unwrap_or(&"".to_string())),
            ResetColor,
            Print("\n"),
        )
        .unwrap();
    }

    if *cleanup_on_exit.lock().unwrap() {
        let clear_length = " ".repeat(longest_frame_len);
        // if cleanup active clear the line after the animation finishes
        execute!(
            stdout(),
            terminal::Clear(terminal::ClearType::CurrentLine),
            cursor::MoveToColumn(0),
            Print(&clear_length),
            Print("\r"),
        )
        .unwrap();
    }

    execute!(
        stdout(),
        cursor::Show, // show cursor
    ).unwrap();
}
