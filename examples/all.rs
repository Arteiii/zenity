use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread::sleep;
use std::time::Duration;

use zenity::spinner::{MultiSpinner, Frames};

static TOTAL_ANIMATIONS: AtomicUsize = AtomicUsize::new(0);

macro_rules! test_predefined_animation {
    ($animation:expr, $text:expr) => {{
        let custom = MultiSpinner::new($animation);
        custom.run_all();
        custom.set_text(&custom.get_uid(), $text.to_string());
        sleep(Duration::from_secs(5));
        custom.stop(&custom.get_uid());

        TOTAL_ANIMATIONS.fetch_add(1, Ordering::Relaxed);
    }};
}

fn main() {
    // new:
    test_predefined_animation!(Frames::dwarf_fortress(false), "dwarf_fortress");

    test_predefined_animation!(Frames::matrix_glitch(false), "matrix_glitch");
    test_predefined_animation!(Frames::matrix_glitch2(false), "matrix_glitch2");
    test_predefined_animation!(
        Frames::matrix_glitch2_small(false),
        "matrix_glitch2_small"
    );
    test_predefined_animation!(Frames::layer(false), "layer");
    test_predefined_animation!(Frames::soccer(false), "soccer");
    test_predefined_animation!(Frames::wavy(false), "wavy");
    test_predefined_animation!(Frames::wavy2(false), "wavy2");
    test_predefined_animation!(Frames::wavy3(false), "wavy3");
    test_predefined_animation!(Frames::wavy4(false), "wavy4");
    test_predefined_animation!(Frames::pray(false), "pray");

    test_predefined_animation!(Frames::red_pulse(false), "red_pulse");
    test_predefined_animation!(Frames::dot_bounce2(false), "dot_bounce2");
    test_predefined_animation!(Frames::dot_box(false), "dot_box");
    test_predefined_animation!(Frames::arrows(false), "arrows");
    test_predefined_animation!(Frames::bomb(false), "bomb");
    test_predefined_animation!(Frames::simple_line_spin(false), "simple_line_spin");
    test_predefined_animation!(Frames::orange_pulse(false), "orange_pulse");
    test_predefined_animation!(Frames::blue_pulse(false), "blue_pulse");
    test_predefined_animation!(Frames::green_pulse(false), "green_pulse");
    test_predefined_animation!(Frames::other(false), "other");

    test_predefined_animation!(
        Frames::short_loading_bar_with_arrow(false),
        "short_loading_bar_with_arrow"
    );
    test_predefined_animation!(
        Frames::loading_bar_with_arrow(false),
        "loading_bar_with_arrow"
    );
    test_predefined_animation!(Frames::speaker(false), "speaker");
    test_predefined_animation!(Frames::finger_dance(false), "finger_dance");
    test_predefined_animation!(Frames::fist_bump(false), "fist_bump");
    test_predefined_animation!(Frames::mind_blown(false), "mind_blown");
    test_predefined_animation!(Frames::dots_simple_big1(false), "dots_simple_big1");
    test_predefined_animation!(Frames::dots_simple_big2(false), "dots_simple_big2");
    test_predefined_animation!(Frames::dots_simple_big3(false), "dots_simple_big3");
    test_predefined_animation!(Frames::dots_simple_big4(false), "dots_simple_big4");
    test_predefined_animation!(Frames::nade(false), "nade");
    test_predefined_animation!(Frames::christmas_tree(false), "christmas_tree");
    test_predefined_animation!(Frames::weather(false), "weather");
    test_predefined_animation!(Frames::raining(false), "raining");
    test_predefined_animation!(Frames::runner(false), "runner");
    test_predefined_animation!(Frames::hearts(false), "hearts");
    test_predefined_animation!(Frames::smiley(false), "smiley");
    test_predefined_animation!(Frames::monkey(false), "monkey");
    test_predefined_animation!(Frames::bouncing_ball(false), "bouncing_ball");
    test_predefined_animation!(Frames::square_corners(false), "square_corners");
    test_predefined_animation!(Frames::circle_corners(false), "circle_corners");
    test_predefined_animation!(Frames::circle_halves(false), "circle_halves");
    test_predefined_animation!(Frames::circle(false), "circle");
    test_predefined_animation!(Frames::arc(false), "arc");
    test_predefined_animation!(Frames::binary(false), "binary");
    test_predefined_animation!(Frames::flip(false), "flip");
    test_predefined_animation!(Frames::star1(false), "star1");
    test_predefined_animation!(Frames::star2(false), "star2");
    test_predefined_animation!(Frames::dots_simple1(false), "dots_simple1");
    test_predefined_animation!(Frames::dots_simple2(false), "dots_simple2");
    test_predefined_animation!(Frames::dot_spinner1(false), "dot_spinner1");
    test_predefined_animation!(Frames::dot_spinner2(false), "dot_spinner2");
    test_predefined_animation!(Frames::dot_spinner3(false), "dot_spinner3");
    test_predefined_animation!(Frames::dot_spinner4(false), "dot_spinner4");
    test_predefined_animation!(Frames::dot_spinner5(false), "dot_spinner5");
    test_predefined_animation!(Frames::dot_spinner6(false), "dot_spinner6");
    test_predefined_animation!(Frames::dot_spinner7(false), "dot_spinner7");
    test_predefined_animation!(Frames::dot_spinner8(false), "dot_spinner8");
    test_predefined_animation!(Frames::dot_spinner9(false), "dot_spinner9");
    test_predefined_animation!(Frames::dot_spinner10(false), "dot_spinner10");
    test_predefined_animation!(Frames::dot_spinner11(false), "dot_spinner11");
    test_predefined_animation!(Frames::toggle(false), "toggle");
    test_predefined_animation!(Frames::toggle2(false), "toggle2");
    test_predefined_animation!(Frames::toggle3(false), "toggle3");
    test_predefined_animation!(Frames::toggle4(false), "toggle4");
    test_predefined_animation!(Frames::toggle5(false), "toggle5");
    test_predefined_animation!(Frames::toggle6(false), "toggle6");
    test_predefined_animation!(Frames::toggle7(false), "toggle7");
    test_predefined_animation!(Frames::toggle8(false), "toggle8");
    test_predefined_animation!(Frames::toggle9(false), "toggle9");
    test_predefined_animation!(Frames::toggle10(false), "toggle10");
    test_predefined_animation!(Frames::toggle11(false), "toggle11");
    test_predefined_animation!(Frames::toggle12(false), "toggle12");
    test_predefined_animation!(Frames::toggle13(false), "toggle13");
    test_predefined_animation!(Frames::stack(false), "stack");
    test_predefined_animation!(Frames::big_loading_bar(false), "big_loading_bar");
    test_predefined_animation!(Frames::dot_bounce(false), "dot_bounce");
    test_predefined_animation!(Frames::fractions(false), "fractions");
    test_predefined_animation!(Frames::wall_bounce_line(false), "wall_bounce_line");
    test_predefined_animation!(Frames::wall_bounce(false), "wall_bounce");
    test_predefined_animation!(Frames::earth(false), "earth");
    test_predefined_animation!(Frames::arrow_row(false), "arrow_row");
    test_predefined_animation!(Frames::block(false), "block");
    test_predefined_animation!(Frames::block_spinn(false), "block_spinn");
    test_predefined_animation!(Frames::line(false), "line");
    test_predefined_animation!(Frames::line2(false), "line2");
    test_predefined_animation!(Frames::moon(false), "moon");
    test_predefined_animation!(Frames::kaomoji(false), "kaomoji");
    test_predefined_animation!(Frames::aesthetic_spin(false), "aesthetic_spin");
    test_predefined_animation!(Frames::aesthetic_load(false), "aesthetic_load");
    test_predefined_animation!(Frames::clock(false), "clock");
    test_predefined_animation!(Frames::small_bouncing_bar(false), "small_bouncing_bar");
    test_predefined_animation!(Frames::small_loading_bar(false), "small_loading_bar");
    test_predefined_animation!(Frames::material(false), "material");
    test_predefined_animation!(Frames::arrow_spinn(false), "arrow_spinn");
    test_predefined_animation!(Frames::line_spinner(false), "line_spinner");
    test_predefined_animation!(Frames::corner(false), "corner");
    test_predefined_animation!(
        Frames::line_spinner_simple(false),
        "line_spinner_simple"
    );
    test_predefined_animation!(Frames::abc(false), "abc");
    test_predefined_animation!(Frames::big_arrow_spinn(false), "big_arrow_spinn");
    test_predefined_animation!(Frames::japanese(false), "japanese");

    println!(
        "Total Animations: {}",
        TOTAL_ANIMATIONS.load(Ordering::Relaxed)
    );
}
