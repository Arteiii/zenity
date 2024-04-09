# Changes in API version 2.0.0

I'll be straight with you, I may have gone a bit overboard with the last one

The source code was a bit of a mess

## Here's what's changed

### Modularization of Features

- Animations: the aniamtions module got replaced by 2 new module (spinner and progress) each with their respective frames


### Style Modules

The style modules, previously re-exported at the library base, have been moved into the style module (this change also impacts re-exports from crossterm colors)


### Following Rust API Guidelines

Oh, and by the way, I did my darnedest to follow the Rust API Guidelines.


changes for the examples:

