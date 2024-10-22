mod yolo_file;

use itertools::{EitherOrBoth, Itertools};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::PathBuf};

pub use crate::yolo_file::*;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExportPaths {
    pub train: String,
    pub validation: String,
    pub test: String,
}

impl Default for ExportPaths {
    fn default() -> Self {
        Self {
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
    pub class_map: HashMap<u32, String>,
    pub duplicate_tolerance: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageLabelPair {
    pub name: String,
    pub image_path: Option<String>,
    pub label_path: Option<String>,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PairingResult {
    Valid(ImageLabelPair),
    Warning(ImageLabelPair),
    Error(Vec<ImageLabelPair>),
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
    pub pairs: HashMap<String, Vec<PairingResult>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YoloProject {
    pub data: YoloProjectData,
    pub config: YoloProjectConfig,
}

impl YoloProject {
    pub fn new(config: &YoloProjectConfig) -> Self {
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
                .values()
                .map(|name| YoloClass {
                    id: 0,
                    name: name.clone(),
                })
                .collect(),
            duplicate_tolerance: 0.0,
        };

        let pairs = Self::pair_images_and_labels(metadata, stems.clone(), label_paths, image_paths);

        Self {
            data: YoloProjectData { stems, pairs },
            config: config.clone(),
        }
    }

    fn validate_label_file() {}

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

    fn pair_images_and_labels(
        file_metadata: FileMetadata,
        stems: Vec<String>,
        label_filenames: Vec<PathWithKey>,
        image_filenames: Vec<PathWithKey>,
    ) -> HashMap<String, Vec<PairingResult>> {
        let mut pairing_map = HashMap::<String, Vec<PairingResult>>::new();

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

            // WILO: Working on filtering out invalid labels before pairing.
            let label_paths_for_stem = label_filenames
                .clone()
                .into_iter()
                .filter(|label| label.key == *stem)
                .map(|label| match label.clone().path.to_str() {
                    Some(path) => Ok(path.to_string()),
                    None => Err(()),
                })
                .collect::<Vec<Result<String, ()>>>();

            // TODO: Validate label files
            let mut label_files = label_paths_for_stem.clone();
            label_files.retain(|path| path.is_ok());

            let test = label_files
                .into_iter()
                .filter_map(|path| {
                    if let Ok(path) = path {
                        let yolo_file = YoloFile::new(&file_metadata, &path);
                        if let Ok(yolo_file) = yolo_file {
                            Some(yolo_file)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect::<Vec<YoloFile>>();

            println!("{:#?}", test);

            // TODO: Remove invalid label files from pairing.
            //       they go straight to errors.

            let unconfirmed_pairs = image_paths_for_stem
                .into_iter()
                .zip_longest(label_paths_for_stem.into_iter());

            pairing_map.insert(
                stem.clone(),
                unconfirmed_pairs
                    .into_iter()
                    .map(|pair| Self::evaluate_pair(stem.clone(), pair))
                    .collect::<Vec<PairingResult>>(),
            );
        }

        pairing_map
    }

    fn evaluate_pair(stem: String, pair: EitherOrBoth<Result<String, ()>>) -> PairingResult {
        match pair {
            EitherOrBoth::Both(image_path, label_path) => match (image_path, label_path) {
                (Ok(image_path), Ok(label_path)) => PairingResult::Valid(ImageLabelPair {
                    name: stem,
                    image_path: Some(image_path),
                    label_path: Some(label_path),
                    message: None,
                }),
                (Ok(image_path), Err(_)) => PairingResult::Warning(ImageLabelPair {
                    name: stem,
                    image_path: Some(image_path),
                    label_path: None,
                    message: Some("Label file is missing.".to_string()),
                }),
                (Err(_), Ok(label_path)) => PairingResult::Warning(ImageLabelPair {
                    name: stem,
                    image_path: None,
                    label_path: Some(label_path),
                    message: Some("Image file is missing.".to_string()),
                }),
                (Err(_), Err(_)) => PairingResult::Error(vec![ImageLabelPair {
                    name: stem,
                    image_path: None,
                    label_path: None,
                    message: Some("Both image and label files are missing.".to_string()),
                }]),
            },
            EitherOrBoth::Left(image_path) => PairingResult::Error(vec![ImageLabelPair {
                name: stem,
                image_path: Some(image_path.unwrap()),
                label_path: None,
                message: Some("Label file is missing.".to_string()),
            }]),
            EitherOrBoth::Right(label_path) => PairingResult::Error(vec![ImageLabelPair {
                name: stem,
                image_path: None,
                label_path: Some(label_path.unwrap()),
                message: Some("Image file is missing.".to_string()),
            }]),
        }
    }

    // pub fn validate_label_file(label_path: &String) -> bool {
    //     // TODO:
    // }

    pub fn validate(
        &self,
    ) -> Result<(Vec<ImageLabelPair>, Vec<ImageLabelPair>), Box<dyn std::error::Error>> {
        // 1. Check if file has a matching image.
        // 2. Check if the file is duplicated
        // 3. Check if file is empty
        // 4. Check if file meets YOLO formatting
        let mut valid_image_label_pairs = Vec::<ImageLabelPair>::new();
        let mut invalid_image_label_pairs = Vec::<ImageLabelPair>::new();

        let data_json = serde_json::to_string(&self.data).unwrap();
        fs::write("validation.json", data_json)?;

        // for (stem, results) in &self.pairs {
        //     for result in results {
        //         match result {
        //             PairingResult::Valid(image_label_pair) => todo!(),
        //             PairingResult::Warning(image_label_pair) => todo!(),
        //             PairingResult::Error(vec) => todo!(),
        //         }
        //     }
        // }

        Ok((valid_image_label_pairs, invalid_image_label_pairs))
    }

    pub fn get_valid_pairs(&self) -> Vec<ImageLabelPair> {
        let mut valid_pairs = Vec::<ImageLabelPair>::new();

        for pair in &self.data.pairs {
            for result in pair.1 {
                if let PairingResult::Valid(image_label_pair) = result {
                    valid_pairs.push(image_label_pair.clone());
                }
            }
        }

        valid_pairs
    }

    pub fn get_invalid_pairs(&self) -> Vec<ImageLabelPair> {
        let mut invalid_pairs = Vec::<ImageLabelPair>::new();

        for pair in &self.data.pairs {
            for result in pair.1 {
                if let PairingResult::Error(vec) = result {
                    invalid_pairs.extend(vec.clone());
                }
            }
        }

        invalid_pairs
    }
}
