use hashbrown::HashMap;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use thiserror::Error;

use crate::{ExportError, YoloFile, YoloFileParseError};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Split {
    pub train: f32,
    pub validation: f32,
    pub test: f32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Export {
    pub paths: Paths,
    pub class_map: HashMap<isize, String>,
    pub duplicate_tolerance: f32,
    pub split: Split,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Paths {
    pub root: String,
    pub train: String,
    pub validation: String,
    pub test: String,
}

impl Paths {
    pub fn new(root: &str, train: &str, validation: &str, test: &str) -> Self {
        Paths {
            root: root.to_string(),
            train: train.to_string(),
            validation: validation.to_string(),
            test: test.to_string(),
        }
    }

    pub fn get_root(&self) -> String {
        self.root.clone()
    }

    pub fn get_train_images_path(&self) -> String {
        format!("{}/train/images", self.root).replace("//", "/")
    }

    pub fn get_train_label_images_path(&self) -> String {
        format!("{}/train/labels", self.root).replace("//", "/")
    }

    pub fn get_validation_images_path(&self) -> String {
        format!("{}/validation/images", self.root).replace("//", "/")
    }

    pub fn get_validation_label_images_path(&self) -> String {
        format!("{}/validation/labels", self.root).replace("//", "/")
    }

    pub fn get_test_images_path(&self) -> String {
        format!("{}/test/images", self.root).replace("//", "/")
    }

    pub fn get_test_label_images_path(&self) -> String {
        format!("{}/test/labels", self.root).replace("//", "/")
    }

    pub fn get_train_stem(&self) -> String {
        self.train.clone()
    }

    pub fn get_validation_stem(&self) -> String {
        self.validation.clone()
    }

    pub fn get_test_stem(&self) -> String {
        self.test.clone()
    }

    pub fn create_all_directories(&self) -> Result<(), ExportError> {
        let paths_to_create = vec![
            self.get_root(),
            self.get_train_images_path(),
            self.get_train_label_images_path(),
            self.get_validation_images_path(),
            self.get_validation_label_images_path(),
            self.get_test_images_path(),
            self.get_test_label_images_path(),
        ];

        for path in paths_to_create {
            if fs::create_dir_all(path.clone()).is_err() {
                return Err(ExportError::UnableToCreateDirectory(path));
            }
        }

        Ok(())
    }
}

impl Default for Paths {
    fn default() -> Self {
        Self {
            root: "export".to_string(),
            train: "train".to_string(),
            validation: "validation".to_string(),
            test: "test".to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SourcePaths {
    pub images: String,
    pub labels: String,
}

impl Default for SourcePaths {
    fn default() -> Self {
        Self {
            images: "images".to_string(),
            labels: "labels".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YoloClass {
    pub id: isize,
    pub name: String,
}

pub struct FileMetadata {
    pub classes: Vec<YoloClass>,
    pub duplicate_tolerance: f32,
}

/// Configuration for a YOLO project.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YoloProjectConfig {
    pub source_paths: SourcePaths,
    /// Identifies the project format. Currently only "yolo" is supported but
    /// this field is reserved for future project types.
    pub r#type: String,
    pub project_name: String,
    pub export: Export,
}

impl Default for YoloProjectConfig {
    fn default() -> Self {
        Self {
            source_paths: SourcePaths::default(),
            r#type: "yolo".to_string(),
            project_name: "default".to_string(),
            export: Export {
                paths: Paths::default(),
                class_map: HashMap::new(),
                duplicate_tolerance: 0.0,
                split: Split {
                    train: 0.7,
                    validation: 0.2,
                    test: 0.1,
                },
            },
        }
    }
}

impl YoloProjectConfig {
    pub fn new(path: &str) -> Result<Self, ExportError> {
        let data = fs::read_to_string(path).expect("Unable to read file");
        let config: YoloProjectConfig = serde_yml::from_str(&data).expect("Unable to parse YAML");
        Ok(config)
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ImageLabelPair {
    pub name: String,
    pub image_path: Option<PathBuf>,
    pub label_file: Option<YoloFile>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct DuplicateImageLabelPair {
    pub name: String,
    pub primary: ImageLabelPair,
    pub duplicate: ImageLabelPair,
}

impl std::fmt::Display for DuplicateImageLabelPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Duplicate image and label files for '{}'", self.name)
    }
}

#[derive(Error, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum PairingError {
    LabelFileError(YoloFileParseError),
    BothFilesMissing,
    LabelFileMissing(String),
    LabelFileMissingUnableToUnwrapImagePath,
    ImageFileMissing(String),
    ImageFileMissingUnableToUnwrapLabelPath,
    Duplicate(DuplicateImageLabelPair),
}

impl std::fmt::Display for PairingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PairingError::LabelFileError(error) => {
                write!(f, "Label file error: {}", error)
            }
            PairingError::BothFilesMissing => write!(f, "Both files missing"),
            PairingError::LabelFileMissing(path) => {
                write!(f, "Label file missing: {}", path)
            }
            PairingError::LabelFileMissingUnableToUnwrapImagePath => {
                write!(f, "Label file missing; unable to unwrap image path")
            }
            PairingError::ImageFileMissing(path) => {
                write!(f, "Image file missing: {}", path)
            }
            PairingError::ImageFileMissingUnableToUnwrapLabelPath => {
                write!(f, "Image file missing; unable to unwrap label path")
            }
            PairingError::Duplicate(duplicate) => {
                write!(f, "{}", duplicate)
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum PairingResult {
    Valid(ImageLabelPair),
    Invalid(PairingError),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PathWithKey {
    pub path: PathBuf,
    pub key: String,
}
