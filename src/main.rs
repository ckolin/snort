extern crate image;

use crate::image::{GenericImage, GenericImageView, ImageBuffer, Pixel, Rgb};
use std::{env, process};

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() < 4 {
        eprintln!("Usage: snort <input> <output> <threshold>");
        process::exit(1);
    }
    let input = &args[1];
    let output = &args[2];
    let threshold_value = args[3]
        .parse::<u8>()
        .expect("Use threshold value from 0 - 255");

    let mut img = image::open(input).expect("Could not open image").to_rgb();
    for y in 0..img.height() {
        let mut start = 0;
        for x in 0..img.width() {
            if threshold(img.get_pixel(x, y), threshold_value) {
                if start + 1 < x {
                    sort_interval(&mut img, y, start, x);
                }
                start = x;
            }
        }
    }

    img.save(output).expect("Could not save image");
}

fn threshold(pixel: &Rgb<u8>, value: u8) -> bool {
    let luma = pixel.to_luma().0[0];
    luma < value || luma > 255 - value
}

fn sort_interval(img: &mut ImageBuffer<image::Rgb<u8>, Vec<u8>>, y: u32, start: u32, end: u32) {
    let sub_img = img.sub_image(start, y, end - start + 1, 1);
    let mut pixels = sub_img.pixels().map(|(_, _, p)| p).collect::<Vec<_>>();
    pixels.sort_unstable_by_key(|p| p.to_luma().0);
    for px in start..end {
        let i = (px - start) as usize;
        img.put_pixel(px, y, pixels[i]);
    }
}
