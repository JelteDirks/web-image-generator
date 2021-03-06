use std::path::PathBuf;
use std::fs;
use image::imageops::FilterType;
use image::DynamicImage;
use std::sync::Arc;

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
               original: Arc<ImageRecord>,
               mut output: PathBuf) {
    let (width, height): (u32, u32) = size_description.dimensions;
    let filter: FilterType = set_filter(&size_description.filter);
    let fill_style = size_description.fill.unwrap_or("preserve".to_owned());
    let original_ref = original.as_ref().as_image_ref();

    let new_image: DynamicImage = match fill_style.as_str() {
        "fill" => {
            original_ref.resize_to_fill(
                width,
                height,
                filter)
        },
        "exact" => {
            original_ref.resize_exact(
                width,
                height,
                filter)
        },
        "preserve" | _ => {
            original_ref.resize(
                width,
                height,
                filter)
        },
    };

    let filename = construct_filename(
                width,
                height,
                &fill_style,
                &size_description.filter,
                &size_description.extension,
                &size_description.prepend,
                &size_description.name);

    output.push(filename);
    new_image.save(&output).expect("error saving image to output");
}

fn set_filter(filter: &Option<String>) -> FilterType {
    let default_filter = FilterType::Nearest;
    return match &filter {
        Some(f) => match f.as_str() {
            "Nearest" => FilterType::Nearest,
            "Triangle" => FilterType::Triangle,
            "CatmullRom" => FilterType::CatmullRom,
            "Gaussian" => FilterType::Gaussian,
            "Lanczos3" => FilterType::Lanczos3,
            _ => default_filter,
        },
        None => default_filter,
    };
}

fn construct_filename(width: u32,
                      height: u32,
                      fill_style: &String,
                      filter: &Option<String>,
                      extension: &Option<String>,
                      prepend: &Option<String>,
                      name: &Option<String>) -> String {

    if name.is_some() {
        let mut filename = name.as_ref().unwrap().to_owned();
        filename.push_str(extension.as_ref().unwrap_or(&".jpg".to_owned()));
        return filename.to_string();
    }

    let mut filename = prepend.as_ref().unwrap_or(&String::from("image")).to_owned();
    filename.push_str(&width.to_string());
    filename.push_str("x");
    filename.push_str(&height.to_string());
    filename.push_str(fill_style);
    filename.push_str(filter.as_ref().unwrap());
    filename.push_str(extension.as_ref().unwrap_or(&".jpg".to_owned()));
    return filename;
}
