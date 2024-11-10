#[macro_use]
mod report;
mod export;
mod file_utils;
mod pairing;
mod types;
mod yolo_file;

pub use export::*;
use file_utils::get_filepaths_for_extension;
use file_utils::FileError;
use pairing::pair;
pub use report::DataQualityItem;
pub use report::YoloDataQualityReport;
pub use types::{
    DuplicateImageLabelPair, Export, FileMetadata, ImageLabelPair, PairingError, PairingResult,
    PathWithKey, Paths, SourcePaths, Split, YoloClass, YoloProjectConfig,
};
pub use yolo_file::{YoloFile, YoloFileParseError, YoloFileParseErrorDetails};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YoloProjectData {
    pub stems: Vec<String>,
    pub pairs: Vec<PairingResult>,
    pub number_of_classes: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YoloProject {
    pub data: YoloProjectData,
    pub config: YoloProjectConfig,
}

impl Default for YoloProject {
    fn default() -> Self {
        Self {
            data: YoloProjectData {
                stems: vec![],
                pairs: vec![],
                number_of_classes: 0,
            },
            config: Default::default(),
        }
    }
}

impl YoloProject {
    pub fn new(config: &YoloProjectConfig) -> Result<Self, FileError> {
        let image_paths = get_filepaths_for_extension(
            &config.source_paths.images,
            vec!["jpg", "png", "PNG", "JPEG"],
        )?;

        let label_paths = get_filepaths_for_extension(&config.source_paths.labels, vec!["txt"])?;

        let all_filepaths = image_paths
            .iter()
            .chain(label_paths.iter())
            .collect::<Vec<&PathWithKey>>();

        let mut stems = Self::get_file_stems(&all_filepaths);

        // Remove duplicate stems; only works if sorted first.
        stems.sort();
        stems.dedup();

        let classes = config
            .export
            .class_map
            .iter()
            .map(|(id, name)| YoloClass {
                id: *id,
                name: name.clone(),
            })
            .collect::<Vec<YoloClass>>();

        let metadata = FileMetadata {
            classes: classes.clone(),
            duplicate_tolerance: 0.0,
        };

        let pairs = pair(metadata, stems.clone(), label_paths, image_paths);

        Ok(Self {
            data: YoloProjectData {
                stems,
                pairs,
                number_of_classes: classes.len(),
            },
            config: config.clone(),
        })
    }

    pub fn get_valid_pairs(&self) -> Vec<ImageLabelPair> {
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

    pub fn get_pair(&self, stem: &str) -> Option<ImageLabelPair> {
        self.get_valid_pairs()
            .iter()
            .find(|pair| pair.name == stem)
            .cloned()
    }

    pub fn pair_at_index(&self, index: isize) -> Option<ImageLabelPair> {
        self.get_valid_pairs().get(index as usize).cloned()
    }

    fn get_file_stems(filenames: &[&PathWithKey]) -> Vec<String> {
        filenames
            .iter()
            .map(|filename| filename.key.clone())
            .collect::<Vec<String>>()
    }
}
