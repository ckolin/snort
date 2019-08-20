extern crate image;

use crate::image::{GenericImage, GenericImageView, ImageBuffer, Pixel, Rgb};
use std::{env, process};

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() < 5 {
        eprintln!("Usage: snort <input> <output> <lower bound> <upper bound> [rotations]");
        process::exit(1);
    }
    
    let input = &args[1];
    let output = &args[2];
    let lower_bound = parse_u8(&args[3]);
    let upper_bound = parse_u8(&args[4]);
    let rotations = if args.len() > 5 {
        parse_u8(&args[5]) % 4
    } else {
        0
    };

    let mut img = image::open(input).expect("Could not open image").to_rgb();

    for _ in 0..rotations {
        img = image::imageops::rotate90(&img);
    }
    
    let (width, height) = img.dimensions();
    for y in 0..height {
        let mut start = 0;
        for x in 0..width {
            if threshold(img.get_pixel(x, y), lower_bound, upper_bound) {
                sort_interval(&mut img, y, start, x);
                start = x;
            }
        }
        sort_interval(&mut img, y, start, width - 1);
    }

    if rotations > 0 {
        for _ in 0..(4 - rotations) {
            img = image::imageops::rotate90(&img);
        }
    }
    
    img.save(output).expect("Could not save image");
}

fn parse_u8(arg: &str) -> u8 {
    arg.parse::<u8>()
        .expect("Use numerical values from 0 - 255")
}

fn threshold(pixel: &Rgb<u8>, lower_bound: u8, upper_bound: u8) -> bool {
    let luma = pixel.to_luma().0[0];
    luma < lower_bound || luma > upper_bound
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
