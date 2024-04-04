use std::io::{stdout, Stdout, Write};
use std::sync::{Arc, Mutex};
use std::thread;

use crossterm::{cursor, execute, terminal};
use crossterm::style::Print;

use zenity::spinner::Frames;
use zenity::spinner::PreDefined;

fn render_spinner(frames: Arc<Frames>) {
    let mut stdout = stdout();

    for frame in &frames.frames {
        execute!(
            stdout,
            cursor::MoveTo(0, 1),
            terminal::Clear(terminal::ClearType::CurrentLine),
            Print(frame)
        ).unwrap();
        stdout.flush().expect("stdout flush panic");
        std::thread::sleep(std::time::Duration::from_millis(frames.speed_ms));
    }
}

fn main() {
    let frames = Arc::new(PreDefined::dot_spinner1(false));
    let other_frames = Arc::new(PreDefined::dot_spinner2(false));

    loop {
        let frames_clone = Arc::clone(&frames);
        let other_frames_clone = Arc::clone(&other_frames);

        let handle1 = thread::spawn(move || render_spinner(frames_clone));
        let handle2 = thread::spawn(move || render_spinner(other_frames_clone));

        handle1.join().unwrap();
        handle2.join().unwrap();
    }
}
