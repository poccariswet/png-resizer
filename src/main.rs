extern crate clap;
use clap::{App, Arg};

use image::imageops::FilterType;
use image::ImageFormat;
use image::{self, GenericImageView};
use std::fs::File;

//const TARGET_SIZE: usize = 250;

fn resize(path: &str, target_size: usize) -> Result<(Vec<u8>, usize, usize), String> {
    //fn resize(path: &str, target_size: usize) -> Result<(Vec<u8>, usize, usize), String> {
    let img = image::open(path).unwrap();
    let width = img.width() as usize;
    let height = img.height() as usize;

    if width > target_size || height > target_size {
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

        let mut output = File::create(format!("output/sample-{}.png", target_size)).unwrap();
        resized_img.write_to(&mut output, ImageFormat::Png).unwrap();

        Ok((
            resized_img.to_rgb().to_vec(),
            resized_img.width() as usize,
            resized_img.height() as usize,
        ))
    } else {
        Ok((img.to_rgb().to_vec(), width, height))
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
                .takes_value(true)
                .required(true),
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
                .long("hegiht")
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

    let mut target_size: usize = 0;
    if let Some(target) = matches.value_of("target-size") {
        target_size = target.parse::<usize>().unwrap();
    }
    println!("{}", target_size);

    if let Some(in_files) = matches.values_of("input") {
        for file in in_files {
            println!("An input file: {}", file);
            resize(file, target_size).unwrap();
        }
    }

    if let Some(c) = matches.value_of("width") {
        println!("Value for width: {}", c);
    }

    if let Some(c) = matches.value_of("height") {
        println!("Value for height: {}", c);
    }
}
