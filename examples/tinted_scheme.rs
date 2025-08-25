use anyhow::Context;
use anyhow::Result;
use dithering::ColourSpace;
use dithering::Palette;
use dithering::quantise_and_dither_image;
use dithering::themes::BASE16_TOKYO_NIGHT_DARK;
use image::GenericImageView;

fn main() -> Result<()> {
    let palette = Palette::from(BASE16_TOKYO_NIGHT_DARK.as_slice());
    let mut args = std::env::args();
    args.next(); // throw away program name
    let input_path = args
        .next()
        .context("usage: dithering <input_path> <output_path>")?;
    let output_path = args
        .next()
        .context("usage: dithering <input_path> <output_path>")?;

    let img =
        image::open(&input_path).context(format!("Failed to open image at '{input_path}'"))?;
    println!(
        "Using image '{}' with dimensions: {:?}",
        &input_path,
        img.dimensions()
    );
    let mut buf = img.to_rgb8();
    println!("Dithering...");
    quantise_and_dither_image(&mut buf, &palette, ColourSpace::RGB);
    println!("Done");

    buf.save(&output_path)
        .context(format!("Failed to write image to '{output_path}'"))?;
    println!("Wrote output to '{}'", &output_path);

    Ok(())
}
