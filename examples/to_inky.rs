use image::GenericImageView;
use image::imageops::FilterType;
use inky::ColourSpace;
use inky::Palette;
use inky::image_to_bytes;
use inky::quantise_and_dither_image;

fn main() {
    let palette = Palette::from(
        [
            [0, 0, 0],
            [161, 164, 165],
            [208, 190, 71],
            [156, 72, 75],
            [61, 59, 94],
            [58, 91, 70],
        ]
        .as_slice(),
    );
    let mut args = std::env::args();
    args.next(); // throw away program name
    let input_path = args.next().expect("usage: inky <input_path> <output_path>");

    let img = image::open(&input_path).unwrap();
    println!(
        "Using image '{}' with dimensions: {:?}",
        &input_path,
        img.dimensions()
    );
    let img = img.resize(800, 400, FilterType::Nearest);
    let buf = img.to_rgb8();
    println!("Quantising and dithering...");
    let res = quantise_and_dither_image(&buf, &palette, ColourSpace::RGB);
    println!("Done");

    let bytes = image_to_bytes(&res, &palette);
    println!("Got some bytes: '{:?}'", &bytes);
}
