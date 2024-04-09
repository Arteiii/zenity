use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use rand::Rng;

use zenity::animations::{
    frames::progress::ProgressBarFrames,
    progress::{Bar, Progress},
};

fn main() {
    // I know that this is not the best solution I will rework it asap
    // contributions welcome

    println!("test Header line");
    thread::sleep(Duration::from_secs(8));

    let mut progress = Progress::default();

    let progress1 = progress.add(Bar::default());

    // TODO: create wrapper for this
    let progress2 = progress.add(Bar {
        frames: Arc::new(Mutex::new(ProgressBarFrames::rect())),
        size: Arc::new(Mutex::new(70_usize)),
        current: Arc::new(Mutex::new(0_usize)),
        goal: Arc::new(Mutex::new(253_usize)),
    });

    let progress3 = progress.add(Bar {
        frames: Arc::new(Mutex::new(ProgressBarFrames::hash())),
        size: Arc::new(Mutex::new(7_usize)),
        current: Arc::new(Mutex::new(0_usize)),
        goal: Arc::new(Mutex::new(253_usize)),
    });

    progress.run_all();

    let loading = 1_usize;

    for loading in loading..=253 {
        progress.set(&progress1, &loading);
        progress.set(&progress2, &loading);
        progress.set(&progress3, &loading);

        let sleep_time = rand::thread_rng().gen_range(1..=70);
        thread::sleep(Duration::from_millis(sleep_time));
    }

    thread::sleep(Duration::from_millis(1000));
}
