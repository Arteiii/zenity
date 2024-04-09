# zenity (Yet Another Spinner Lib)

Elevate your Rust command-line interfaces with 100+ spinner animations and Progress Bars + multiline support

[![Publish to Crates](https://github.com/Arteiii/zenity/actions/workflows/publish_crate.yml/badge.svg)](https://github.com/Arteiii/zenity/actions/workflows/publish_crate.yml)
[![Compile Rust](https://github.com/Arteiii/zenity/actions/workflows/release_examples.yml/badge.svg)](https://github.com/Arteiii/zenity/actions/workflows/release_examples.yml)
![Crates.io Version](https://img.shields.io/crates/v/zenity)
![docs.rs](https://img.shields.io/docsrs/zenity)

[![CodeFactor](https://www.codefactor.io/repository/github/arteiii/zenity/badge)](https://www.codefactor.io/repository/github/arteiii/zenity)

![multiline preview](./images/rustrover64_4bzlv2mWxK.gif)

![](./images/rustrover64_tlGiHM9JP0.gif)

![progress bar](./images/rustrover64_WupAJU44Lu.gif)



[**CHANGELOG**](CHANGELOG.md)

Do you often find yourself gazing into the void of your terminal, 
wondering if your computer has decided to take a coffee break without notifying you?

100+ predefined animations

[List Of Animations](https://docs.rs/zenity/latest/zenity/animations/frames/spinner/struct.PreDefined.html)


## Introducing

After countless late nights spent wrestling with terminal buffers and ASCII art,
after enduring more trial and error than we care to admit,
we proudly present to you our humble attempt to transform loading animations from a headache into a delight

## How to Use?

It's as easy as pie (or maybe even easier, depending on your pie-making skills)! Just follow these simple steps:

```rust
// example/basic.rs

use std::thread::sleep;
use std::time::Duration;
use zenity::{style::Color, LoadingAnimation};

fn main() {
    println!("println test");
    scope_example();
}

fn scope_example() {
    // create a LoadingAnimation instance using one of the predefined animations
    let spinner = LoadingAnimation::default();

    // optional:
    spinner.set_text("Loading..."); // sets the text to "Loading..."
    spinner.set_text_color(Color::DarkBlue); // set the color

    sleep(Duration::from_secs(5));
    // `loading_animation` will run out of scope now and get dropped,
    // thus the animation will stop and remove itself from the console
}
```

check out the examples for more

**NOTE:**

- the lib already includes checks for `--color` following the conventions by: [Rain's Rust CLI recommendations](https://rust-cli-recommendations.sunshowers.io/colors.html#general-recommendations)


## Disclaimer

Now, we won't promise you that cli_loading_magic will solve all your problems.
We can't guarantee it won't crash your terminal or cause your computer to sprout legs and walk away in protest.
But hey, life's an adventure, right? Embrace the chaos, enjoy the ride, and remember: if all else fails, there's always Ctrl+C.

Feel free to create an issue with suggestions or bug reports.

## Credits

This project wouldn't have been possible without the amazing work of the following projects:

### Animations

These projects not only provided delightful animations but also spared me from the arduous task of creating frames myself (who wants to do that anyway?)

- [Spinner](https://github.com/FGRibreau/spinners)
- [spinoff](https://github.com/ad4mx/spinoff)

### Documentation

- [ChatGPT](https://chat.openai.com/)

### Dependencies

- [Crossterm](https://github.com/crossterm-rs/crossterm)
- [supports_color](https://docs.rs/supports-color/latest/supports_color/)


## License

This project is licensed under the **DWFYW** License.

For more information, see the [LICENSE](LICENSE.md) file.

### Copyright (c) 2024 Ben
