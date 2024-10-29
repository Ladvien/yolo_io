mod common;

#[cfg(test)]
mod tests {

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

        create_dir(&export_source_dir);
        // println!("{:?}", export_source_dir);

        create_yolo_project_config.source_paths.images = export_source_dir.clone();
        create_yolo_project_config.source_paths.labels = export_source_dir.clone();

        create_yolo_project_config.export.paths.root = format!("{}/export", TEST_SANDBOX_DIR);

        let num_of_pairs = 10;
        for i in 0..num_of_pairs {
            let image_path = format!("{}/test_{}.jpg", export_source_dir, i);
            let label_path = format!("{}/test_{}.txt", export_source_dir, i);
            image_data.save(&image_path).expect("Unable to save image");
            create_dir_and_write_file(std::path::Path::new(&label_path), "0 0.5 0.5 0.5 0.5");
        }

        let project = YoloProject::new(&create_yolo_project_config);
        // println!("{:#?}", project.get_invalid_pairs());

        YoloProjectExporter::export(project).expect("Unable to export project");

        assert!(false)
    }
}
