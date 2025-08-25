# Dithering

A simple, lightweight crate for image quantisation and dithering in Rust.

This crate provides utilities for:

- Creating and manipulating colour palettes.
- Quantising images to a given palette.
- Applying [Floyd–Steinberg dithering](https://en.wikipedia.org/wiki/Floyd%E2%80%93Steinberg_dithering) for smoother visual results.
- Compactly encoding images into byte arrays for storage or transmission.

## Features

- Multiple colour spaces for distance calculations.
- Palette creation from raw arrays, hex text, and from Tinted Themes.
- Optional themes support (via the `theme` feature).

## Example

Using a simple palette:

```no_run
use dithering::{Palette, ColourSpace, quantise_and_dither_image};
use image::open;

fn main() -> anyhow::Result<()> {
    // Load an image
    let mut img = open("example.png")?.to_rgb8();

    // Define a simple palette (red, green, blue)
    let palette = Palette::from([[255, 0, 0], [0, 255, 0], [0, 0, 255]].as_slice());

    // Quantise and dither the image in CIELAB space
    quantise_and_dither_image(&mut img, &palette, ColourSpace::CIELAB);

    // Save result
    img.save("output.png")?;
    Ok(())
}
```

Using a palette built from [Tinted Theming](https://github.com/tinted-theming/schemes):

```no_run
use dithering::{Palette, ColourSpace, quantise_and_dither_image};
use dithering::themes::BASE16_TOKYO_NIGHT_DARK;
use image::open;

fn main() -> anyhow::Result<()> {
    // Load an image
    let mut img = open("example.png")?.to_rgb8();

    // Define a simple palette (red, green, blue)
    let palette = Palette::from(BASE16_TOKYO_NIGHT_DARK.as_slice());

    // Quantise and dither the image in RGB space
    quantise_and_dither_image(&mut img, &palette, ColourSpace::RGB);

    // Save result
    img.save("output.png")?;
    Ok(())
}
```
