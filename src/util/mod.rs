use std::path::PathBuf;
use std::fs;
use std::time::Instant;
use image::imageops::FilterType;
use image::DynamicImage;

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
    let default_filter = FilterType::Nearest;

    let filter: FilterType = match size_description.filter {
        Some(f) => match f.as_str() {
            "Gaussian" => FilterType::Gaussian,
            "Nearest" => FilterType::Gaussian,
            "Triangle" => FilterType::Gaussian,
            "CatmullRom" => FilterType::Gaussian,
            "Lanczos3" => FilterType::Gaussian,
            _ => default_filter,
        },
        None => default_filter,
    };

    let fill_style = size_description.fill.unwrap_or("preserve".to_owned());
    let original_ref = original.as_ref();

    println!("resizing to width={:?},height={:?},filter={:?},style={:?}",
             &width,
             &height,
             &filter,
             &fill_style);

    let now = Instant::now();

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

    println!("resizing took {:?}", now.elapsed());

    output.push(filename);
    new_image.save(&output).expect("error saving image to output");

    println!("saving took {:?}", now.elapsed().as_millis());
}


fn construct_filename(width: u32, height: u32) -> String {
    let mut filename = width.to_string();
    filename.push_str("x");
    filename.push_str(&height.to_string());
    filename.push_str(".png");
    return filename;
}
