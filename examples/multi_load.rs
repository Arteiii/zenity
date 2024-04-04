use std::io::{stdout, Stdout, Write};
use std::sync::Arc;
use std::thread;

use crossterm::style::Print;
use crossterm::{cursor, execute, queue, terminal};

use zenity::spinner::{Frames, PreDefined};

struct LocatedFrame {
    frame: &'static str,
    column: u16,
    row: u16,
}
struct MultiFrame {
    located_frames: Vec<LocatedFrame>,
    stdout: Stdout,
}

fn render_frame(mut stdout: Stdout, located_frame: LocatedFrame) {
    queue!(
        stdout,
        cursor::RestorePosition,
        cursor::MoveTo(located_frame.column, located_frame.row),
        terminal::Clear(terminal::ClearType::CurrentLine),
        Print(located_frame.frame),
    )
    .unwrap();
}

fn prepare_frames(frames: Vec<Frames>) {
    for frame in frames {}
}

fn main() {
    // sample frames for demonstration
    let frames = vec![
        PreDefined::dot_spinner1(false),
        PreDefined::dot_spinner1(false),
    ];

    let handle = thread::spawn(move || prepare_frames(frames));

    handle.join().unwrap();
}
