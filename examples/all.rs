use animations_rs::{spinner::PreDefined, LoadingAnimation};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    preview_aesthetic_spin();
    preview_aesthetic_load();
    preview_dot_spinner1();
    preview_dot_spinner2();
    preview_dot_spinner3();
    preview_dot_spinner4();
    preview_dot_spinner5();
    preview_kaomoji();
    preview_clock();
    preview_bouncing_bar();
    preview_material();
}

fn preview_dot_spinner1() {
    println!("dot_spinner1");
    let custom_loading_animation = LoadingAnimation::new(PreDefined::dot_spinner1(false));
    sleep(Duration::from_secs(5));
}

fn preview_dot_spinner2() {
    {
        // so the custom loading animation runs out of scope
        println!("dot_spinner2(false)");
        let custom_loading_animation = LoadingAnimation::new(PreDefined::dot_spinner2(false));
        sleep(Duration::from_secs(5));
    }

    println!("dot_spinner2(true)");
    // invert active
    let custom_loading_animation = LoadingAnimation::new(PreDefined::dot_spinner2(true));
    sleep(Duration::from_secs(5));
}

fn preview_dot_spinner3() {
    println!("dot_spinner3");
    let custom_loading_animation = LoadingAnimation::new(PreDefined::dot_spinner3(false));
    sleep(Duration::from_secs(5));
}

fn preview_dot_spinner4() {
    println!("dot_spinner4");
    let custom_loading_animation = LoadingAnimation::new(PreDefined::dot_spinner4(false));
    sleep(Duration::from_secs(5));
}

fn preview_dot_spinner5() {
    println!("dot_spinner5");
    let custom_loading_animation = LoadingAnimation::new(PreDefined::dot_spinner5(false));
    sleep(Duration::from_secs(5));
}

fn preview_kaomoji() {
    println!("kaomoji");
    let custom_loading_animation = LoadingAnimation::new(PreDefined::kaomoji(false));
    sleep(Duration::from_secs(5));
}

fn preview_aesthetic_spin() {
    println!("aesthetic");
    let custom_loading_animation = LoadingAnimation::new(PreDefined::aesthetic_spin(false));
    sleep(Duration::from_secs(5));
}

fn preview_aesthetic_load() {
    println!("aesthetic");
    let custom_loading_animation = LoadingAnimation::new(PreDefined::aesthetic_load(false));
    sleep(Duration::from_secs(5));
}

fn preview_clock() {
    println!("clock");
    let custom_loading_animation = LoadingAnimation::new(PreDefined::clock(false));
    sleep(Duration::from_secs(5));
}

fn preview_bouncing_bar() {
    println!("bouncing_bar");
    let custom_loading_animation = LoadingAnimation::new(PreDefined::bouncing_bar(false));
    sleep(Duration::from_secs(5));
}

fn preview_material() {
    println!("material");
    let custom_loading_animation = LoadingAnimation::new(PreDefined::material(false));
    sleep(Duration::from_secs(5));
}
