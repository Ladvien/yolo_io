#[cfg(test)]
mod config_tests {
    use yolo_io::{ExportError, YoloProjectConfig};

    #[test]
    fn test_invalid_config_path_returns_error() {
        let result = YoloProjectConfig::new("tests/does_not_exist.yaml");
        assert!(matches!(result, Err(ExportError::ReadConfig(_))));
    }
}
