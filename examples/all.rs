use animations_rs::{
    spinner::{Frames, PreDefined},
    LoadingAnimation,
};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    preview_aesthetic();
    preview_dot_spinner1();
    preview_kaomoji();
    preview_time_travel();
    preview_bouncing_bar();
    preview_material();
}

fn preview_dot_spinner1() {
    println!("dot_spinner1");
    let custom_loading_animation = LoadingAnimation::new(PreDefined::dot_spinner1());
    sleep(Duration::from_secs(5));
}

fn preview_kaomoji() {
    println!("kaomoji");
    let custom_loading_animation = LoadingAnimation::new(PreDefined::kaomoji());
    sleep(Duration::from_secs(5));
}

fn preview_aesthetic() {
    println!("aesthetic");
    let custom_loading_animation = LoadingAnimation::new(PreDefined::aesthetic());
    sleep(Duration::from_secs(5));
}

fn preview_time_travel() {
    println!("time_travel");
    let custom_loading_animation = LoadingAnimation::new(PreDefined::time_travel());
    sleep(Duration::from_secs(5));
}

fn preview_bouncing_bar() {
    println!("bouncing_bar");
    let custom_loading_animation = LoadingAnimation::new(PreDefined::bouncing_bar());
    sleep(Duration::from_secs(5));
}

fn preview_material() {
    println!("material");
    let custom_loading_animation = LoadingAnimation::new(PreDefined::material());
    sleep(Duration::from_secs(5));
}
