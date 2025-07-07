//! Basic example demonstrating loading a project configuration, validating the
//! dataset and exporting it. A label file is provided in
//! `examples/labels`. The example will create a sample image under
//! `examples/images` on the fly if one does not already exist. These paths are
//! referenced in `examples/config.yaml` so the example can be run without any
//! additional setup.

use image::{ImageBuffer, Rgb};
use std::fs;
use std::path::Path;
use yolo_io::*;

fn ensure_sample_data() {
    let img_path = Path::new("examples/images/sample.jpg");
    if !img_path.exists() {
        if let Some(parent) = img_path.parent() {
            fs::create_dir_all(parent).expect("Unable to create image directory");
        }
        let width = 16;
        let height = 16;
        let mut imgbuf = ImageBuffer::new(width, height);
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let r = ((x + y) as u8) * 8;
            *pixel = Rgb([r, 0, 255 - r]);
        }
        imgbuf.save(img_path).expect("Unable to write sample image");
    }
    let label_path = Path::new("examples/labels/sample.txt");
    if !label_path.exists() {
        if let Some(parent) = label_path.parent() {
            fs::create_dir_all(parent).expect("Unable to create label directory");
        }
        fs::write(label_path, "0 0.5 0.5 0.5 0.5").expect("Unable to write sample label");
    }
}

fn main() {
    // Prepare minimal example data
    ensure_sample_data();

    // Load the project configuration from disk
    let config = YoloProjectConfig::new(std::path::Path::new("examples/config.yaml"))
        .expect("Failed to load config");

    // Build a project using the configuration
    let project = YoloProject::new(&config).expect("Failed to create project");

    // Generate data quality reports
    if let Some(report) = YoloDataQualityReport::generate(project.clone()) {
        fs::write("report.json", &report).expect("Unable to write report");
    }

    if let Some(report) = YoloDataQualityReport::generate_yaml(project.clone()) {
        fs::write("report.yml", &report).expect("Unable to write report");
    }

    // Export the validated project to the configured paths
    YoloProjectExporter::export(project).expect("Failed to export project");
}
