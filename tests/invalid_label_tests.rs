mod common;

#[cfg(test)]
mod tests {
    use std::{fs, path::PathBuf};

    use yolo_io::{YoloClass, YoloFile};

    use crate::common::{create_yolo_label_file, TEST_SANDBOX_DIR};

    #[test]
    fn test_yolo_file_new_parses_valid_file_correctly() {
        let path = format!("{}/data/valid1.txt", TEST_SANDBOX_DIR);
        let content = format!(
            "{}\n{}\n{}\n",
            "0 0.5 0.5 0.5 0.5", "0 0.5 0.5 0.5 0.5", "1 0.5 0.5 0.5 0.5"
        );
        let content = content.as_ref();
        create_yolo_label_file(&PathBuf::from(&path), content);

        let classes = vec![
            YoloClass {
                id: 0,
                name: "person".to_string(),
            },
            YoloClass {
                id: 1,
                name: "car".to_string(),
            },
        ];

        let yolo_file = YoloFile::new(classes, &path);

        assert!(yolo_file.is_ok());
    }

    #[test]
    fn test_yolo_file_new_invalid_file_format_due_to_missing_column() {
        let path = format!("{}/data/invalid1.txt", TEST_SANDBOX_DIR);
        let content = r#"0 0.5 0.5 0.5"#;
        create_yolo_label_file(&PathBuf::from(&path), content);

        let classes = vec![YoloClass {
            id: 0,
            name: "person".to_string(),
        }];

        let yolo_file = YoloFile::new(classes, &path);
        assert!(yolo_file.is_err());
    }

    #[test]
    fn test_yolo_file_new_invalid_file_format_due_unparseable_class_id() {
        let path = format!("{}/data/invalid2.txt", TEST_SANDBOX_DIR);
        let content = r#"a 0.5 0.5 0.5 0.5"#;
        create_yolo_label_file(&PathBuf::from(&path), content);

        let classes = vec![YoloClass {
            id: 0,
            name: "person".to_string(),
        }];

        let yolo_file = YoloFile::new(classes, &path);

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
        let path = format!("{}/data/invalid3.txt", TEST_SANDBOX_DIR);
        let content = r#"2 0.5 0.5 0.5 0.5"#;
        create_yolo_label_file(&PathBuf::from(&path), content);

        let classes = vec![YoloClass {
            id: 0,
            name: "person".to_string(),
        }];

        let yolo_file = YoloFile::new(classes, &path);

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
        let path = format!("{}/data/invalid4.txt", TEST_SANDBOX_DIR);
        let content = r#"1 0.5 0.5 0.5 0.5"#;
        create_yolo_label_file(&PathBuf::from(&path), content);

        let classes = vec![YoloClass {
            id: 0,
            name: "person".to_string(),
        }];

        let yolo_file = YoloFile::new(classes, &path);

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
        let path = format!("{}/data/invalid5.txt", TEST_SANDBOX_DIR);
        let content = r#"0 a 0.5 0.5 0.5"#;
        create_yolo_label_file(&PathBuf::from(&path), content);

        let classes = vec![YoloClass {
            id: 0,
            name: "person".to_string(),
        }];

        let yolo_file = YoloFile::new(classes, &path);

        if let Err(err) = yolo_file {
            assert_eq!(
                err.to_string(),
                "Failed to parse 'x' on line 0 in file 'tests/sandbox/data/invalid5.txt'"
            );
        } else {
            panic!("Expected error");
        }
    }

    #[test]
    pub fn test_yolo_file_new_invalid_column_data_in_y_center() {
        let path = format!("{}/data/invalid6.txt", TEST_SANDBOX_DIR);
        let content = r#"0 0.5 a 0.5 0.5"#;
        create_yolo_label_file(&PathBuf::from(&path), content);

        let classes = vec![YoloClass {
            id: 0,
            name: "person".to_string(),
        }];

        let yolo_file = YoloFile::new(classes, &path);

        if let Err(err) = yolo_file {
            assert_eq!(
                err.to_string(),
                "Failed to parse 'y' on line 0 in file 'tests/sandbox/data/invalid6.txt'"
            );
        } else {
            panic!("Expected error");
        }
    }

    #[test]
    pub fn test_yolo_file_new_invalid_column_data_in_width() {
        let path = format!("{}/data/invalid7.txt", TEST_SANDBOX_DIR);
        let content = r#"0 0.5 0.5 a 0.5"#;
        create_yolo_label_file(&PathBuf::from(&path), content);

        let classes = vec![YoloClass {
            id: 0,
            name: "person".to_string(),
        }];

        let yolo_file = YoloFile::new(classes, &path);

        if let Err(err) = yolo_file {
            assert_eq!(
                err.to_string(),
                "Failed to parse 'w' on line 0 in file 'tests/sandbox/data/invalid7.txt'"
            );
        } else {
            panic!("Expected error");
        }
    }

    #[test]
    pub fn test_yolo_file_new_invalid_column_data_in_height() {
        let path = format!("{}/data/invalid8.txt", TEST_SANDBOX_DIR);
        let content = r#"0 0.5 0.5 0.5 a"#;
        create_yolo_label_file(&PathBuf::from(&path), content);

        let classes = vec![YoloClass {
            id: 0,
            name: "person".to_string(),
        }];

        let yolo_file = YoloFile::new(classes, &path);

        if let Err(err) = yolo_file {
            assert_eq!(
                err.to_string(),
                "Failed to parse 'h' on line 0 in file 'tests/sandbox/data/invalid8.txt'"
            );
        } else {
            panic!("Expected error");
        }
    }

    #[test]
    pub fn test_yolo_file_new_x_column_contains_value_out_of_range() {
        let path = format!("{}/data/invalid9.txt", TEST_SANDBOX_DIR);
        let content = r#"0 1.5 0.5 0.5 0.5"#;
        create_yolo_label_file(&PathBuf::from(&path), content);

        let classes = vec![YoloClass {
            id: 0,
            name: "person".to_string(),
        }];

        let yolo_file = YoloFile::new(classes, &path);

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
        let path = format!("{}/data/invalid10.txt", TEST_SANDBOX_DIR);
        let content = r#"0 0.5 1.5 0.5 0.5"#;
        create_yolo_label_file(&PathBuf::from(&path), content);

        let classes = vec![YoloClass {
            id: 0,
            name: "person".to_string(),
        }];

        let yolo_file = YoloFile::new(classes, &path);

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
        let path = format!("{}/data/invalid11.txt", TEST_SANDBOX_DIR);
        let content = r#"0 0.5 0.5 1.5 0.5"#;
        create_yolo_label_file(&PathBuf::from(&path), content);

        let classes = vec![YoloClass {
            id: 0,
            name: "person".to_string(),
        }];

        let yolo_file = YoloFile::new(classes, &path);

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
        let path = format!("{}/data/invalid12.txt", TEST_SANDBOX_DIR);
        let content = r#"0 0.5 0.5 0.5 1.5"#;
        create_yolo_label_file(&PathBuf::from(&path), content);

        let classes = vec![YoloClass {
            id: 0,
            name: "person".to_string(),
        }];

        let yolo_file = YoloFile::new(classes, &path);

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
        let path = format!("{}/data/invalid13.txt", TEST_SANDBOX_DIR);
        let content = "";
        create_yolo_label_file(&PathBuf::from(&path), content);

        let classes = vec![YoloClass {
            id: 0,
            name: "person".to_string(),
        }];

        let yolo_file = YoloFile::new(classes, &path);

        if let Err(err) = yolo_file {
            assert_eq!(
                err.to_string(),
                "File 'tests/sandbox/data/invalid13.txt' is empty"
            );
        } else {
            panic!("Expected error");
        }
    }
}
