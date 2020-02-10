extern crate clap;
extern crate image;
use clap::{App, Arg};

use image::error::ImageResult;
use image::imageops::FilterType;
use image::{DynamicImage, GenericImageView, ImageFormat};
use std::fs::File;
use std::path::Path;

pub fn resize_with_target(img: DynamicImage, target_size: usize) -> Result<DynamicImage, String> {
    let width = img.width() as usize;
    let height = img.height() as usize;

    let (target_width, target_height) = if width > height {
        let ratio: f32 = target_size as f32 / width as f32;
        (target_size, (height as f32 * ratio) as usize)
    } else {
        let ratio: f32 = target_size as f32 / height as f32;
        ((width as f32 * ratio) as usize, target_size)
    };

    let resized_img = img.resize(
        target_width as u32,
        target_height as u32,
        FilterType::Lanczos3,
    );

    Ok(resized_img)
}

struct Config {
    file_path: String,
    img: DynamicImage,
}

impl Config {
    pub fn new(file_path: &str, img: DynamicImage) -> Config {
        Config {
            file_path: file_path.to_string(),
            img: img,
        }
    }

    pub fn write_to_path(self, format: ImageFormat) -> ImageResult<()> {
        let path = Path::new(&self.file_path);
        let mut output = File::create(format!(
            "output/resizd-{}.png",
            path.file_stem().unwrap().to_str().unwrap(),
        ))
        .unwrap();

        self.img.write_to(&mut output, format)
    }
}

fn main() {
    let matches = App::new("png-resizer")
        .version("1.0")
        .author("poccariswet <poccariswet@gmail.com>")
        .about("png resize command line interface")
        .arg(
            Arg::with_name("target-size")
                .short("t")
                .long("target-size")
                .value_name("NUM")
                .help("Sets a target size value")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("width")
                .short("w")
                .long("width")
                .value_name("NUM")
                .help("Sets a width value")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("height")
                .short("h")
                .long("height")
                .value_name("NUM")
                .help("Sets a height value")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("input")
                .value_name("FILES")
                .help("Sets for resize files")
                .index(1)
                .multiple(true)
                .required(true),
        )
        .get_matches();

    // This target-size doesn't override the width and height.
    let resized_images = if matches.is_present("width") && matches.is_present("height") {
        let mut resized_imgs = vec![];
        for path in matches.values_of("input").unwrap() {
            let img = image::open(path).unwrap();
            let w = matches.value_of("width").unwrap();
            let h = matches.value_of("height").unwrap();

            let w: u32 = w.parse().unwrap();
            let h: u32 = h.parse().unwrap();

            let resized_img = img.resize(w, h, FilterType::Lanczos3);
            resized_imgs.push(Config::new(path, resized_img));
        }
        resized_imgs
    } else if matches.is_present("target-size") {
        let mut resized_imgs = vec![];
        for path in matches.values_of("input").unwrap() {
            let img = image::open(path).unwrap();
            let target_size = matches.value_of("target-size").unwrap();
            let target_size: usize = target_size.parse().unwrap();

            let resized_img = resize_with_target(img, target_size).unwrap();
            resized_imgs.push(Config::new(path, resized_img));
        }
        resized_imgs
    } else {
        vec![]
    };

    for img in resized_images {
        img.write_to_path(ImageFormat::Png).unwrap();
    }
}
