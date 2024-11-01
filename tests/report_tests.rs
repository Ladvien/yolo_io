mod common;
#[cfg(test)]
mod report_tests {
    use std::path::PathBuf;

    use rstest::rstest;

    use crate::common::create_yolo_project_config;

    use yolo_io::{
        DataQualityItem, DuplicateImageLabelPair, ImageLabelPair, PairingError, PairingResult,
        YoloDataQualityReport, YoloFileParseError, YoloFileParseErrorDetails, YoloProject,
        YoloProjectData,
    };

    // #[rstest]
    // fn test_generate_report_with_label_file_error(
    //     mut create_yolo_project_config: yolo_io::YoloProjectConfig,
    // ) {
    //     let yolo_file_parse_error =
    //         YoloFileParseError::InvalidFormat(YoloFileParseErrorDetails::new(
    //             "label.txt".to_string(),
    //             1,
    //             "Invalid format".to_string(),
    //         ));
    //     let pairing_error = PairingError::LabelFileError(yolo_file_parse_error.clone());
    //     let project = create_test_project(vec![PairingResult::Invalid(pairing_error.clone())]);

    //     let report = YoloDataQualityReport::generate(project).unwrap();
    //     let expected = serde_json::to_string(&vec![DataQualityItem {
    //         source: "YoloFileParseError::InvalidFormat".to_string(),
    //         message: pairing_error.to_string(),
    //     }])
    //     .unwrap();

    //     assert_eq!(report, expected);
    // }

    // #[test]
    // fn test_generate_report_with_both_files_missing() {
    //     let pairing_error = PairingError::BothFilesMissing;
    //     let project = create_test_project(vec![PairingResult::Invalid(pairing_error.clone())]);

    //     let report = YoloDataQualityReport::generate(project).unwrap();
    //     let expected = serde_json::to_string(&vec![DataQualityItem {
    //         source: "BothFilesMissing".to_string(),
    //         message: pairing_error.to_string(),
    //     }])
    //     .unwrap();

    //     assert_eq!(report, expected);
    // }

    // #[test]
    // fn test_generate_report_with_label_file_missing() {
    //     let pairing_error = PairingError::LabelFileMissing("label.txt".to_string());
    //     let project = create_test_project(vec![PairingResult::Invalid(pairing_error.clone())]);

    //     let report = YoloDataQualityReport::generate(project).unwrap();
    //     let expected = serde_json::to_string(&vec![DataQualityItem {
    //         source: "LabelFileMissing".to_string(),
    //         message: pairing_error.to_string(),
    //     }])
    //     .unwrap();

    //     assert_eq!(report, expected);
    // }

    // #[test]
    // fn test_generate_report_with_image_file_missing() {
    //     let pairing_error = PairingError::ImageFileMissing("image.jpg".to_string());
    //     let project = create_test_project(vec![PairingResult::Invalid(pairing_error.clone())]);

    //     let report = YoloDataQualityReport::generate(project).unwrap();
    //     let expected = serde_json::to_string(&vec![DataQualityItem {
    //         source: "ImageFileMissing".to_string(),
    //         message: pairing_error.to_string(),
    //     }])
    //     .unwrap();

    //     assert_eq!(report, expected);
    // }

    // #[test]
    // fn test_generate_report_with_duplicate() {
    //     let primary_pair = ImageLabelPair {
    //         name: "test".to_string(),
    //         image_path: Some(PathBuf::from("image.jpg")),
    //         label_path: Some(PathBuf::from("label.txt")),
    //     };
    //     let duplicate_pair = ImageLabelPair {
    //         name: "test".to_string(),
    //         image_path: Some(PathBuf::from("image2.jpg")),
    //         label_path: Some(PathBuf::from("label2.txt")),
    //     };
    //     let pairing_error = PairingError::Duplicate(DuplicateImageLabelPair {
    //         name: "test".to_string(),
    //         primary: primary_pair.clone(),
    //         duplicate: duplicate_pair.clone(),
    //     });
    //     let project = create_test_project(vec![PairingResult::Invalid(pairing_error.clone())]);

    //     let report = YoloDataQualityReport::generate(project).unwrap();
    //     let expected = serde_json::to_string(&vec![DataQualityItem {
    //         source: "Duplicate".to_string(),
    //         message: pairing_error.to_string(),
    //     }])
    //     .unwrap();

    //     assert_eq!(report, expected);
    // }
}
