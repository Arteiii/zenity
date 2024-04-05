use std::thread::sleep;
use std::time::Duration;

use unicode_icons::symbols::{check_mark_button,cross_mark};

use zenity::multi_spinner::MultiSpinner;
use zenity::spinner::PreDefined;

fn main() {
    let check_mark_text = check_mark_button();
    let cross_mark_text = cross_mark();


    let mut spinner = MultiSpinner::new();

    // main thread operations
    let spinner1 = spinner.add(PreDefined::dot_spinner11(false));
    let spinner2 = spinner.add(PreDefined::binary(false));
    let spinner3 = spinner.add(PreDefined::dot_spinner9(false));
    let spinner4 = spinner.add(PreDefined::dot_spinner8(false));

    // access the spinner through the Arc<Mutex<MultiSpinner>> reference
    spinner.run_all();

    sleep(Duration::from_secs(2));


    sleep(Duration::from_secs(8));
    spinner.set_text(&spinner2, "spinner2".to_string());
    // stop spinner1
    spinner.set_text(&spinner1, "spinner1".to_string());

    sleep(Duration::from_secs(2));

    spinner.stop(&spinner2);
    spinner.set_text(&spinner2, format!("{}   Successfully", &check_mark_text));


    sleep(Duration::from_secs(2));
    spinner.set_text(&spinner1, "spinner1 stopped".to_string());

    spinner.stop(&spinner1);

    sleep(Duration::from_secs(9));
    spinner.stop(&spinner3);
    spinner.stop(&spinner4);
    
    spinner.set_text(&spinner3, format!("{}   Failed!", &cross_mark_text));
    spinner.set_text(&spinner4, format!("{}   Failed!", &cross_mark_text));


    sleep(Duration::from_secs(3000000));
}
