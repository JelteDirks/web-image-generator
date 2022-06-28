use image::io::Reader as ImageReader;
use image::ImageFormat;
use std::fs::File;
use std::io::{BufReader};
use std::path::PathBuf;

pub struct ImageRecord {
    path: PathBuf,
    image: ImageReader<BufReader<File>>,
    dimensions: (u32, u32),
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

        // create a clone from the file, this uses the same file descriptor as
        // the original, so operations technically affect both. we only need
        // this for dimensions though...
        let f_clone = match file.try_clone() {
            Ok(f) => {
                println!("successfully cloned the file");
                f
            }
            Err(e) => panic!("{}", e),
        };

        // create an image to store in our record
        let original = ImageReader::new(BufReader::new(file));
        // create a clone so we can set the dimensions
        let clone = ImageReader::new(BufReader::new(f_clone));
        // try guessing the format, sets it automatically
        let clone = clone.with_guessed_format().unwrap();

        // set the dimensions of this image, consumes the image reader
        let dimensions = match clone.into_dimensions() {
            Ok(d) => d,
            Err(e) => panic!("{}", e),
        };

        return ImageRecord {
            path: p,
            image: original,
            dimensions
        };
    }

    pub fn get_dimensions(&self) -> &(u32, u32) {
        return &self.dimensions;
    }

    pub fn get_path(&self) -> &PathBuf {
        return &self.path;
    }
}
