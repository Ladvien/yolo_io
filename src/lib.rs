#[macro_use]
mod report;
mod export;
mod yolo_file;

pub use export::*;
pub use report::YoloDataQualityReport;
pub use yolo_file::{YoloFile, YoloFileParseError, YoloFileParseErrorDetails};

use itertools::{EitherOrBoth, Itertools};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};
use thiserror::Error;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExportPaths {
    pub root: String,
    pub train: String,
    pub validation: String,
    pub test: String,
}

impl Default for ExportPaths {
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
    pub export_paths: ExportPaths,
    pub class_map: HashMap<usize, String>,
    pub duplicate_tolerance: f32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ImageLabelPair {
    pub name: String,
    pub image_path: Option<PathBuf>,
    pub label_path: Option<PathBuf>,
}

#[derive(Error, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum PairingError {
    LabelFileError(YoloFileParseError),
    BothFilesMissing,
    LabelFileMissing(String),
    LabelFileMissingUnableToUnwrapImagePath,
    ImageFileMissing(String),
    ImageFileMissingUnableToUnwrapLabelPath,
    DuplicatedImageFile((ImageLabelPair, ImageLabelPair)),
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
            PairingError::DuplicatedImageFile((image_label_pair, duplicate_image_label_pair)) => {
                write!(
                    f,
                    "Duplicated image file: {:?} and {:?}",
                    image_label_pair, duplicate_image_label_pair
                )
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
        println!("Getting invalid pairs: {:#?}", self.data.pairs);

        let invalid_pairs = self
            .data
            .pairs
            .iter()
            .filter_map(|pair| match pair {
                PairingResult::Invalid(error) => {
                    println!("Invalid pair found: {:#?}", error);
                    Some(error.clone())
                }
                _ => {
                    println!("Valid pair found in invalid pairs: {:#?}", pair);
                    None
                }
            })
            .collect::<Vec<PairingError>>();

        println!("Invalid pairs: {:#?}", invalid_pairs);

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
        let mut invalid_pairs: Vec<PairingResult> = Vec::new();

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

            Self::process_label_path(
                &file_metadata,
                label_paths_for_stem.clone(),
                &mut invalid_pairs,
            );

            let unconfirmed_pairs = image_paths_for_stem
                .into_iter()
                .zip_longest(label_paths_for_stem.into_iter());

            let unduplicated_pairs = unconfirmed_pairs
                .into_iter()
                .map(|pair| Self::evaluate_pair(stem.clone(), pair))
                .collect::<Vec<PairingResult>>();

            Self::check_for_duplicates(&mut pairs, &mut invalid_pairs);

            pairs.extend(unduplicated_pairs);
        }

        pairs.extend(invalid_pairs);

        pairs
    }

    fn process_label_path(
        file_metadata: &FileMetadata,
        label_paths_for_stem: Vec<Result<String, ()>>,
        invalid_pairs: &mut Vec<PairingResult>,
    ) {
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

    fn check_for_duplicates(
        valid_pairs: &mut Vec<PairingResult>,
        invalid_pairs: &mut Vec<PairingResult>,
    ) {
        for pair in valid_pairs.clone() {
            let mut other_pairs = valid_pairs.clone();
            other_pairs.retain(|other_pair| other_pair != &pair);

            for other_pair in other_pairs {
                if let (PairingResult::Valid(pair), PairingResult::Valid(other_pair)) =
                    (pair.clone(), other_pair.clone())
                {
                    match (pair.name.clone(), other_pair.name.clone()) {
                        (image_name, other_image_name) => {
                            if image_name == other_image_name {
                                valid_pairs.push(PairingResult::Valid(pair));
                                invalid_pairs.push(PairingResult::Valid(other_pair));
                            }
                        }
                        _ => {}
                    }
                }
            }

            valid_pairs.retain(|pair| match pair {
                PairingResult::Valid(pair) => !pair.name.is_empty(),
                _ => true,
            });
        }
    }
}
