use hashbrown::HashMap;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use thiserror::Error;

use crate::{ExportError, YoloFile, YoloFileParseError};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
/// Percentage split used when exporting.
pub struct Split {
    /// Portion of data to use for training.
    pub train: f32,
    /// Portion of data to use for validation.
    pub validation: f32,
    /// Portion of data to use for testing.
    pub test: f32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
/// Settings controlling dataset export.
pub struct Export {
    /// Directory layout for the exported dataset.
    pub paths: Paths,
    /// Mapping of class id to class name.
    pub class_map: HashMap<isize, String>,
    /// Bounding box overlap tolerance used for duplicate detection.
    pub duplicate_tolerance: f32,
    /// Train/val/test ratio.
    pub split: Split,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
/// Collection of paths used during export.
pub struct Paths {
    /// Root directory for exported data.
    pub root: PathBuf,
    /// Sub directory used for training data.
    pub train: PathBuf,
    /// Sub directory used for validation data.
    pub validation: PathBuf,
    /// Sub directory used for test data.
    pub test: PathBuf,
}

impl Paths {
    /// Create a new set of export paths.
    pub fn new(
        root: impl Into<PathBuf>,
        train: impl Into<PathBuf>,
        validation: impl Into<PathBuf>,
        test: impl Into<PathBuf>,
    ) -> Self {
        Paths {
            root: root.into(),
            train: train.into(),
            validation: validation.into(),
            test: test.into(),
        }
    }

    /// Root path used for export.
    pub fn get_root(&self) -> PathBuf {
        self.root.clone()
    }

    /// Path to the training images directory.
    pub fn get_train_images_path(&self) -> PathBuf {
        self.root.join(&self.train).join("images")
    }

    /// Path to the training labels directory.
    pub fn get_train_label_images_path(&self) -> PathBuf {
        self.root.join(&self.train).join("labels")
    }

    /// Path to the validation images directory.
    pub fn get_validation_images_path(&self) -> PathBuf {
        self.root.join(&self.validation).join("images")
    }

    /// Path to the validation labels directory.
    pub fn get_validation_label_images_path(&self) -> PathBuf {
        self.root.join(&self.validation).join("labels")
    }

    /// Path to the test images directory.
    pub fn get_test_images_path(&self) -> PathBuf {
        self.root.join(&self.test).join("images")
    }

    /// Path to the test labels directory.
    pub fn get_test_label_images_path(&self) -> PathBuf {
        self.root.join(&self.test).join("labels")
    }

    /// Directory stem used for training data.
    pub fn get_train_stem(&self) -> String {
        self.train.to_string_lossy().into_owned()
    }

    /// Directory stem used for validation data.
    pub fn get_validation_stem(&self) -> String {
        self.validation.to_string_lossy().into_owned()
    }

    /// Directory stem used for test data.
    pub fn get_test_stem(&self) -> String {
        self.test.to_string_lossy().into_owned()
    }

    /// Create the directory structure on disk.
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
            if fs::create_dir_all(&path).is_err() {
                return Err(ExportError::UnableToCreateDirectory(
                    path.to_string_lossy().into_owned(),
                ));
            }
        }

        Ok(())
    }
}

impl Default for Paths {
    fn default() -> Self {
        Self {
            root: PathBuf::from("export"),
            train: PathBuf::from("train"),
            validation: PathBuf::from("validation"),
            test: PathBuf::from("test"),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
/// Where to locate source images and labels when loading a project.
pub struct SourcePaths {
    /// Directory containing image files.
    pub images: String,
    /// Directory containing label files.
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
/// Class id and name as defined by the project configuration.
pub struct YoloClass {
    /// Numeric class identifier.
    pub id: isize,
    /// Human readable class name.
    pub name: String,
}

/// Parameters used when validating label files.
pub struct FileMetadata {
    /// Allowed classes for labels.
    pub classes: Vec<YoloClass>,
    /// Tolerance for bounding box duplication.
    pub duplicate_tolerance: f32,
}

/// Configuration for a YOLO project.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Top level configuration for a [`crate::YoloProject`].
pub struct YoloProjectConfig {
    /// Location of images and labels to scan.
    pub source_paths: SourcePaths,
    /// Type of project, currently always "yolo".
    pub r#type: String,
    /// Name of the project.
    pub project_name: String,
    /// Export related settings.
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
    /// Read a YAML configuration from disk.
    pub fn new(path: impl AsRef<std::path::Path>) -> Result<Self, ExportError> {
        let data = fs::read_to_string(path.as_ref())
            .map_err(|e| ExportError::ReadConfig(e.to_string()))?;
        let config =
            serde_yml::from_str(&data).map_err(|e| ExportError::ParseConfig(e.to_string()))?;
        Ok(config)
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
/// An image and label pair discovered in the project.
pub struct ImageLabelPair {
    /// File stem shared by the image and label.
    pub name: String,
    /// Path to the image file if it exists.
    pub image_path: Option<PathBuf>,
    /// Parsed label file if it exists.
    pub label_file: Option<YoloFile>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
/// Used when multiple files with the same stem are found.
pub struct DuplicateImageLabelPair {
    /// The shared file stem.
    pub name: String,
    /// First discovered pair for the stem.
    pub primary: ImageLabelPair,
    /// Additional pair detected as a duplicate.
    pub duplicate: ImageLabelPair,
}

impl std::fmt::Display for DuplicateImageLabelPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Duplicate image and label files for '{}'", self.name)
    }
}

#[derive(Error, Clone, PartialEq, Debug, Serialize, Deserialize)]
/// Reasons why a stem could not be paired.
pub enum PairingError {
    LabelFileError(YoloFileParseError),
    BothFilesMissing,
    LabelFileMissing(String),
    LabelFileMissingUnableToUnwrapImagePath,
    ImageFileMissing(String),
    ImageFileMissingUnableToUnwrapLabelPath,
    Duplicate(DuplicateImageLabelPair),
    DuplicateLabelMismatch(DuplicateImageLabelPair),
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
            PairingError::DuplicateLabelMismatch(_) => {
                write!(f, "Duplicate image with differing label files")
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
/// Result of attempting to pair an image and label file.
pub enum PairingResult {
    Valid(ImageLabelPair),
    Invalid(PairingError),
}

#[derive(Debug, Clone, Eq, PartialEq)]
/// Helper used when scanning for files.
pub struct PathWithKey {
    /// Full path to the file.
    pub path: PathBuf,
    /// Stem of the file used for matching.
    pub key: String,
}
