// ABOUTME: Eink image converter utility that optimizes images for eink displays
// ABOUTME: Applies dithering, contrast enhancement, and grayscale conversion

use clap::{Arg, Command};
use image::{DynamicImage, ImageBuffer, Luma};
use indicatif::{ProgressBar, ProgressStyle};

fn main() {
    let matches = Command::new("eink-image")
        .version("0.2.0")
        .about("Convert images for optimal eink display rendering")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .value_name("FILE")
                .help("Input image file")
                .required(true),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .help("Output image file")
                .required(true),
        )
        .arg(
            Arg::new("contrast")
                .short('c')
                .long("contrast")
                .value_name("LEVEL")
                .help("Contrast enhancement level (0.0-2.0)")
                .default_value("1.3"),
        )
        .arg(
            Arg::new("no-dither")
                .long("no-dither")
                .help("Disable Floyd-Steinberg dithering")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("diffusion")
                .long("diffusion")
                .value_name("AMOUNT")
                .help("Error diffusion amount (0.0-1.0)")
                .default_value("0.8"),
        )
        .arg(
            Arg::new("gamma")
                .short('g')
                .long("gamma")
                .value_name("GAMMA")
                .help("Gamma correction value")
                .default_value("2.2"),
        )
        .arg(
            Arg::new("threshold")
                .short('t')
                .long("threshold")
                .value_name("LEVEL")
                .help("Dithering threshold (0-255)")
                .default_value("128"),
        )
        .get_matches();

    let input_path = matches.get_one::<String>("input").unwrap();
    let output_path = matches.get_one::<String>("output").unwrap();
    let contrast_level: f32 = matches
        .get_one::<String>("contrast")
        .unwrap()
        .parse()
        .unwrap_or(1.3);
    let enable_dither = !matches.get_flag("no-dither");
    let diffusion_amount: f32 = matches
        .get_one::<String>("diffusion")
        .unwrap()
        .parse()
        .unwrap_or(0.8);
    let gamma: f32 = matches
        .get_one::<String>("gamma")
        .unwrap()
        .parse()
        .unwrap_or(2.2);
    let threshold: u8 = matches
        .get_one::<String>("threshold")
        .unwrap()
        .parse()
        .unwrap_or(128);

    let pb = ProgressBar::new(100);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} {msg}",
            )
            .unwrap()
            .progress_chars("#>-"),
    );
    pb.set_message("Processing image...");

    match process_image(
        input_path,
        output_path,
        contrast_level,
        enable_dither,
        diffusion_amount,
        gamma,
        threshold,
        &pb,
    ) {
        Ok(_) => {
            pb.finish_with_message("Image processed successfully!");
            println!("Output saved to: {}", output_path);
        }
        Err(e) => {
            pb.finish_with_message("Processing failed");
            eprintln!("Error processing image: {}", e);
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn process_image(
    input_path: &str,
    output_path: &str,
    contrast_level: f32,
    enable_dither: bool,
    diffusion_amount: f32,
    gamma: f32,
    threshold: u8,
    pb: &ProgressBar,
) -> Result<(), Box<dyn std::error::Error>> {
    pb.set_message("Loading image...");
    let img = image::open(input_path)?;
    pb.set_position(20);

    pb.set_message("Converting to grayscale...");
    let grayscale_img = convert_to_grayscale(img);
    pb.set_position(40);

    pb.set_message("Applying gamma correction...");
    let gamma_corrected_img = apply_gamma_correction(grayscale_img, gamma);
    pb.set_position(60);

    pb.set_message("Enhancing contrast...");
    let enhanced_img = enhance_contrast(gamma_corrected_img, contrast_level);
    pb.set_position(70);

    pb.set_message(if enable_dither {
        "Applying Floyd-Steinberg dithering..."
    } else {
        "Applying threshold..."
    });
    let final_img = if enable_dither {
        apply_floyd_steinberg_dithering(enhanced_img, diffusion_amount, threshold)
    } else {
        apply_simple_threshold(enhanced_img, threshold)
    };
    pb.set_position(90);

    pb.set_message("Saving output...");
    final_img.save(output_path)?;
    pb.set_position(100);

    Ok(())
}

fn convert_to_grayscale(img: DynamicImage) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    img.to_luma8()
}

fn enhance_contrast(
    img: ImageBuffer<Luma<u8>, Vec<u8>>,
    contrast_level: f32,
) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let (width, height) = img.dimensions();
    let mut result = ImageBuffer::new(width, height);

    for (x, y, pixel) in img.enumerate_pixels() {
        let luminance = pixel[0] as f32 / 255.0;
        let enhanced = ((luminance - 0.5) * contrast_level + 0.5).clamp(0.0, 1.0);
        let new_value = (enhanced * 255.0) as u8;
        result.put_pixel(x, y, Luma([new_value]));
    }

    result
}

fn apply_gamma_correction(
    img: ImageBuffer<Luma<u8>, Vec<u8>>,
    gamma: f32,
) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let (width, height) = img.dimensions();
    let mut result = ImageBuffer::new(width, height);

    let gamma_lut: Vec<u8> = (0..256)
        .map(|i| {
            let normalized = i as f32 / 255.0;
            let corrected = normalized.powf(1.0 / gamma);
            (corrected * 255.0).round() as u8
        })
        .collect();

    for (x, y, pixel) in img.enumerate_pixels() {
        let corrected_value = gamma_lut[pixel[0] as usize];
        result.put_pixel(x, y, Luma([corrected_value]));
    }

    result
}

fn apply_simple_threshold(
    img: ImageBuffer<Luma<u8>, Vec<u8>>,
    threshold: u8,
) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let (width, height) = img.dimensions();
    let mut result = ImageBuffer::new(width, height);

    for (x, y, pixel) in img.enumerate_pixels() {
        let new_value = if pixel[0] >= threshold { 255 } else { 0 };
        result.put_pixel(x, y, Luma([new_value]));
    }

    result
}

fn apply_floyd_steinberg_dithering(
    img: ImageBuffer<Luma<u8>, Vec<u8>>,
    diffusion_amount: f32,
    threshold: u8,
) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let (width, height) = img.dimensions();
    let mut result = img.clone();
    let mut error_buffer: Vec<Vec<f32>> = vec![vec![0.0; width as usize]; height as usize];

    for y in 0..height {
        for x in 0..width {
            let pixel = result.get_pixel(x, y);
            let old_value = pixel[0] as f32 + error_buffer[y as usize][x as usize];
            let new_value = if old_value < threshold as f32 { 0 } else { 255 };
            let error = (old_value - new_value as f32) * diffusion_amount;

            result.put_pixel(x, y, Luma([new_value]));

            if x + 1 < width {
                error_buffer[y as usize][(x + 1) as usize] += error * 7.0 / 16.0;
            }
            if y + 1 < height {
                if x > 0 {
                    error_buffer[(y + 1) as usize][(x - 1) as usize] += error * 3.0 / 16.0;
                }
                error_buffer[(y + 1) as usize][x as usize] += error * 5.0 / 16.0;
                if x + 1 < width {
                    error_buffer[(y + 1) as usize][(x + 1) as usize] += error * 1.0 / 16.0;
                }
            }
        }
    }

    result
}
