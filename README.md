# zenity (Yet Another Spinner Lib)

![](./images/WindowsTerminal_ZlTLEYK249.gif)

easy to use cli animation lib based on crossterm

Do you often find yourself gazing into the void of your terminal, wondering if your computer has decided to take a coffee break without notifying you?

## Introducing

After countless late nights (1) spent wrestling with terminal buffers and ASCII art,
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
    let spinner = LoadingAnimation::default(); // invert frames bool (false)

    spinner.set_text("Loading..."); // sets the text to "Loading..."
    spinner.set_text_color(Color::DarkBlue);

    sleep(Duration::from_secs(5));
    // `loading_animation` will run out of scope now and get dropped,
    // thus the animation will stop and remove itself from the console
}

```

check out the examples for more

But wait, there's more! We're cooking up some extra features,
like the ability to have multiple animations simultaneously and the power to delete specific ones.

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

## License

This project is licensed under the **DWFYW** License.

For more information, see the [LICENSE](LICENSE.md) file.

**Copyright (c) 2024 Ben**
