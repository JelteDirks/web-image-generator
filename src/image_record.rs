use image::io::Reader as ImageReader;
use image::DynamicImage;
use std::fs::File;
use std::io::{BufReader};
use std::path::PathBuf;

pub struct ImageRecord {
    content: DynamicImage,
}

impl ImageRecord {
    pub fn new(p: PathBuf) -> ImageRecord {
        let file = match File::open(&p) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("error opening file: {:?}", &p);
                panic!("{}", e);
            },
        };
        let imagereader = ImageReader::new(BufReader::new(file));
        let formatted = imagereader.with_guessed_format().unwrap();
        let image = formatted.decode().unwrap();
        return ImageRecord {
            content: image,
        };
    }

    pub fn as_image_ref(&self) -> &DynamicImage {
        return &self.content;
    }
}
