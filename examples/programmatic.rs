//! Example showing how to build a `YoloProjectConfig` directly in code.
//! The example prepares minimal sample data, constructs the configuration
//! programmatically, generates a data-quality report and finally exports
//! the dataset.

use hashbrown::HashMap;
use image::{ImageBuffer, Rgb};
use std::fs;
use std::path::Path;
use yolo_io::YoloDataQualityReport;
use yolo_io::{
    Export, Paths, SourcePaths, Split, YoloProject, YoloProjectConfig, YoloProjectExporter,
};

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
    // Prepare the example dataset so the code runs standalone.
    ensure_sample_data();

    // Manually construct the class map used by the project.
    let mut class_map = HashMap::new();
    class_map.insert(0, "person".to_string());
    class_map.insert(1, "car".to_string());

    // Define where images and labels reside and where exports should be written.
    let export_paths = Paths::new("examples/export", "train", "validation", "test");
    let source_paths = SourcePaths {
        images: "examples/images".to_string(),
        labels: "examples/labels".to_string(),
    };

    // Assemble the project configuration from the pieces above.
    let config = YoloProjectConfig {
        source_paths,
        r#type: "yolo".to_string(),
        project_name: "demo_project".to_string(),
        export: Export {
            paths: export_paths,
            class_map,
            duplicate_tolerance: 0.0,
            split: Split {
                train: 0.8,
                validation: 0.2,
                test: 0.0,
            },
        },
    };

    // Build the project using the configuration built above.
    let project = YoloProject::new(&config).expect("Failed to create project");

    // Generate a JSON data-quality report if any issues are found.
    if let Some(report) = YoloDataQualityReport::generate(project.clone()) {
        fs::write("report.json", &report).expect("Unable to write report");
    }

    // Export the dataset to the configured paths.
    YoloProjectExporter::export(project).expect("Failed to export project");
}
