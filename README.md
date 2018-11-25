# Deprecated

This repo is not actively maintained. The rust sdl2 project should have up to date example code.


# Rust SDL2 Examples

This is a collection of examples of the [Rust bindings for SDL](https://github.com/AngryLawyer/rust-sdl2).

## Setup

You will need [Rust](http://www.rust-lang.org/) and [SDL2](http://www.libsdl.org) on your system.
For some examples, you'll need extra SDL libraries as noted below.
That is, if an example requires the Rust SDL2 image library, you have to
install the corresponding low level C language SDL2 image library on your system too.

Build the examples via [Cargo](http://crates.io/)

> cargo build

Some examples will require assets so run each example from the root of the project.

> cargo run --bin eg04

## [Eg01](src/eg01.rs)

Displays a window for a moment then quits.

## [Eg02](src/eg02.rs)

Handling events.

## [Eg03](src/eg03.rs)

Rendering shapes.

## [Eg04](src/eg04.rs)

Surface rendering.

## [Eg05](src/eg05.rs)

Texture rendering.

## [Eg06](src/eg06.rs)

Shows how to load images in PNG format using [SDL2
image](https://github.com/xsleonard/rust-sdl2_image), and scale them to fill a
tiled background.

## [Eg07](src/eg07.rs)

Shows how to display text using [SDL2 ttf](https://github.com/andelf/rust-sdl2_ttf).

## Acknowledgments

Eg06 and 07 are ports of [TwinklebearDev SDL 2.0 Tutorial](http://www.willusher.io/pages/sdl2/) to Rust.
