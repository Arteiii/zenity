use std::thread::sleep;
use std::time::Duration;

use unicode_icons::icons::symbols::{check_mark_button, cross_mark};

use zenity::spinner::Frames;
use zenity::spinner::MultiSpinner;
use zenity::style::{Color, StyledString};

fn main() {
    {
        let check_mark_text = check_mark_button();
        let cross_mark_text = cross_mark();

        let spinner = MultiSpinner::default();
        let spinner1 = spinner.get_last();

        // main thread operations
        let spinner2 = spinner.add(Frames::wavy());
        let spinner3 = spinner.add(Frames::dot_spinner9());
        let spinner4 = spinner.add(Frames::dot_spinner8());

        spinner.run_all();

        sleep(Duration::from_secs(4));
        spinner.set_text(&spinner2, "  spinner2".to_string());
        // stop spinner1
        spinner.set_styled_text(
            &spinner1,
            StyledString::simple(" spinner1", Some(Color::Blue), Some(Color::DarkBlue), None),
        );

        sleep(Duration::from_secs(2));

        spinner.stop(&spinner2);
        spinner.set_styled_text(
            &spinner2,
            StyledString::simple(
                &format!(" {} Successfully", &check_mark_text),
                Some(Color::Green),
                None,
                None,
            ),
        );

        sleep(Duration::from_secs(2));
        spinner.set_text(&spinner1, " spinner1 stopped".to_string());

        spinner.stop(&spinner1);

        sleep(Duration::from_secs(9));
        spinner.stop(&spinner3);
        spinner.stop(&spinner4);

        spinner.set_styled_text(
            &spinner3,
            StyledString::simple(
                &format!(" {}  Failed!", &cross_mark_text),
                Some(Color::Red),
                None,
                None,
            ),
        );
        spinner.set_styled_text(
            &spinner4,
            StyledString::simple(
                &format!(" {}  Failed!", &cross_mark_text),
                Some(Color::Red),
                None,
                None,
            ),
        );

        sleep(Duration::from_secs(1));
    }

    sleep(Duration::from_secs(5));
}
