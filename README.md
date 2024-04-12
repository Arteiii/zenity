<div align="center">
  <img src="images/ZENITY.svg" alt="Zenity svg logo" width="400">
<p>Yet Another Spinner Lib</p>
<p style="margin-top: -10px;">Elevate your Rust command-line interfaces with 100+ spinner animations and Progress Bars + multiline support</p>
  <a href="https://github.com/Arteiii/zenity/actions/workflows/publish_crate.yml">
    <img src="https://github.com/Arteiii/zenity/actions/workflows/publish_crate.yml/badge.svg" alt="Publish to Crates">
  </a>
  <a href="https://github.com/Arteiii/zenity/actions/workflows/release_examples.yml">
    <img src="https://github.com/Arteiii/zenity/actions/workflows/release_examples.yml/badge.svg" alt="Compile Rust">
  </a>
  <img src="https://img.shields.io/crates/v/zenity" alt="Crates.io Version">
  <img src="https://img.shields.io/docsrs/zenity" alt="docs.rs">
  <br>
  <a href="https://www.codefactor.io/repository/github/arteiii/zenity">
    <img src="https://www.codefactor.io/repository/github/arteiii/zenity/badge" alt="CodeFactor">
  </a>
</div>

![progress bar](./images/rustrover64_WupAJU44Lu.gif)

![multiline preview](./images/rustrover64_4bzlv2mWxK.gif)

Do you often find yourself gazing into the void of your terminal,  
wondering if your computer has decided to take a coffee break without notifying you?

100+ predefined animations

[Spinner](https://docs.rs/zenity/latest/zenity/spinner/frames/struct.Frames.html)
[ProgressBar](https://docs.rs/zenity/latest/zenity/progress/frames/struct.Frames.html)

## How to Use?

It's as easy as pie (or maybe even easier, depending on your pie-making skills)!  
Follow these simple steps:

````shell
cargo add zenity
````

```rust
// example/basic.rs

use std::thread::sleep;
use std::time::Duration;
use zenity::spinner::MultiSpinner;

fn main() {
    // create a LoadingAnimation instance using one of the predefined animations
    let spinner = MultiSpinner::default();
    sleep(Duration::from_secs(5));

    // optional:
    spinner.set_text(&spinner.get_last(), "  Loading...".to_string()); // sets the text to "Loading..."

    sleep(Duration::from_secs(500));
    // `loading_animation` will run out of scope now and get dropped,
    // thus the animation will stop and remove itself from the console
}
```

check out the examples for more

**NOTE:**

- the lib already includes checks for `--color` following the conventions
  by:
  [Rain's Rust CLI recommendations](https://rust-cli-recommendations.sunshowers.io/colors.html#general-recommendations)

## Disclaimer

Now, we won't promise you that cli_loading_magic will solve all your problems.  
We can't guarantee it won't crash your terminal or cause your computer to sprout legs and walk away in protest.  

But hey, life's an adventure, right?  
Embrace the chaos, enjoy the ride, and remember: if all else fails, there's always  
**Ctrl+C.**

## Contributing

Contributions, bug reports, feature requests, and suggestions are all welcome!

If you encounter any issues or have ideas for improvements, please don't hesitate to open
an [issue on GitHub](https://github.com/Arteiii/zenity/issues/new).  
[Pull requests](https://github.com/Arteiii/zenity/pulls) are also highly appreciated.  
If you find this project helpful or enjoyable, consider giving it a star on [GitHub](https://github.com/Arteiii/zenity).

Thank you for your interest and contributions!

## Credits

This project wouldn't have been possible without the amazing work of the following projects:

### Animations

These projects not only provided delightful animations but also spared me from the arduous task of creating frames
myself

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
