#[macro_use]
mod report;
mod export;
mod yolo_file;

pub use export::*;
pub use report::YoloDataQualityReport;
pub use yolo_file::{YoloFile, YoloFileParseError, YoloFileParseErrorDetails};

use itertools::{EitherOrBoth, Itertools};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::PathBuf};
use thiserror::Error;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Split {
    pub train: f32,
    pub validation: f32,
    pub test: f32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Export {
    pub paths: Paths,
    pub class_map: HashMap<usize, String>,
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
    pub id: usize,
    pub name: String,
}

pub struct FileMetadata {
    pub classes: Vec<YoloClass>,
    pub duplicate_tolerance: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YoloProjectConfig {
    pub source_paths: SourcePaths,
    pub r#type: String,
    pub project_name: String,
    pub export: Export,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ImageLabelPair {
    pub name: String,
    pub image_path: Option<PathBuf>,
    pub label_path: Option<PathBuf>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct DuplicateImageLabelPair {
    pub name: String,
    pub primary: ImageLabelPair,
    pub duplicate: ImageLabelPair,
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
                write!(f, "Duplicate image and label files: {:#?}", duplicate)
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum PairingResult {
    Valid(ImageLabelPair),
    Invalid(PairingError),
}

#[derive(Debug, Clone)]
pub struct ValidationResults {
    pub valid_results: Vec<PairingResult>,
    pub invalid_results: Vec<PairingResult>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PathWithKey {
    pub path: PathBuf,
    pub key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YoloProjectData {
    pub stems: Vec<String>,
    pub pairs: Vec<PairingResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YoloProject {
    pub data: YoloProjectData,
    pub config: YoloProjectConfig,
}

impl YoloProject {
    pub fn new(config: &YoloProjectConfig) -> Self {
        println!("Creating new YoloProject from config: {:#?}", config);

        let image_paths = Self::get_filepaths_for_extension(
            &config.source_paths.images,
            vec!["jpg", "png", "PNG", "JPEG"],
        );

        let label_paths =
            Self::get_filepaths_for_extension(&config.source_paths.labels, vec!["txt"]);

        let all_filepaths = image_paths
            .iter()
            .chain(label_paths.iter())
            .collect::<Vec<&PathWithKey>>();

        let mut stems = Self::get_file_stems(&all_filepaths);

        // Remove duplicate stems; only works if sorted first.
        stems.sort();
        stems.dedup();

        let metadata = FileMetadata {
            classes: config
                .export
                .class_map
                .iter()
                .map(|(id, name)| YoloClass {
                    id: *id,
                    name: name.clone(),
                })
                .collect(),
            duplicate_tolerance: 0.0,
        };

        let pairs = Self::pair(metadata, stems.clone(), label_paths, image_paths);
        // let duplicates = gather_duplicates(stems.clone(), pairs.clone());

        Self {
            data: YoloProjectData { stems, pairs },
            config: config.clone(),
        }
    }

    pub fn get_valid_pairs(&self) -> Vec<ImageLabelPair> {
        // Return a Vec of _only_ Valid ImageLabelPairs
        self.data
            .pairs
            .iter()
            .filter_map(|pair| match pair {
                PairingResult::Valid(pair) => Some(pair.clone()),
                _ => None,
            })
            .collect::<Vec<ImageLabelPair>>()
    }

    pub fn get_invalid_pairs(&self) -> Vec<PairingError> {
        let invalid_pairs = self
            .data
            .pairs
            .iter()
            .filter_map(|pair| match pair {
                PairingResult::Invalid(error) => Some(error.clone()),
                _ => None,
            })
            .collect::<Vec<PairingError>>();

        invalid_pairs
    }

    fn get_filepaths_for_extension(path: &str, extensions: Vec<&str>) -> Vec<PathWithKey> {
        let file_paths = std::fs::read_dir(path);

        if file_paths.is_err() {
            return Vec::<PathWithKey>::new();
        }

        let mut paths = Vec::<PathWithKey>::new();

        for file_path in file_paths.unwrap() {
            let file_path = file_path.unwrap().path();

            if file_path.is_dir() {
                let filepaths = Self::get_filepaths_for_extension(
                    file_path.to_str().unwrap(),
                    extensions.clone(),
                );

                paths.extend(filepaths);
            }

            if let Some(file_extension) = file_path.extension() {
                let stem = file_path.file_stem().unwrap().to_str().unwrap();
                // TODO: Convert to return a PathWithKey
                let extension_str = file_extension.to_str().unwrap();

                if extensions.contains(&extension_str) {
                    paths.push(PathWithKey {
                        path: file_path.clone(),
                        key: String::from(stem),
                    });
                }
            }
        }

        paths
    }

    fn get_file_stems(filenames: &[&PathWithKey]) -> Vec<String> {
        filenames
            .iter()
            .map(|filename| filename.key.clone())
            .collect::<Vec<String>>()
    }

    fn pair(
        file_metadata: FileMetadata,
        stems: Vec<String>,
        label_filenames: Vec<PathWithKey>,
        image_filenames: Vec<PathWithKey>,
        // TODO: I should modify to collect pairs _and_ errors.
    ) -> Vec<PairingResult> {
        let mut pairs: Vec<PairingResult> = Vec::new();

        for stem in stems {
            let image_paths_for_stem = image_filenames
                .clone()
                .into_iter()
                .filter(|image| image.key == *stem)
                .map(|image| match image.clone().path.to_str() {
                    Some(path) => Ok(path.to_string()),
                    None => Err(()),
                })
                .collect::<Vec<Result<String, ()>>>();

            let label_paths_for_stem = label_filenames
                .clone()
                .into_iter()
                .filter(|label| label.key == *stem)
                .map(|label| match label.clone().path.to_str() {
                    Some(path) => Ok(path.to_string()),
                    None => Err(()),
                })
                .collect::<Vec<Result<String, ()>>>();

            let invalid_pairs =
                Self::process_label_path(&file_metadata, label_paths_for_stem.clone());

            // Remove invalid paths from label_paths_for_stem
            let label_paths_for_stem = label_paths_for_stem
                .into_iter()
                .filter(|path| path.is_ok())
                .collect::<Vec<Result<String, ()>>>();

            let unconfirmed_pairs = image_paths_for_stem
                .into_iter()
                .zip_longest(label_paths_for_stem.into_iter());

            let mut primary_pair: Option<ImageLabelPair> = None;

            for pair in unconfirmed_pairs {
                let result = Self::evaluate_pair(stem.clone(), pair.clone());

                match result {
                    PairingResult::Valid(pair) => match primary_pair {
                        Some(ref primary_pair) => {
                            pairs.push(PairingResult::Invalid(PairingError::Duplicate(
                                DuplicateImageLabelPair {
                                    name: stem.clone(),
                                    primary: primary_pair.clone(),
                                    duplicate: pair.clone(),
                                },
                            )));
                        }
                        None => {
                            primary_pair = Some(pair.clone());
                            pairs.push(PairingResult::Valid(pair));
                        }
                    },
                    PairingResult::Invalid(error) => {
                        pairs.push(PairingResult::Invalid(error));
                    }
                }
            }

            pairs.extend(invalid_pairs);
        }

        pairs
    }

    fn process_label_path(
        file_metadata: &FileMetadata,
        label_paths_for_stem: Vec<Result<String, ()>>,
    ) -> Vec<PairingResult> {
        let mut invalid_pairs = Vec::<PairingResult>::new();

        if label_paths_for_stem.is_empty() {
            invalid_pairs.push(PairingResult::Invalid(
                PairingError::LabelFileMissingUnableToUnwrapImagePath,
            ));
        } else {
            for label_path in &label_paths_for_stem {
                if let Ok(path) = label_path {
                    let yolo_file = YoloFile::new(file_metadata, path);
                    match yolo_file {
                        Ok(_) => {}
                        Err(error) => {
                            invalid_pairs
                                .push(PairingResult::Invalid(PairingError::LabelFileError(error)));
                        }
                    }
                } else {
                    invalid_pairs.push(PairingResult::Invalid(
                        PairingError::LabelFileMissingUnableToUnwrapImagePath,
                    ));
                }
            }
        }

        invalid_pairs
    }

    fn evaluate_pair(stem: String, pair: EitherOrBoth<Result<String, ()>>) -> PairingResult {
        match pair {
            EitherOrBoth::Both(image_path, label_path) => match (image_path, label_path) {
                (Ok(image_path), Ok(label_path)) => PairingResult::Valid(ImageLabelPair {
                    name: stem,
                    image_path: Some(PathBuf::from(image_path)),
                    label_path: Some(PathBuf::from(label_path)),
                }),
                (Ok(image_path), Err(_)) => {
                    PairingResult::Invalid(PairingError::LabelFileMissing(image_path))
                }
                (Err(_), Ok(label_path)) => {
                    PairingResult::Invalid(PairingError::ImageFileMissing(label_path))
                }
                (Err(_), Err(_)) => PairingResult::Invalid(PairingError::BothFilesMissing),
            },
            EitherOrBoth::Left(image_path) => match image_path {
                Ok(image_path) => {
                    PairingResult::Invalid(PairingError::LabelFileMissing(image_path))
                }
                Err(_) => {
                    PairingResult::Invalid(PairingError::LabelFileMissingUnableToUnwrapImagePath)
                }
            },
            EitherOrBoth::Right(label_path) => match label_path {
                Ok(label_path) => {
                    PairingResult::Invalid(PairingError::ImageFileMissing(label_path))
                }
                Err(_) => {
                    PairingResult::Invalid(PairingError::ImageFileMissingUnableToUnwrapLabelPath)
                }
            },
        }
    }
}
