use anyhow::Context;
use anyhow::Result;
use image::GenericImageView;
use inky::ColourSpace;
use inky::Palette;
use inky::quantise_and_dither_image;

fn main() -> Result<()> {
    let palette = Palette::from_tinted_scheme_yaml("./themes/tokyo-night-dark.yaml")?;
    let mut args = std::env::args();
    args.next(); // throw away program name
    let input_path = args
        .next()
        .context("usage: inky <input_path> <output_path>")?;
    let output_path = args
        .next()
        .context("usage: inky <input_path> <output_path>")?;

    let img =
        image::open(&input_path).context(format!("Failed to open image at '{input_path}'"))?;
    println!(
        "Using image '{}' with dimensions: {:?}",
        &input_path,
        img.dimensions()
    );
    let buf = img.to_rgb8();
    println!("Dithering...");
    let res = quantise_and_dither_image(&buf, &palette, ColourSpace::RGB);
    println!("Done");

    res.save(&output_path)
        .context(format!("Failed to write image to '{output_path}'"))?;
    println!("Wrote output to '{}'", &output_path);

    Ok(())
}
