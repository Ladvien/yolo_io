use hashbrown::HashMap;
use yolo_io::*;

fn main() {
    let mut class_map = HashMap::new();
    class_map.insert(0, "sample".to_string());

    let config = YoloProjectConfig {
        source_paths: SourcePaths {
            images: "examples/images".to_string(),
            labels: "examples/labels".to_string(),
        },
        r#type: "yolo".to_string(),
        project_name: "programmatic".to_string(),
        export: Export {
            paths: Paths {
                train: "train".into(),
                validation: "validation".into(),
                test: "test".into(),
                root: "examples/export".into(),
            },
            class_map,
            duplicate_tolerance: 0.01,
            split: Split {
                train: 1.0,
                validation: 0.0,
                test: 0.0,
            },
        },
    };

    let project = YoloProject::new(&config).expect("Failed to create project");
    YoloProjectExporter::export(project).expect("Failed to export project");
}
