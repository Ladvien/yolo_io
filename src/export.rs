use hashbrown::HashMap;
use log::debug;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::{
    fs,
    path::{Path, PathBuf},
};

use thiserror::Error;

use crate::{
    types::{ImageLabelPair, Paths, Split},
    YoloProject,
};

/// Errors that may occur while exporting a project.

#[derive(Error, Debug)]
pub enum ExportError {
    #[error("Unable to create '{0}' directory")]
    UnableToCreateDirectory(String),
    #[error("Failed to unwrap label path.")]
    FailedToUnwrapLabelPath,
    #[error("Failed to copy file '{0}' to '{1}'.")]
    FailedToCopyFile(String, String),
    #[error("Failed to read config: {0}")]
    ReadConfig(String),
    #[error("Failed to parse config: {0}")]
    ParseConfig(String),
    #[error("Failed to write file '{0}'")]
    WriteFile(String),
}

/// Handles writing a [`YoloProject`] to disk.
///
/// The exporter creates the directory layout, generates the YAML
/// configuration and copies all files into the appropriate train,
/// validation and test folders.
pub struct YoloProjectExporter {
    /// Project to be exported.
    pub project: YoloProject,
}

impl YoloProjectExporter {
    /// Write the given [`YoloProject`] to disk according to its configuration.
    ///
    /// The project is split into training, validation and test sets
    /// based on the configured ratio. A `data.yaml` file is produced
    /// alongside the copied images and labels so the dataset can be
    /// consumed directly by YOLO tooling.
    pub fn export(project: YoloProject) -> Result<(), ExportError> {
        let paths = &project.config.export.paths;

        paths.create_all_directories()?;

        let project_name = &project.config.project_name;
        let classes = &project.config.export.class_map;

        Self::create_yolo_yaml(project_name, paths, classes)?;

        let (train_pairs, validation_pairs, test_pairs) =
            Self::split_pairs(project.get_valid_pairs(), project.config.export.split);

        let test_image_path = paths.get_test_images_path();
        let test_label_path = paths.get_test_label_images_path();

        let train_image_path = paths.get_train_images_path();
        let train_label_path = paths.get_train_label_images_path();

        let val_image_path = paths.get_validation_images_path();
        let val_label_path = paths.get_validation_label_images_path();

        let splits: Vec<(PathBuf, PathBuf, Vec<ImageLabelPair>)> = vec![
            (test_image_path, test_label_path, test_pairs),
            (train_image_path, train_label_path, train_pairs),
            (val_image_path, val_label_path, validation_pairs),
        ];

        for (images_path, labels_path, pairs) in splits {
            Self::copy_files(&images_path, &labels_path, pairs)?;
        }

        Ok(())
    }

    fn split_pairs(
        pairs: Vec<ImageLabelPair>,
        split: Split,
    ) -> (
        Vec<ImageLabelPair>,
        Vec<ImageLabelPair>,
        Vec<ImageLabelPair>,
    ) {
        let mut rng = thread_rng();
        let mut pairs = pairs;
        pairs.shuffle(&mut rng);

        let total = pairs.len();

        let num_test_pairs = (split.test * total as f32).round() as usize;
        let num_val_pairs = (split.validation * total as f32).round() as usize;

        let test_pairs: Vec<ImageLabelPair> =
            pairs.drain(0..num_test_pairs.min(pairs.len())).collect();
        let validation_pairs: Vec<ImageLabelPair> =
            pairs.drain(0..num_val_pairs.min(pairs.len())).collect();
        let train_pairs = pairs;

        (train_pairs, validation_pairs, test_pairs)
    }

    fn copy_files(
        export_images_path: &Path,
        export_labels_path: &Path,
        pairs: Vec<ImageLabelPair>,
    ) -> Result<(), ExportError> {
        for pair in pairs {
            debug!("pair: {:?}", pair);

            let image_path = pair
                .image_path
                .ok_or(ExportError::FailedToUnwrapLabelPath)?;

            let label_file = pair
                .label_file
                .ok_or(ExportError::FailedToUnwrapLabelPath)?;

            let label_path = PathBuf::from(label_file.path);

            let image_ext = image_path
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("");

            let label_ext = label_path
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("");

            let new_image_path =
                export_images_path.join(PathBuf::from(&pair.name).with_extension(image_ext));

            let new_label_path =
                export_labels_path.join(PathBuf::from(&pair.name).with_extension(label_ext));

            fs::copy(&image_path, &new_image_path).map_err(|_| {
                ExportError::FailedToCopyFile(
                    image_path.to_string_lossy().to_string(),
                    new_image_path.to_string_lossy().to_string(),
                )
            })?;

            fs::copy(&label_path, &new_label_path).map_err(|_| {
                ExportError::FailedToCopyFile(
                    label_path.to_string_lossy().to_string(),
                    new_label_path.to_string_lossy().to_string(),
                )
            })?;
        }

        Ok(())
    }

    fn create_yolo_yaml(
        project_name: &str,
        paths: &Paths,
        classes: &HashMap<isize, String>,
    ) -> Result<(), ExportError> {
        let mut classes_vec: Vec<(isize, String)> =
            classes.iter().map(|(&k, v)| (k, v.clone())).collect();

        classes_vec.sort_by(|a, b| a.0.cmp(&b.0));

        let classes_as_yaml = classes_vec
            .iter()
            .map(|(key, value)| format!("  {}: {}", key, value))
            .collect::<Vec<String>>()
            .join("\n");

        let root_path = paths.get_root();
        let train_path = paths.get_train_stem();
        let val_path = paths.get_validation_stem();
        let test_path = paths.get_test_stem();

        let yolo_yaml = format!(
            "# Generate by yolo_io - https://github.com/Ladvien/yolo_io
path: {}
train: {}
val: {}
test: {}

names:
{}
",
            root_path.to_string_lossy(),
            train_path,
            val_path,
            test_path,
            classes_as_yaml
        );

        let yolo_yaml_path = root_path.join(format!("{project_name}.yaml"));
        fs::write(&yolo_yaml_path, yolo_yaml)
            .map_err(|_| ExportError::WriteFile(yolo_yaml_path.to_string_lossy().into()))?;

        Ok(())
    }
}
