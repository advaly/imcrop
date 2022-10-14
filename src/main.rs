use image::{imageops::{FilterType, self}, DynamicImage};
use std::{env, cmp};
use regex::Regex;
use clap::{App, Arg};

fn main() {
    // Parse command line arguments
    let args = App::new("imcrop")
        // headers
        .version(env!("CARGO_PKG_VERSION"))
        .author("ADVALY SYSTEM Inc.")
        .about("Image manipulation tool")

        // options
        .arg(Arg::with_name("src file")
            .help("Input image file")
            .required(true)    
        )
        .arg(Arg::with_name("out file")
            .help("Output image file")
            .required(true)
            .takes_value(true)
        )
        .arg(Arg::with_name("canvas")
            .short("b").long("canvas")
            .help("Overlay input image on canvas with geometory 'WxH'")
            .takes_value(true)
        )
        .arg(Arg::with_name("crop")
            .short("c").long("crop")
            .help("Cropping geometory 'WxH+X+Y'")
            .takes_value(true)
        )
        .arg(Arg::with_name("contrast")
            .long("contrast")
            .help("Adjust contrast by f32. Negative values decrease and positive values increase")
            .takes_value(true)
        )
        .arg(Arg::with_name("brightness")
            .long("brightness")
            .help("Adjust brightness by i32. Negative values decrease and positive values increase")
            .takes_value(true)
        )
        .arg(Arg::with_name("resize")
            .short("s").long("resize")
            .help("Resizing geometory 'WxH'")
            .takes_value(true)
        )
        .arg(Arg::with_name("rotate")
            .short("r").long("rotate")
            .help("Rotate 90/180/270")
            .takes_value(true)
        )
        .arg(Arg::with_name("overlay file")
            .short("o").long("overlay")
            .help("Overlay a transparent image on the final image")
            .takes_value(true)
        )
        .get_matches();

    let src_file = args.value_of("src file").unwrap();
    let out_file = args.value_of("out file").unwrap();

    // Load input image
    let mut im = image::open(src_file).unwrap();

    // Rotate
    if let Some(rotate) = args.value_of("rotate") {
        im = match rotate {
            "90" => im.rotate90(),
            "180" => im.rotate180(),
            "270" => im.rotate270(),
            _ => im
        };
    }

    // Crop region
    if let Some(region) = args.value_of("crop") {
        let re = Regex::new(r"(\d+)x(\d+)\+(\d+)\+(\d+)").unwrap();
        let caps = re.captures(region).unwrap();
        let crop_w: u32 = caps[1].parse().unwrap();
        let crop_h: u32 = caps[2].parse().unwrap();
        let crop_x: u32 = caps[3].parse().unwrap();
        let crop_y: u32 = caps[4].parse().unwrap();
        im = im.crop(crop_x, crop_y, crop_w, crop_h);
    }

    // Resize
    if let Some(resize) = args.value_of("resize") {
        let re = Regex::new(r"(\d+)x(\d+)").unwrap();
        let caps = re.captures(resize).unwrap();
        let resize_w: u32 = caps[1].parse().unwrap();
        let resize_h: u32 = caps[2].parse().unwrap();
        im = im.resize(resize_w, resize_h, FilterType::Triangle);
    }

    // Brightness
    if let Some(brightness) = args.value_of("brightness") {
        let b: i32 = brightness.parse().unwrap();
        imageops::colorops::brighten_in_place(&mut im, b);
    }

    // Contrast
    if let Some(contrast) = args.value_of("contrast") {
        let c: f32 = contrast.parse().unwrap();
        imageops::colorops::contrast_in_place(&mut im, c);
    }

    // Canvas
    if let Some(canvas) = args.value_of("canvas") {
        let re = Regex::new(r"(\d+)x(\d+)").unwrap();
        let caps = re.captures(canvas).unwrap();
        let canvas_w: u32 = cmp::max(caps[1].parse().unwrap(), im.width());
        let canvas_h: u32 = cmp::max(caps[2].parse().unwrap(), im.height());
        let mut im_canvas = DynamicImage::new_rgb8(canvas_w, canvas_h);
        imageops::overlay(&mut im_canvas, &im, 0, 0);
        im = im_canvas;
    }
    
    // Overlay
    if let Some(overlay_file) = args.value_of("overlay file") {
        let im_overlay = image::open(overlay_file).unwrap();
        imageops::overlay(&mut im, &im_overlay, 0, 0);
    }

    // Save output image
    im.save(out_file).unwrap();
}
