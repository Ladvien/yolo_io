//! IO helpers for working with YOLO style datasets.
//!
//! The crate scans directories for image and label files, pairs them,
//! validates the labels and can export the result into the YOLO directory
//! structure. See [`YoloProject`] for the main entry point.
//!
//! # Example
//!
//! ```rust,no_run
//! use yolo_io::{YoloProjectConfig, YoloProject, YoloProjectExporter};
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let config = YoloProjectConfig::new("examples/config.yaml")?;
//!     let project = YoloProject::new(&config)?;
//!     YoloProjectExporter::export(project)?;
//!     Ok(())
//! }
//! ```
mod export;
mod file_utils;
mod pairing;
mod report;
mod types;
mod yolo_file;

pub use export::*;
use file_utils::get_filepaths_for_extension;
use file_utils::FileError;
use pairing::pair;
pub use report::generate_yaml;
pub use report::DataQualityItem;
pub use report::YoloDataQualityReport;
pub use types::{
    DuplicateImageLabelPair, Export, FileMetadata, ImageLabelPair, PairingError, PairingResult,
    PathWithKey, Paths, SourcePaths, Split, YoloClass, YoloProjectConfig,
};
pub use yolo_file::{YoloEntry, YoloFile, YoloFileParseError, YoloFileParseErrorDetails};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Results of scanning the source directories when a project is loaded.
///
/// This structure stores every file stem discovered along with the
/// outcome of pairing the corresponding image and label files. It also
/// records how many unique classes were found in the project
/// configuration.
pub struct YoloProjectData {
    /// File stems found in the source directories.
    pub stems: Vec<String>,
    /// Pairing and validation results for each stem.
    pub pairs: Vec<PairingResult>,
    /// Number of classes defined in the project configuration.
    pub number_of_classes: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// High level representation of a YOLO dataset project.
///
/// A `YoloProject` combines the user supplied configuration with the
/// data discovered during scanning. It exposes helper methods to query
/// valid pairs or inspect any errors detected while validating the
/// dataset.
pub struct YoloProject {
    /// Data produced when loading the project.
    pub data: YoloProjectData,
    /// Configuration used when loading and exporting the project.
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
    /// Load a project using a [`YoloProjectConfig`].
    ///
    /// The method scans the configured image and label directories,
    /// pairs files with the same stem and validates each pair. The
    /// results of this process are stored within the returned
    /// [`YoloProject`] for further inspection or export.
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
            duplicate_tolerance: config.export.duplicate_tolerance,
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

    /// Retrieve all successfully paired image/label combinations.
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

    /// Return any errors encountered during pairing or validation.
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

    /// Lookup a pair by the file stem.
    pub fn get_pair(&self, stem: &str) -> Option<ImageLabelPair> {
        self.get_valid_pairs()
            .iter()
            .find(|pair| pair.name == stem)
            .cloned()
    }

    /// Access a valid pair by index.
    ///
    /// Returns `None` if the index is out of bounds.
    pub fn pair_at_index(&self, index: usize) -> Option<ImageLabelPair> {
        self.get_valid_pairs().get(index).cloned()
    }

    fn get_file_stems(filenames: &[&PathWithKey]) -> Vec<String> {
        filenames
            .iter()
            .map(|filename| filename.key.clone())
            .collect::<Vec<String>>()
    }
}
