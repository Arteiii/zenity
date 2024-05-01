use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread::sleep;
use std::time::Duration;

use zenity::spinner::{Frames, MultiSpinner};

static TOTAL_ANIMATIONS: AtomicUsize = AtomicUsize::new(0);

macro_rules! test_predefined_animation {
    ($animation:expr, $text:expr) => {{
        let custom = MultiSpinner::new();
        custom.add($animation);
        custom.run_all();
        custom.set_text(&custom.get_last(), $text.to_string());
        sleep(Duration::from_secs(5));
        custom.stop(&custom.get_last());

        TOTAL_ANIMATIONS.fetch_add(1, Ordering::Relaxed);
    }};
}

fn main() {
    // new:
    test_predefined_animation!(Frames::dwarf_fortress(), "dwarf_fortress");

    test_predefined_animation!(Frames::matrix_glitch(), "matrix_glitch");
    test_predefined_animation!(Frames::matrix_glitch2(), "matrix_glitch2");
    test_predefined_animation!(Frames::matrix_glitch2_small(), "matrix_glitch2_small");
    test_predefined_animation!(Frames::layer(), "layer");
    test_predefined_animation!(Frames::soccer(), "soccer");
    test_predefined_animation!(Frames::wavy(), "wavy");
    test_predefined_animation!(Frames::wavy2(), "wavy2");
    test_predefined_animation!(Frames::wavy3(), "wavy3");
    test_predefined_animation!(Frames::wavy4(), "wavy4");
    test_predefined_animation!(Frames::pray(), "pray");

    test_predefined_animation!(Frames::red_pulse(), "red_pulse");
    test_predefined_animation!(Frames::dot_bounce2(), "dot_bounce2");
    test_predefined_animation!(Frames::dot_box(), "dot_box");
    test_predefined_animation!(Frames::arrows(), "arrows");
    test_predefined_animation!(Frames::bomb(), "bomb");
    test_predefined_animation!(Frames::simple_line_spin(), "simple_line_spin");
    test_predefined_animation!(Frames::orange_pulse(), "orange_pulse");
    test_predefined_animation!(Frames::blue_pulse(), "blue_pulse");
    test_predefined_animation!(Frames::green_pulse(), "green_pulse");
    test_predefined_animation!(Frames::other(), "other");

    test_predefined_animation!(
        Frames::short_loading_bar_with_arrow(),
        "short_loading_bar_with_arrow"
    );
    test_predefined_animation!(Frames::loading_bar_with_arrow(), "loading_bar_with_arrow");
    test_predefined_animation!(Frames::speaker(), "speaker");
    test_predefined_animation!(Frames::finger_dance(), "finger_dance");
    test_predefined_animation!(Frames::fist_bump(), "fist_bump");
    test_predefined_animation!(Frames::mind_blown(), "mind_blown");
    test_predefined_animation!(Frames::dots_simple_big1(), "dots_simple_big1");
    test_predefined_animation!(Frames::dots_simple_big2(), "dots_simple_big2");
    test_predefined_animation!(Frames::dots_simple_big3(), "dots_simple_big3");
    test_predefined_animation!(Frames::dots_simple_big4(), "dots_simple_big4");
    test_predefined_animation!(Frames::nade(), "nade");
    test_predefined_animation!(Frames::christmas_tree(), "christmas_tree");
    test_predefined_animation!(Frames::weather(), "weather");
    test_predefined_animation!(Frames::raining(), "raining");
    test_predefined_animation!(Frames::runner(), "runner");
    test_predefined_animation!(Frames::hearts(), "hearts");
    test_predefined_animation!(Frames::smiley(), "smiley");
    test_predefined_animation!(Frames::monkey(), "monkey");
    test_predefined_animation!(Frames::bouncing_ball(), "bouncing_ball");
    test_predefined_animation!(Frames::square_corners(), "square_corners");
    test_predefined_animation!(Frames::circle_corners(), "circle_corners");
    test_predefined_animation!(Frames::circle_halves(), "circle_halves");
    test_predefined_animation!(Frames::circle(), "circle");
    test_predefined_animation!(Frames::arc(), "arc");
    test_predefined_animation!(Frames::binary(), "binary");
    test_predefined_animation!(Frames::flip(), "flip");
    test_predefined_animation!(Frames::star1(), "star1");
    test_predefined_animation!(Frames::star2(), "star2");
    test_predefined_animation!(Frames::dots_simple1(), "dots_simple1");
    test_predefined_animation!(Frames::dots_simple2(), "dots_simple2");
    test_predefined_animation!(Frames::dot_spinner1(), "dot_spinner1");
    test_predefined_animation!(Frames::dot_spinner2(), "dot_spinner2");
    test_predefined_animation!(Frames::dot_spinner3(), "dot_spinner3");
    test_predefined_animation!(Frames::dot_spinner4(), "dot_spinner4");
    test_predefined_animation!(Frames::dot_spinner5(), "dot_spinner5");
    test_predefined_animation!(Frames::dot_spinner6(), "dot_spinner6");
    test_predefined_animation!(Frames::dot_spinner7(), "dot_spinner7");
    test_predefined_animation!(Frames::dot_spinner8(), "dot_spinner8");
    test_predefined_animation!(Frames::dot_spinner9(), "dot_spinner9");
    test_predefined_animation!(Frames::dot_spinner10(), "dot_spinner10");
    test_predefined_animation!(Frames::dot_spinner11(), "dot_spinner11");
    test_predefined_animation!(Frames::toggle(), "toggle");
    test_predefined_animation!(Frames::toggle2(), "toggle2");
    test_predefined_animation!(Frames::toggle3(), "toggle3");
    test_predefined_animation!(Frames::toggle4(), "toggle4");
    test_predefined_animation!(Frames::toggle5(), "toggle5");
    test_predefined_animation!(Frames::toggle6(), "toggle6");
    test_predefined_animation!(Frames::toggle7(), "toggle7");
    test_predefined_animation!(Frames::toggle8(), "toggle8");
    test_predefined_animation!(Frames::toggle9(), "toggle9");
    test_predefined_animation!(Frames::toggle10(), "toggle10");
    test_predefined_animation!(Frames::toggle11(), "toggle11");
    test_predefined_animation!(Frames::toggle12(), "toggle12");
    test_predefined_animation!(Frames::toggle13(), "toggle13");
    test_predefined_animation!(Frames::stack(), "stack");
    test_predefined_animation!(Frames::big_loading_bar(), "big_loading_bar");
    test_predefined_animation!(Frames::dot_bounce(), "dot_bounce");
    test_predefined_animation!(Frames::fractions(), "fractions");
    test_predefined_animation!(Frames::wall_bounce_line(), "wall_bounce_line");
    test_predefined_animation!(Frames::wall_bounce(), "wall_bounce");
    test_predefined_animation!(Frames::earth(), "earth");
    test_predefined_animation!(Frames::arrow_row(), "arrow_row");
    test_predefined_animation!(Frames::block(), "block");
    test_predefined_animation!(Frames::block_spinn(), "block_spinn");
    test_predefined_animation!(Frames::line(), "line");
    test_predefined_animation!(Frames::line2(), "line2");
    test_predefined_animation!(Frames::moon(), "moon");
    test_predefined_animation!(Frames::kaomoji(), "kaomoji");
    test_predefined_animation!(Frames::aesthetic_spin(), "aesthetic_spin");
    test_predefined_animation!(Frames::aesthetic_load(), "aesthetic_load");
    test_predefined_animation!(Frames::clock(), "clock");
    test_predefined_animation!(Frames::small_bouncing_bar(), "small_bouncing_bar");
    test_predefined_animation!(Frames::small_loading_bar(), "small_loading_bar");
    test_predefined_animation!(Frames::material(), "material");
    test_predefined_animation!(Frames::arrow_spinn(), "arrow_spinn");
    test_predefined_animation!(Frames::line_spinner(), "line_spinner");
    test_predefined_animation!(Frames::corner(), "corner");
    test_predefined_animation!(Frames::line_spinner_simple(), "line_spinner_simple");
    test_predefined_animation!(Frames::abc(), "abc");
    test_predefined_animation!(Frames::big_arrow_spinn(), "big_arrow_spinn");
    test_predefined_animation!(Frames::japanese(), "japanese");

    println!(
        "Total Animations: {}",
        TOTAL_ANIMATIONS.load(Ordering::Relaxed)
    );
}
