# yolo_io

A friendly Rust crate for wrangling YOLO datasets. It loads projects, checks them for common issues, and spits out clean, well-structured datasets. Think of it as your personal assistant for YOLO files.

## Why yolo_io?

Working with image annotations can feel messy. Filenames don't match, labels go missing, and sometimes your dataset just refuses to cooperate. `yolo_io` aims to smooth those edges. It follows a straightforward, object‑oriented approach so you can reason about your project in clear steps:

1. **Configure** a `YoloProject` with a simple YAML file.
2. **Load** the project and let the library automatically pair images with labels.
3. **Validate** the pairs—empty files, corrupt formats, duplicate labels, and invalid class IDs are all flagged.
4. **Export** the cleaned dataset or generate a data quality report in JSON or YAML.

## Features at a Glance

- Automatic pairing based on filenames
- Detection of incomplete or conflicting pairs
- Validation checks for empty, malformed, or duplicate label files
- Built-in exporter that keeps your project structure intact
- Data quality reports with a single function call

## Quick Start

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
yolo_io = "0.1.103"
```

Load your project and export it back out:

```rust
use yolo_io::*;

let config = YoloProjectConfig::new("examples/config.yaml")?;
let project = YoloProject::new(&config)?;
YoloProjectExporter::export(project)?;
# Ok::<(), Box<dyn std::error::Error>>(())
```

Run the included example (requires the sample dataset in `examples/`):

```bash
cargo run --example basic
```

### Command-Line Reports

Generate a data quality report without writing a line of code:

```bash
cargo run --bin report -- --config examples/config.yaml --output report.json
```

Add `--format yaml` if you prefer YAML over JSON.

## Configuration

The project expects a YAML file that declares the dataset type. For now `"yolo"` is the only recognized type, but keeping the field allows future formats to slot right in.

## Additional Reading

If you're new to the YOLO format, see the [Ultralytics documentation](https://docs.ultralytics.com/yolov5/tutorials/train_custom_data/#21-create-datasetyaml) for a primer.

## Work in Progress

`yolo_io` is evolving. Features might change, and some corners are still rough. Feedback and contributions are welcome.

