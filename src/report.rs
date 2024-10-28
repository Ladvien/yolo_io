use crate::{PairingError, YoloProject};
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

        for error in project.data.pairs.invalid.iter() {
            if let crate::PairingResult::Invalid(pairing_error) = error {
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
                crate::yolo_file::YoloFileParseError::InvalidFormat(_) => {
                    String::from("YoloFileParseError::InvalidFormat")
                }
                crate::yolo_file::YoloFileParseError::EmptyFile(_) => {
                    String::from("YoloFileParseError::EmptyFile")
                }
                crate::yolo_file::YoloFileParseError::DuplicateEntries(_) => {
                    String::from("YoloFileParseError::DuplicateEntries")
                }
                crate::yolo_file::YoloFileParseError::FailedToParseClassId(_) => {
                    String::from("YoloFileParseError::FailedToParseClassId")
                }
                crate::yolo_file::YoloFileParseError::ClassIdNotFound(_) => {
                    String::from("YoloFileParseError::ClassIdNotFound")
                }
                crate::yolo_file::YoloFileParseError::LabelDataOutOfRange(_) => {
                    String::from("YoloFileParseError::LabelDataOutOfRange")
                }
                crate::yolo_file::YoloFileParseError::FailedToParseColumn(_) => {
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
            PairingError::DuplicatedImageFile((pair, duplicate_pair)) => {
                format!(
                    "DuplicatedImageFile: {:?} and {:?}",
                    pair.image_path.to_owned(),
                    duplicate_pair.image_path.to_owned()
                )
            }
        }
    }
}
