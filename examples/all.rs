use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread::sleep;
use std::time::Duration;

use zenity::{spinner::PreDefined, LoadingAnimation};

static TOTAL_ANIMATIONS: AtomicUsize = AtomicUsize::new(0);

macro_rules! test_predefined_animation {
    ($animation:expr, $text:expr) => {{
        let mut custom_loading_animation = LoadingAnimation::new($animation);
        custom_loading_animation.set_text($text);
        sleep(Duration::from_secs(5));
        custom_loading_animation.stop_and_persist(None, None, None);

        TOTAL_ANIMATIONS.fetch_add(1, Ordering::Relaxed);
    }};
}

fn main() {
    // new:
    test_predefined_animation!(PreDefined::dwarf_fortress(false), "dwarf_fortress");
    
    test_predefined_animation!(PreDefined::matrix_glitch(false), "matrix_glitch");
    test_predefined_animation!(PreDefined::matrix_glitch2(false), "matrix_glitch2");
    test_predefined_animation!(PreDefined::matrix_glitch2_small(false), "matrix_glitch2_small");
    test_predefined_animation!(PreDefined::layer(false), "layer");
    test_predefined_animation!(PreDefined::soccer(false), "soccer");
    test_predefined_animation!(PreDefined::wavy(false), "wavy");
    test_predefined_animation!(PreDefined::wavy2(false), "wavy2");
    test_predefined_animation!(PreDefined::wavy3(false), "wavy3");
    test_predefined_animation!(PreDefined::wavy4(false), "wavy4");
    test_predefined_animation!(PreDefined::pray(false), "pray");
    
    test_predefined_animation!(PreDefined::red_pulse(false), "red_pulse");
    test_predefined_animation!(PreDefined::dot_bounce2(false), "dot_bounce2");
    test_predefined_animation!(PreDefined::dot_box(false), "dot_box");
    test_predefined_animation!(PreDefined::arrows(false), "arrows");
    test_predefined_animation!(PreDefined::bomb(false), "bomb");
    test_predefined_animation!(PreDefined::simple_line_spin(false), "simple_line_spin");
    test_predefined_animation!(PreDefined::orange_pulse(false), "orange_pulse");
    test_predefined_animation!(PreDefined::blue_pulse(false), "blue_pulse");
    test_predefined_animation!(PreDefined::green_pulse(false), "green_pulse");
    test_predefined_animation!(PreDefined::other(false), "other");
    
    
    test_predefined_animation!(
        PreDefined::short_loading_bar_with_arrow(false),
        "short_loading_bar_with_arrow"
    );
    test_predefined_animation!(
        PreDefined::loading_bar_with_arrow(false),
        "loading_bar_with_arrow"
    );
    test_predefined_animation!(PreDefined::speaker(false), "speaker");
    test_predefined_animation!(PreDefined::finger_dance(false), "finger_dance");
    test_predefined_animation!(PreDefined::fist_bump(false), "fist_bump");
    test_predefined_animation!(PreDefined::mind_blown(false), "mind_blown");
    test_predefined_animation!(PreDefined::dots_simple_big1(false), "dots_simple_big1");
    test_predefined_animation!(PreDefined::dots_simple_big2(false), "dots_simple_big2");
    test_predefined_animation!(PreDefined::dots_simple_big3(false), "dots_simple_big3");
    test_predefined_animation!(PreDefined::dots_simple_big4(false), "dots_simple_big4");
    test_predefined_animation!(PreDefined::nade(false), "nade");
    test_predefined_animation!(PreDefined::christmas_tree(false), "christmas_tree");
    test_predefined_animation!(PreDefined::weather(false), "weather");
    test_predefined_animation!(PreDefined::raining(false), "raining");
    test_predefined_animation!(PreDefined::runner(false), "runner");
    test_predefined_animation!(PreDefined::hearts(false), "hearts");
    test_predefined_animation!(PreDefined::smiley(false), "smiley");
    test_predefined_animation!(PreDefined::monkey(false), "monkey");
    test_predefined_animation!(PreDefined::bouncing_ball(false), "bouncing_ball");
    test_predefined_animation!(PreDefined::square_corners(false), "square_corners");
    test_predefined_animation!(PreDefined::circle_corners(false), "circle_corners");
    test_predefined_animation!(PreDefined::circle_halves(false), "circle_halves");
    test_predefined_animation!(PreDefined::circle(false), "circle");
    test_predefined_animation!(PreDefined::arc(false), "arc");
    test_predefined_animation!(PreDefined::binary(false), "binary");
    test_predefined_animation!(PreDefined::flip(false), "flip");
    test_predefined_animation!(PreDefined::star1(false), "star1");
    test_predefined_animation!(PreDefined::star2(false), "star2");
    test_predefined_animation!(PreDefined::dots_simple1(false), "dots_simple1");
    test_predefined_animation!(PreDefined::dots_simple2(false), "dots_simple2");
    test_predefined_animation!(PreDefined::dot_spinner1(false), "dot_spinner1");
    test_predefined_animation!(PreDefined::dot_spinner2(false), "dot_spinner2");
    test_predefined_animation!(PreDefined::dot_spinner3(false), "dot_spinner3");
    test_predefined_animation!(PreDefined::dot_spinner4(false), "dot_spinner4");
    test_predefined_animation!(PreDefined::dot_spinner5(false), "dot_spinner5");
    test_predefined_animation!(PreDefined::dot_spinner6(false), "dot_spinner6");
    test_predefined_animation!(PreDefined::dot_spinner7(false), "dot_spinner7");
    test_predefined_animation!(PreDefined::dot_spinner8(false), "dot_spinner8");
    test_predefined_animation!(PreDefined::dot_spinner9(false), "dot_spinner9");
    test_predefined_animation!(PreDefined::dot_spinner10(false), "dot_spinner10");
    test_predefined_animation!(PreDefined::dot_spinner11(false), "dot_spinner11");
    test_predefined_animation!(PreDefined::toggle(false), "toggle");
    test_predefined_animation!(PreDefined::toggle2(false), "toggle2");
    test_predefined_animation!(PreDefined::toggle3(false), "toggle3");
    test_predefined_animation!(PreDefined::toggle4(false), "toggle4");
    test_predefined_animation!(PreDefined::toggle5(false), "toggle5");
    test_predefined_animation!(PreDefined::toggle6(false), "toggle6");
    test_predefined_animation!(PreDefined::toggle7(false), "toggle7");
    test_predefined_animation!(PreDefined::toggle8(false), "toggle8");
    test_predefined_animation!(PreDefined::toggle9(false), "toggle9");
    test_predefined_animation!(PreDefined::toggle10(false), "toggle10");
    test_predefined_animation!(PreDefined::toggle11(false), "toggle11");
    test_predefined_animation!(PreDefined::toggle12(false), "toggle12");
    test_predefined_animation!(PreDefined::toggle13(false), "toggle13");
    test_predefined_animation!(PreDefined::stack(false), "stack");
    test_predefined_animation!(PreDefined::big_loading_bar(false), "big_loading_bar");
    test_predefined_animation!(PreDefined::dot_bounce(false), "dot_bounce");
    test_predefined_animation!(PreDefined::fractions(false), "fractions");
    test_predefined_animation!(PreDefined::wall_bounce_line(false), "wall_bounce_line");
    test_predefined_animation!(PreDefined::wall_bounce(false), "wall_bounce");
    test_predefined_animation!(PreDefined::earth(false), "earth");
    test_predefined_animation!(PreDefined::arrow_row(false), "arrow_row");
    test_predefined_animation!(PreDefined::block(false), "block");
    test_predefined_animation!(PreDefined::block_spinn(false), "block_spinn");
    test_predefined_animation!(PreDefined::line(false), "line");
    test_predefined_animation!(PreDefined::line2(false), "line2");
    test_predefined_animation!(PreDefined::moon(false), "moon");
    test_predefined_animation!(PreDefined::kaomoji(false), "kaomoji");
    test_predefined_animation!(PreDefined::aesthetic_spin(false), "aesthetic_spin");
    test_predefined_animation!(PreDefined::aesthetic_load(false), "aesthetic_load");
    test_predefined_animation!(PreDefined::clock(false), "clock");
    test_predefined_animation!(PreDefined::small_bouncing_bar(false), "small_bouncing_bar");
    test_predefined_animation!(PreDefined::small_loading_bar(false), "small_loading_bar");
    test_predefined_animation!(PreDefined::material(false), "material");
    test_predefined_animation!(PreDefined::arrow_spinn(false), "arrow_spinn");
    test_predefined_animation!(PreDefined::line_spinner(false), "line_spinner");
    test_predefined_animation!(PreDefined::corner(false), "corner");
    test_predefined_animation!(
        PreDefined::line_spinner_simple(false),
        "line_spinner_simple"
    );
    test_predefined_animation!(PreDefined::abc(false), "abc");
    test_predefined_animation!(PreDefined::big_arrow_spinn(false), "big_arrow_spinn");
    test_predefined_animation!(PreDefined::japanese(false), "japanese");

    println!(
        "Total Animations: {}",
        TOTAL_ANIMATIONS.load(Ordering::Relaxed)
    );
}
