extern crate image;
extern crate regex;
use std::env;
use std::process::exit;
use regex::Regex;

fn main() {
    // Parse arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: imcrop geometry src_file out_file");
        exit(1);
    }

    let region = &args[1];
    let src_file = &args[2];
    let out_file = &args[3];

    // Parse cropping region
    let re = Regex::new(r"(\d+)x(\d+)\+(\d+)\+(\d+)").unwrap();
    let caps = re.captures(region).unwrap();
    let width: u32 = caps[1].parse().unwrap();
    let height: u32 = caps[2].parse().unwrap();
    let crop_x: u32 = caps[3].parse().unwrap();
    let crop_y: u32 = caps[4].parse().unwrap();

    // Crop and save
    image::open(src_file).unwrap().crop(crop_x, crop_y, width, height).save(out_file).unwrap();
}
