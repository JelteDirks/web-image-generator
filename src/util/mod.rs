use std::path::PathBuf;
use std::fs;
use image::imageops::FilterType;

use crate::config::SizeDescription;
use crate::image_record::ImageRecord;

pub fn check_output(path: &PathBuf) {
    let as_path = path.as_path();
    let path_exists = as_path.exists();

    if !path_exists {
        fs::create_dir_all(&as_path).expect("error creating output directory");
    } else if !as_path.is_dir(){
        // exists but is not a directory, problem!
        panic!("output path is not a directory {:?}", as_path);
    }
}

pub fn convert(size_description: SizeDescription,
               original: &ImageRecord,
               mut output: PathBuf) {
    let (width, height): (u32, u32) = size_description.dimensions;
    let filename = construct_filename(width, height);

    // make resize an option in the config (fill / exact / fit)
    let new_image = original.as_ref().resize_to_fill(
        width,
        height,
        FilterType::Gaussian);

    output.push(filename);
    new_image.save(&output).expect("error saving image to output");
}


fn construct_filename(width: u32, height: u32) -> String {
    let mut filename = width.to_string();
    filename.push_str("x");
    filename.push_str(&height.to_string());
    filename.push_str(".png");
    return filename;
}
