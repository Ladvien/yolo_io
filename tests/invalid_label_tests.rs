#[cfg(test)]
mod tests {
    use std::{
        fs,
        path::{Path, PathBuf},
    };

    use image::{ImageBuffer, Rgb};
    use rstest::{fixture, rstest};
    use yolo_io::{YoloProject, YoloProjectConfig};

    const TEST_SANDBOX_DIR: &str = "tests/sandbox/";

    #[fixture]
    fn image_data() -> ImageBuffer<Rgb<u8>, Vec<u8>> {
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

    fn create_dir_and_write_file(path: &Path, content: &str) {
        fs::create_dir_all(path.parent().unwrap()).expect("Unable to create directory");
        fs::write(path, content).expect("Unable to write file");
    }

    fn create_image_file(path: &Path, image_data: &ImageBuffer<Rgb<u8>, Vec<u8>>) {
        fs::create_dir_all(path.parent().unwrap()).expect("Unable to create directory");
        image_data.save(path).expect("Unable to write file");
    }

    /*
    Test Scenarios
        Type
        Error = E
        Warn  = W
        Valid = V
        Mixed = M

                 | 1 Label | No Label | Label >2
        1 Image  |  V      |   E      |  M
        No Image |  E      |   -      |  M
        Image >2 |  M      |   E      |  V
     */
    #[rstest]
    fn test_project_validation_flags_empty_label_file() {}
}
