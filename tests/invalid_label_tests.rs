mod common;

#[cfg(test)]
mod invalid_label_tests {
    use rstest::rstest;
    use std::path::PathBuf;

    use crate::common::TEST_SANDBOX_DIR;
    use yolo_io::{
        FileMetadata, YoloClass, YoloFile, YoloFileParseError, YoloFileParseErrorDetails,
    };

    fn create_yolo_classes(classes: Vec<(usize, &str)>) -> Vec<YoloClass> {
        classes
            .iter()
            .map(|(id, name)| YoloClass {
                id: *id,
                name: name.to_string(),
            })
            .collect()
    }

    fn create_yolo_label_file(
        filename: &str,
        classes: Vec<YoloClass>,
        content: &str,
    ) -> (FileMetadata, String) {
        let path = format!("{}/data/{}", TEST_SANDBOX_DIR, filename);
        let mut file_content = String::new();

        file_content.push_str(content);

        std::fs::write(&path, file_content).unwrap();

        let metadata = FileMetadata {
            classes,
            duplicate_tolerance: 0.01,
        };

        (metadata, path)
    }

    #[test]
    fn test_yolo_file_new_parses_valid_file_correctly() {
        let filename = "valid1.txt";
        let classes_raw = vec![(0, "person"), (1, "car")];
        let classes = create_yolo_classes(classes_raw.clone());
        let (metadata, path) = create_yolo_label_file(
            "valid1.txt",
            classes.clone(),
            "0 0.25 0.5 0.25 0.5\n0 0.5 0.5 0.15 0.5\n1 0.5 0.5 0.5 0.35",
        );

        let expected_result: Result<YoloFile, YoloFileParseError> = YoloFile::new(&metadata, &path);

        let path = format!("{}/data/{}", TEST_SANDBOX_DIR, filename);

        let metadata = FileMetadata {
            classes,
            duplicate_tolerance: 0.01,
        };

        let yolo_file = YoloFile::new(&metadata, &path);

        assert_eq!(yolo_file, expected_result);
    }

    #[test]
    fn test_yolo_file_new_invalid_file_format_due_to_missing_column() {
        let filename = "invalid1.txt";
        let classes_raw = vec![(0, "person")];
        let classes = create_yolo_classes(classes_raw.clone());
        let (metadata, path) = create_yolo_label_file(filename, classes.clone(), "0 0.5 0.5");

        let yolo_file = YoloFile::new(&metadata, &path);

        assert!(yolo_file.is_err());
    }

    #[test]
    fn test_yolo_file_new_invalid_file_format_due_unparsable_class_id() {
        let filename = "invalid2.txt";
        let classes_raw = vec![(0, "person")];
        let classes = create_yolo_classes(classes_raw.clone());
        let (metadata, path) =
            create_yolo_label_file(filename, classes.clone(), "a 0.5 0.5 0.5 0.5");

        let yolo_file = YoloFile::new(&metadata, &path);

        if let Err(err) = yolo_file {
            assert_eq!(
                err.to_string(),
                "Unable to parse value 'a' in file 'tests/sandbox/data/invalid2.txt' on line 0"
            );
        } else {
            panic!("Expected error");
        }
    }

    #[test]
    pub fn test_yolo_file_new_invalid_file_format_due_to_invalid_class_id() {
        let filename = "invalid3.txt";
        let classes_raw = vec![(0, "person")];
        let classes = create_yolo_classes(classes_raw.clone());
        let (metadata, path) =
            create_yolo_label_file(filename, classes.clone(), "2 0.5 0.5 0.5 0.5");

        let yolo_file = YoloFile::new(&metadata, &path);

        if let Err(err) = yolo_file {
            assert_eq!(
                err.to_string(),
                "Invalid class id '2' in file 'tests/sandbox/data/invalid3.txt'"
            );
        } else {
            panic!("Expected error");
        }
    }

    #[test]
    pub fn test_yolo_file_new_invalid_file_format_due_class_id_not_found() {
        let filename = "invalid4.txt";
        let classes_raw = vec![(0, "person")];
        let classes = create_yolo_classes(classes_raw.clone());
        let (metadata, path) =
            create_yolo_label_file(filename, classes.clone(), "1 0.5 0.5 0.5 0.5");

        let yolo_file = YoloFile::new(&metadata, &path);

        if let Err(err) = yolo_file {
            assert_eq!(
                err.to_string(),
                "Invalid class id '1' in file 'tests/sandbox/data/invalid4.txt'"
            );
        } else {
            panic!("Expected error");
        }
    }

    #[test]
    pub fn test_yolo_file_new_invalid_column_data_in_x_center() {
        let filename = "invalid5.txt";
        let classes_raw = vec![(0, "person")];
        let classes = create_yolo_classes(classes_raw.clone());
        let (metadata, path) = create_yolo_label_file(filename, classes.clone(), "0 a 0.5 0.5 0.5");

        let yolo_file = YoloFile::new(&metadata, &path);

        if let Err(err) = yolo_file {
            assert_eq!(
                err.to_string(),
                "Failed to parse 'x' column with value of '0' on line 0 in file 'tests/sandbox/data/invalid5.txt'"
            );
        } else {
            panic!("Expected error");
        }
    }

    #[test]
    pub fn test_yolo_file_new_invalid_column_data_in_y_center() {
        let filename = "invalid6.txt";
        let classes_raw = vec![(0, "person")];
        let classes = create_yolo_classes(classes_raw.clone());
        let (metadata, path) = create_yolo_label_file(filename, classes.clone(), "0 0.5 a 0.5 0.5");

        let yolo_file = YoloFile::new(&metadata, &path);

        if let Err(err) = yolo_file {
            assert_eq!(
                err.to_string(),
                "Failed to parse 'y' column with value of '0' on line 0 in file 'tests/sandbox/data/invalid6.txt'"
            );
        } else {
            panic!("Expected error");
        }
    }

    #[test]
    pub fn test_yolo_file_new_invalid_column_data_in_width() {
        let filename = "invalid7.txt";
        let classes_raw = vec![(0, "person")];
        let classes = create_yolo_classes(classes_raw.clone());
        let (metadata, path) = create_yolo_label_file(filename, classes.clone(), "0 0.5 0.5 a 0.5");

        let yolo_file = YoloFile::new(&metadata, &path);

        if let Err(err) = yolo_file {
            assert_eq!(
                err.to_string(),
                "Failed to parse 'w' column with value of '0' on line 0 in file 'tests/sandbox/data/invalid7.txt'"
            );
        } else {
            panic!("Expected error");
        }
    }

    #[test]
    pub fn test_yolo_file_new_invalid_column_data_in_height() {
        let filename = "invalid8.txt";
        let classes_raw = vec![(0, "person")];
        let classes = create_yolo_classes(classes_raw.clone());
        let (metadata, path) = create_yolo_label_file(filename, classes.clone(), "0 0.5 0.5 0.5 a");
        let yolo_file = YoloFile::new(&metadata, &path);

        if let Err(err) = yolo_file {
            assert_eq!(
                err.to_string(),
                "Failed to parse 'h' column with value of '0' on line 0 in file 'tests/sandbox/data/invalid8.txt'"
            );
        } else {
            panic!("Expected error");
        }
    }

    #[test]
    pub fn test_yolo_file_new_x_column_contains_value_out_of_range() {
        let filename = "invalid9.txt";
        let classes_raw = vec![(0, "person")];
        let classes = create_yolo_classes(classes_raw.clone());
        let (metadata, path) =
            create_yolo_label_file(filename, classes.clone(), "0 1.5 0.5 0.5 0.5");

        let yolo_file = YoloFile::new(&metadata, &path);

        if let Err(err) = yolo_file {
            assert_eq!(
                err.to_string(),
                "Invalid data value for 'x' in file 'tests/sandbox/data/invalid9.txt' on line 0.  Value is '1.5'"
            );
        } else {
            panic!("Expected error");
        }
    }

    #[test]
    pub fn test_yolo_file_new_y_column_contains_value_out_of_range() {
        let filename = "invalid10.txt";
        let classes_raw = vec![(0, "person")];
        let classes = create_yolo_classes(classes_raw.clone());
        let (metadata, path) =
            create_yolo_label_file(filename, classes.clone(), "0 0.5 1.5 0.5 0.5");

        let yolo_file = YoloFile::new(&metadata, &path);

        if let Err(err) = yolo_file {
            assert_eq!(
                err.to_string(),
                "Invalid data value for 'y' in file 'tests/sandbox/data/invalid10.txt' on line 0.  Value is '1.5'"
            );
        } else {
            panic!("Expected error");
        }
    }

    #[test]
    pub fn test_yolo_file_new_width_column_contains_value_out_of_range() {
        let filename = "invalid11.txt";
        let classes_raw = vec![(0, "person")];
        let classes = create_yolo_classes(classes_raw.clone());
        let (metadata, path) =
            create_yolo_label_file(filename, classes.clone(), "0 0.5 0.5 1.5 0.5");

        let yolo_file = YoloFile::new(&metadata, &path);

        if let Err(err) = yolo_file {
            assert_eq!(
                err.to_string(),
                "Invalid data value for 'w' in file 'tests/sandbox/data/invalid11.txt' on line 0.  Value is '1.5'"
            );
        } else {
            panic!("Expected error");
        }
    }

    #[test]
    pub fn test_yolo_file_new_height_column_contains_value_out_of_range() {
        let filename = "invalid12.txt";
        let classes_raw = vec![(0, "person")];
        let classes = create_yolo_classes(classes_raw.clone());
        let (metadata, path) =
            create_yolo_label_file(filename, classes.clone(), "0 0.5 0.5 0.5 1.5");

        let yolo_file = YoloFile::new(&metadata, &path);

        if let Err(err) = yolo_file {
            assert_eq!(
                err.to_string(),
                "Invalid data value for 'h' in file 'tests/sandbox/data/invalid12.txt' on line 0.  Value is '1.5'"
            );
        } else {
            panic!("Expected error");
        }
    }

    #[test]
    pub fn test_yolo_file_new_invalid_file_format_due_to_empty_file() {
        let filename = "invalid13.txt";
        let classes_raw = vec![(0, "person")];
        let classes = create_yolo_classes(classes_raw.clone());
        let (metadata, path) = create_yolo_label_file(filename, classes.clone(), "");

        let yolo_file = YoloFile::new(&metadata, &path);

        if let Err(err) = yolo_file {
            assert_eq!(
                err.to_string(),
                "File 'tests/sandbox/data/invalid13.txt' is empty"
            );
        } else {
            panic!("Expected error");
        }
    }

    #[rstest]
    fn test_yolo_file_new_contains_duplicate_labels() {
        let filename = "invalid14.txt";
        let classes_raw = vec![(0, "person"), (1, "car")];
        let classes = create_yolo_classes(classes_raw.clone());
        let (metadata, path) = create_yolo_label_file(
            filename,
            classes.clone(),
            "0 0.25 0.5 0.25 0.5\n0 0.25 0.5 0.25 0.5",
        );

        let yolo_file = YoloFile::new(&metadata, &path);

        if let Err(err) = yolo_file {
            assert_eq!(
                err.to_string(),
                "Duplicate entries found in file 'tests/sandbox/data/invalid14.txt' on row 0 and row 1"
            );
        } else {
            panic!("Expected error");
        }
    }
}
