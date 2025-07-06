use crate::{
    types::{PairingError, PairingResult},
    YoloFileParseError, YoloProject,
};

use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
/// Entry describing a single data quality issue.
pub struct DataQualityItem {
    /// Source of the issue (error type).
    pub source: String,
    /// Human readable error message.
    pub message: String,
    /// Structured data backing the error.
    pub data: crate::PairingError,
}

/// Utility for turning pairing results into JSON reports.
pub struct YoloDataQualityReport;

impl YoloDataQualityReport {
    /// Create a JSON report from a [`YoloProject`].
    pub fn generate(project: YoloProject) -> Option<String> {
        let mut errors = Vec::<DataQualityItem>::new();

        for error in project.data.pairs.iter() {
            if let PairingResult::Invalid(pairing_error) = error {
                let dq_item = DataQualityItem {
                    source: Self::get_source_name(pairing_error),
                    message: pairing_error.to_string(),
                    data: pairing_error.clone(),
                };

                errors.push(dq_item.clone());
            }
        }

        if errors.is_empty() {
            None
        } else {
            serde_json::to_string(&errors).ok()
        }
    }

    fn get_source_name(pairing_error: &PairingError) -> String {
        match pairing_error {
            PairingError::LabelFileError(yolo_file_parse_error) => match yolo_file_parse_error {
                YoloFileParseError::InvalidFormat(_) => {
                    String::from("YoloFileParseError::InvalidFormat")
                }
                YoloFileParseError::EmptyFile(_) => String::from("YoloFileParseError::EmptyFile"),
                YoloFileParseError::DuplicateEntries(_) => {
                    String::from("YoloFileParseError::DuplicateEntries")
                }
                YoloFileParseError::FailedToParseClassId(_) => {
                    String::from("YoloFileParseError::FailedToParseClassId")
                }
                YoloFileParseError::ClassIdNotFound(_) => {
                    String::from("YoloFileParseError::ClassIdNotFound")
                }
                YoloFileParseError::LabelDataOutOfRange(_) => {
                    String::from("YoloFileParseError::LabelDataOutOfRange")
                }
                YoloFileParseError::FailedToParseColumn(_) => {
                    String::from("YoloFileParseError::FailedToParseColumn")
                }
                YoloFileParseError::FailedToGetFileStem(_) => {
                    String::from("YoloFileParseError::FailedToGetFileStem")
                }
            },
            PairingError::BothFilesMissing => String::from("BothFilesMissing"),
            PairingError::LabelFileMissing(_) => String::from("LabelFileMissing"),
            PairingError::LabelFileMissingUnableToUnwrapImagePath => {
                String::from("LabelFileMissingUnableToUnwrapImagePath")
            }
            PairingError::ImageFileMissing(_) => String::from("ImageFileMissing"),
            PairingError::ImageFileMissingUnableToUnwrapLabelPath => {
                String::from("ImageFileMissingUnableToUnwrapLabelPath")
            }
            PairingError::Duplicate(_) => String::from("DuplicateImageLabelPair"),
            PairingError::DuplicateLabelMismatch(_) => String::from("DuplicateImageLabelMismatch"),
            PairingError::BothFilesMissing => String::from("BothFilesMissing"),
        }
    }
}
