//! Minimal example that mirrors the Quick Start snippet in the README.

use yolo_io::{YoloProject, YoloProjectConfig, YoloProjectExporter};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = YoloProjectConfig::new("examples/config.yaml")?;
    let project = YoloProject::new(&config)?;
    YoloProjectExporter::export(project)?;
    Ok(())
}
