use std::fs;
use yolo_io::*;

fn main() {
    // Load the project configuration from disk
    let config = YoloProjectConfig::new("examples/config.yaml").expect("Failed to load config");

    // Build a project using the configuration
    let project = YoloProject::new(&config).expect("Failed to create project");

    // Generate a data quality report
    if let Some(report) = YoloDataQualityReport::generate(project.clone()) {
        fs::write("report.json", report).expect("Unable to write report");
    }

    // Export the validated project to the configured paths
    YoloProjectExporter::export(project).expect("Failed to export project");
}
