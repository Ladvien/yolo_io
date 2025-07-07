mod common;

#[cfg(test)]
mod yolo_file_tests {
    use crate::common::TEST_SANDBOX_DIR;
    use yolo_io::{FileMetadata, YoloClass, YoloFile, YoloFileParseError};

    #[test]
    fn test_yolo_file_new_nonexistent_path_returns_error() {
        let classes = vec![YoloClass {
            id: 0,
            name: "person".to_string(),
        }];
        let metadata = FileMetadata {
            classes,
            duplicate_tolerance: 0.01,
        };
        let path = format!("{}/missing.txt", TEST_SANDBOX_DIR);

        let result = YoloFile::new(&metadata, &path);

        assert!(matches!(
            result,
            Err(YoloFileParseError::FailedToReadFile(_))
        ));
    }
}
