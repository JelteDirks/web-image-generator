use image::io::Reader as ImageReader;
use image::ImageFormat;
use image::DynamicImage;
use std::fs::File;
use std::io::{BufReader};
use std::path::PathBuf;

pub struct ImageRecord {
    path: PathBuf,
    content: DynamicImage,
}

impl ImageRecord {
    pub fn new(p: PathBuf) -> ImageRecord {

        // create a file from our input
        let file = match File::open(&p) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("error opening file: {:?}", &p);
                panic!("{}", e);
            },
        };

        // file exists

        let imagereader = ImageReader::new(BufReader::new(file));
        let formatted = imagereader.with_guessed_format().unwrap();
        let image = formatted.decode().unwrap();

        return ImageRecord {
            path: p,
            content: image,
        };
    }

    pub fn get_path(&self) -> &PathBuf {
        return &self.path;
    }
}
