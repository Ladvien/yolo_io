mod common;

#[cfg(test)]
mod tests {

    use crate::common::{create_yolo_project_config, TEST_SANDBOX_DIR};
    use rstest::rstest;
    use yolo_io::{YoloProject, YoloProjectConfig, YoloProjectExporter};

    #[rstest]
    fn test_exporter(create_yolo_project_config: YoloProjectConfig) {
        assert!(YoloProjectExporter::export(YoloProject::new(&create_yolo_project_config)).is_ok());

        assert!(false)
    }
}
