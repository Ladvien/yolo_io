use crate::{
    types::{PairingError, PairingResult},
    YoloFileParseError, YoloProject,
};

use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct DataQualityItem {
    pub source: String,
    pub message: String,
}

pub struct YoloDataQualityReport;

impl YoloDataQualityReport {
    pub fn generate(project: YoloProject) -> Option<String> {
        let mut errors = Vec::<DataQualityItem>::new();

        for error in project.data.pairs.iter() {
            if let PairingResult::Invalid(pairing_error) = error {
                let dq_item = DataQualityItem {
                    source: Self::get_source_name(pairing_error),
                    message: pairing_error.to_string(),
                };

                errors.push(dq_item.clone());
            }
        }

        serde_json::to_string(&errors).ok()
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
            PairingError::Duplicate(_) => String::from("Duplicate"),
        }
    }
}
