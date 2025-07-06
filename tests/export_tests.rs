mod common;

#[cfg(test)]
mod tests {

    use std::fs;

    use crate::common::{
        create_dir, create_dir_and_write_file, create_yolo_project_config, image_data,
        TEST_SANDBOX_DIR,
    };
    use image::{ImageBuffer, Rgb};
    use rstest::rstest;
    use yolo_io::{YoloProject, YoloProjectConfig, YoloProjectExporter};

    fn run_export(
        mut create_yolo_project_config: YoloProjectConfig,
        export_name: String,
        image_data: ImageBuffer<Rgb<u8>, Vec<u8>>,
    ) -> YoloProjectConfig {
        let export_source_dir = format!("{}/export_source_{}", TEST_SANDBOX_DIR, export_name);
        let export_out_dir = format!("{}/export_{}", TEST_SANDBOX_DIR, export_name);

        // Clean up old export directory
        let _ = fs::remove_dir_all(&export_source_dir);
        let _ = fs::remove_dir_all(&export_out_dir);

        create_dir(&export_source_dir);

        // Set split percentages
        create_yolo_project_config.export.split.train = 0.6;
        create_yolo_project_config.export.split.validation = 0.2;
        create_yolo_project_config.export.split.test = 0.2;

        create_yolo_project_config.source_paths.images = export_source_dir.clone();
        create_yolo_project_config.source_paths.labels = export_source_dir.clone();

        create_yolo_project_config.export.paths.root = export_out_dir.clone();

        let num_of_pairs = 10;
        for i in 0..num_of_pairs {
            let image_path = format!("{}/test_{}.jpg", export_source_dir, i);
            let label_path = format!("{}/test_{}.txt", export_source_dir, i);
            image_data.save(&image_path).expect("Unable to save image");
            create_dir_and_write_file(std::path::Path::new(&label_path), "0 0.5 0.5 0.5 0.5");
        }

        let project =
            YoloProject::new(&create_yolo_project_config).expect("Unable to create project");

        YoloProjectExporter::export(project).expect("Unable to export project");

        create_yolo_project_config
    }

    #[rstest]
    fn test_splits_correctly(create_yolo_project_config: YoloProjectConfig) {
        // Check train folder has 6 labels, 6 images
        let exported_config = run_export(
            create_yolo_project_config,
            "test_splits_correctly".to_string(),
            image_data(),
        );
        let train_image_path = format!("{}/train/images", exported_config.export.paths.root);

        let num_train_image_files = fs::read_dir(train_image_path)
            .expect("Unable to read train folder")
            .count();

        // Check validation folder has 2 label, 2 image
        let num_validation_image_files = fs::read_dir(format!(
            "{}/validation/images",
            exported_config.export.paths.root
        ))
        .unwrap()
        .count();

        // Check test folder has 2 label, 2 image
        let num_test_image_files =
            fs::read_dir(format!("{}/test/images", exported_config.export.paths.root))
                .unwrap()
                .count();

        assert_eq!(num_train_image_files, 6);
        assert_eq!(num_validation_image_files, 2);
        assert_eq!(num_test_image_files, 2);
    }

    #[rstest]
    fn test_yolo_yaml_created(create_yolo_project_config: YoloProjectConfig) {
        let exported_config = run_export(
            create_yolo_project_config,
            "test_yolo_yaml_created".to_string(),
            image_data(),
        );

        let yolo_yaml_path = format!("{}/test_project.yaml", exported_config.export.paths.root);

        let expected_yaml = r#"# Generate by yolo_io - https://github.com/Ladvien/yolo_io
path: tests/sandbox/export_test_yolo_yaml_created
train: train/
val: validation/
test: test/

names:
  0: person
  1: car
"#;

        let yolo_yaml = fs::read_to_string(yolo_yaml_path).expect("Unable to read yolo.yaml");

        assert_eq!(yolo_yaml, expected_yaml);
    }
}
