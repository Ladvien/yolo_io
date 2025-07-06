mod common;
#[cfg(test)]
<<<<<<< HEAD
mod report_tests {
    use std::path::PathBuf;

    use rstest::rstest;

    use crate::common::create_yolo_project_config;

    use yolo_io::{
        DataQualityItem, DuplicateImageLabelPair, ImageLabelPair, PairingError, PairingResult,
        YoloDataQualityReport, YoloFileParseError, YoloFileParseErrorDetails, YoloProject,
        YoloProjectData,
    };

    fn create_test_project(pairs: Vec<PairingResult>) -> YoloProject {
        let config = create_yolo_project_config();
        let data = YoloProjectData {
            stems: vec![],
            pairs,
            number_of_classes: config.export.class_map.len(),
        };

        YoloProject { data, config }
    }

    #[rstest]
    fn test_generate_report_with_label_file_error(
        mut create_yolo_project_config: yolo_io::YoloProjectConfig,
    ) {
        let details = YoloFileParseErrorDetails {
            path: "label.txt".to_string(),
            class: None,
            row: Some(1),
            other_row: None,
            column: None,
            value: None,
        };
        let yolo_file_parse_error = YoloFileParseError::InvalidFormat(details);
        let pairing_error = PairingError::LabelFileError(yolo_file_parse_error.clone());
        let project = create_test_project(vec![PairingResult::Invalid(pairing_error.clone())]);

        let report = YoloDataQualityReport::generate(project).unwrap();
        let expected = serde_json::to_string(&vec![DataQualityItem {
            source: "YoloFileParseError::InvalidFormat".to_string(),
            message: pairing_error.to_string(),
            data: pairing_error.clone(),
        }])
        .unwrap();

        assert_eq!(report, expected);
    }

    #[test]
    fn test_generate_report_with_both_files_missing() {
        let pairing_error = PairingError::BothFilesMissing;
        let project = create_test_project(vec![PairingResult::Invalid(pairing_error.clone())]);

        let report = YoloDataQualityReport::generate(project).unwrap();
        let expected = serde_json::to_string(&vec![DataQualityItem {
            source: "BothFilesMissing".to_string(),
            message: pairing_error.to_string(),
            data: pairing_error.clone(),
        }])
        .unwrap();

        assert_eq!(report, expected);
    }

    #[test]
    fn test_generate_report_with_label_file_missing() {
        let pairing_error = PairingError::LabelFileMissing("label.txt".to_string());
        let project = create_test_project(vec![PairingResult::Invalid(pairing_error.clone())]);

        let report = YoloDataQualityReport::generate(project).unwrap();
        let expected = serde_json::to_string(&vec![DataQualityItem {
            source: "LabelFileMissing".to_string(),
            message: pairing_error.to_string(),
            data: pairing_error.clone(),
        }])
        .unwrap();

        assert_eq!(report, expected);
    }

    #[test]
    fn test_generate_report_with_image_file_missing() {
        let pairing_error = PairingError::ImageFileMissing("image.jpg".to_string());
        let project = create_test_project(vec![PairingResult::Invalid(pairing_error.clone())]);

        let report = YoloDataQualityReport::generate(project).unwrap();
        let expected = serde_json::to_string(&vec![DataQualityItem {
            source: "ImageFileMissing".to_string(),
            message: pairing_error.to_string(),
            data: pairing_error.clone(),
        }])
        .unwrap();

        assert_eq!(report, expected);
    }

    #[test]
    fn test_generate_report_with_duplicate() {
        let primary_pair = ImageLabelPair {
            name: "test".to_string(),
            image_path: Some(PathBuf::from("image.jpg")),
            label_file: None,
        };
        let duplicate_pair = ImageLabelPair {
            name: "test".to_string(),
            image_path: Some(PathBuf::from("image2.jpg")),
            label_file: None,
        };
        let pairing_error = PairingError::Duplicate(DuplicateImageLabelPair {
            name: "test".to_string(),
            primary: primary_pair.clone(),
            duplicate: duplicate_pair.clone(),
        });
        let project = create_test_project(vec![PairingResult::Invalid(pairing_error.clone())]);

        let report = YoloDataQualityReport::generate(project).unwrap();
        let expected = serde_json::to_string(&vec![DataQualityItem {
            source: "DuplicateImageLabelPair".to_string(),
            message: pairing_error.to_string(),
            data: pairing_error.clone(),
        }])
        .unwrap();

        assert_eq!(report, expected);
    }
}
=======
mod report_tests {}
>>>>>>> 88c6208b5242bb685205ed0cd2acd75901f72741
