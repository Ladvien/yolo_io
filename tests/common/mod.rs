use std::{
    fs,
    path::{Path, PathBuf},
};

use hashbrown::HashMap;
use image::{ImageBuffer, Rgb};
use rstest::fixture;
use yolo_io::{Export, Paths, SourcePaths, Split, YoloProjectConfig};

#[allow(dead_code)]
pub const TEST_SANDBOX_DIR: &str = "tests/sandbox";

#[fixture]
pub fn image_data() -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    //! An example of generating julia fractals.
    let imgx = 800;
    let imgy = 800;

    let scalex = 3.0 / imgx as f32;
    let scaley = 3.0 / imgy as f32;

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = (0.3 * x as f32) as u8;
        let b = (0.3 * y as f32) as u8;
        *pixel = image::Rgb([r, 0, b]);
    }

    // A redundant loop to demonstrate reading image data
    for x in 0..imgx {
        for y in 0..imgy {
            let cx = y as f32 * scalex - 1.5;
            let cy = x as f32 * scaley - 1.5;

            let c = num_complex::Complex::new(-0.4, 0.6);
            let mut z = num_complex::Complex::new(cx, cy);

            let mut i = 0;
            while i < 255 && z.norm() <= 2.0 {
                z = z * z + c;
                i += 1;
            }

            let pixel = imgbuf.get_pixel_mut(x, y);
            let image::Rgb(data) = *pixel;
            *pixel = image::Rgb([data[0], i as u8, data[2]]);
        }
    }

    imgbuf
}

#[allow(dead_code)]
pub fn create_dir(path: &str) {
    fs::create_dir_all(path).expect("Unable to create directory");
}

#[allow(dead_code)]
pub fn create_dir_and_write_file(path: &Path, content: &str) {
    fs::create_dir_all(path.parent().unwrap()).expect("Unable to create directory");

    match fs::write(path, content) {
        Ok(_) => (),
        Err(e) => panic!("Unable to write file: {}", e),
    }
}

#[allow(dead_code)]
pub fn create_image_file(path: &Path, image_data: &ImageBuffer<Rgb<u8>, Vec<u8>>) {
    fs::create_dir_all(path.parent().unwrap()).expect("Unable to create directory");
    image_data.save(path).expect("Unable to write file");
}

#[allow(dead_code)]
pub fn create_yolo_label_file(path: &Path, content: &str) {
    fs::create_dir_all(path.parent().unwrap()).expect("Unable to create directory");
    fs::write(path, content).expect("Unable to write file");
}

#[fixture]
pub fn create_yolo_project_config() -> YoloProjectConfig {
    let mut class_map = HashMap::new();

    class_map.insert(0, "person".to_string());
    class_map.insert(1, "car".to_string());

    YoloProjectConfig {
        source_paths: SourcePaths {
            images: String::from("tests/sandbox/"),
            labels: String::from("tests/sandbox/"),
        },
        r#type: String::from("yolo"),
        project_name: String::from("test_project"),
        export: Export {
            paths: Paths {
                train: PathBuf::from("train/"),
                validation: PathBuf::from("validation/"),
                test: PathBuf::from("test/"),
                root: PathBuf::from("tests/sandbox/export/"),
            },
            class_map,
            duplicate_tolerance: 0.01,
            split: Split {
                train: 0.80,
                validation: 0.20,
                test: 0.0,
            },
        },
    }
}
