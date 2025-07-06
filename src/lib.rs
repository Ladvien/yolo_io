//! IO helpers for working with YOLO style datasets.
//!
//! The crate scans directories for image and label files, pairs them,
//! validates the labels and can export the result into the YOLO directory
//! structure.  See [`YoloProject`] for the main entry point.
<<<<<<< HEAD
//!
//! # Example
//! ```no_run
//! use std::fs;
//! use yolo_io::{YoloProjectConfig, YoloProject, YoloDataQualityReport, YoloProjectExporter};
//!
//! let config = YoloProjectConfig::new("examples/config.yaml").unwrap();
//! let project = YoloProject::new(&config).unwrap();
//! if let Some(report) = YoloDataQualityReport::generate(project.clone()) {
//!     fs::write("report.json", report).unwrap();
//! }
//! YoloProjectExporter::export(project).unwrap();
//! ```
#[macro_use]
=======
>>>>>>> 4f3b3d75592e0b37becbaae01f804963cc209459
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
pub use yolo_file::{YoloEntry, YoloFile, YoloFileParseError, YoloFileParseErrorDetails};

use serde::{Deserialize, Serialize};

/// Data collected during project loading.

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Results of scanning the source directories.
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
/// Constructed from a [`YoloProjectConfig`], this struct contains the
/// scan results and configuration for a dataset. Use [`YoloProject::new`]
/// to load a project from disk and then inspect or export the validated
/// pairs.
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
    /// Load a project from disk using the provided configuration.
    ///
    /// The function scans the configured image and label directories,
    /// pairs files with the same stem and validates each pair.
    /// Returns an IO error if any of the directories cannot be read.
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
