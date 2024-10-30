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

    #[rstest]
    fn test_exporter(
        mut create_yolo_project_config: YoloProjectConfig,
        image_data: ImageBuffer<Rgb<u8>, Vec<u8>>,
    ) {
        let export_source_dir = format!("{}/export_source", TEST_SANDBOX_DIR);
        let export_out_dir = format!("{}/export", TEST_SANDBOX_DIR);

        create_dir(&export_source_dir);

        // Clean up old export directory
        let _ = fs::remove_dir_all(&export_out_dir);

        create_yolo_project_config.source_paths.images = export_source_dir.clone();
        create_yolo_project_config.source_paths.labels = export_source_dir.clone();

        // Set split percentages
        create_yolo_project_config.export.split.train = 0.6;
        create_yolo_project_config.export.split.validation = 0.2;
        create_yolo_project_config.export.split.test = 0.2;

        create_yolo_project_config.export.paths.root = format!("{}/export", TEST_SANDBOX_DIR);

        let num_of_pairs = 10;
        for i in 0..num_of_pairs {
            let image_path = format!("{}/test_{}.jpg", export_source_dir, i);
            let label_path = format!("{}/test_{}.txt", export_source_dir, i);
            image_data.save(&image_path).expect("Unable to save image");
            create_dir_and_write_file(std::path::Path::new(&label_path), "0 0.5 0.5 0.5 0.5");
        }

        let project = YoloProject::new(&create_yolo_project_config);

        YoloProjectExporter::export(project).expect("Unable to export project");

        // Check train folder has 6 labels, 6 images
        let num_train_files = fs::read_dir(format!(
            "{}/train",
            create_yolo_project_config.export.paths.root
        ))
        .expect("Unable to read train folder")
        .count();

        // Check validation folder has 2 label, 2 image
        let num_validation_files = fs::read_dir(format!(
            "{}/validation",
            create_yolo_project_config.export.paths.root
        ))
        .expect("Unable to read validation folder")
        .count();

        // Check test folder has 2 label, 2 image
        let num_test_files = fs::read_dir(format!(
            "{}/test",
            create_yolo_project_config.export.paths.root
        ));

        assert_eq!(num_train_files, 6);
        assert_eq!(num_validation_files, 2);
        assert_eq!(num_test_files.unwrap().count(), 2);
    }
}
