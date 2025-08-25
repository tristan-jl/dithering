use anyhow::Result;
use dithering::ColourSpace;
use dithering::Palette;
use dithering::quantise_and_dither_image;
use image::GenericImageView;
use image::imageops::FilterType;

fn main() -> Result<()> {
    let palette = Palette::from_hex_text(
        "
    2f321b
    64472e
    b56036
    e25462
    fccb65
    1c3abe
    45b3c3
    fdafa0
    dec69c
    b4c9de",
    )?;
    let mut args = std::env::args();
    args.next(); // throw away program name
    let input_path = args
        .next()
        .expect("usage: dithering <input_path> <output_path>");
    let output_path = args
        .next()
        .expect("usage: dithering <input_path> <output_path>");

    let img = image::open(&input_path).unwrap();
    println!(
        "Using image '{}' with dimensions: {:?}",
        &input_path,
        img.dimensions()
    );
    let img = img.resize(800, 400, FilterType::Nearest);
    let mut buf = img.to_rgb8();
    println!("Quantising and dithering...");
    quantise_and_dither_image(&mut buf, &palette, ColourSpace::RGB);
    println!("Done");

    buf.save(&output_path).unwrap();
    println!("Wrote output to '{}'", &output_path);

    Ok(())
}
