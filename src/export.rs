use rand::seq::SliceRandom;
use rand::thread_rng;
use std::{collections::HashMap, fs};

use thiserror::Error;

use crate::{ImageLabelPair, YoloProject};

#[derive(Error, Debug)]
pub enum ExportError {
    #[error("Unable to create '{0}' directory")]
    UnableToCreateDirectory(String),
    #[error("Failed to unwrap label path.")]
    FailedToUnwrapLabelPath,
}

pub struct YoloProjectExporter {
    pub project: YoloProject,
}

impl YoloProjectExporter {
    pub fn export(project: YoloProject) -> Result<(), ExportError> {
        if fs::create_dir_all(project.config.export.paths.root.clone()).is_ok() {
            let train_path = format!(
                "{}/{}",
                project.config.export.paths.root.clone(),
                project.config.export.paths.train
            )
            .replace("//", "/");
            let validation_path = format!(
                "{}/{}",
                project.config.export.paths.root.clone(),
                project.config.export.paths.validation
            )
            .replace("//", "/");

            let test_path = format!(
                "{}/{}",
                project.config.export.paths.root.clone(),
                project.config.export.paths.test
            )
            .replace("//", "/");

            let paths_to_create = vec![
                train_path.clone(),
                validation_path.clone(),
                test_path.clone(),
            ];

            for path in paths_to_create {
                if fs::create_dir_all(path.clone()).is_err() {
                    return Err(ExportError::UnableToCreateDirectory(path));
                }
            }

            Self::create_yolo_yaml(
                &project.config.project_name,
                &project.config.export.paths.root,
                &train_path,
                &validation_path,
                &test_path,
                project.config.export.class_map.clone(),
            );

            let (train_pairs, validation_pairs, test_pairs) =
                Self::split_pairs(project.get_valid_pairs(), project.config.export.split.train);

            let splits: Vec<(String, Vec<ImageLabelPair>)> = vec![
                (test_path, test_pairs),
                (validation_path, validation_pairs),
                (train_path, train_pairs),
            ];

            for split in splits {
                Self::copy_files(&split.0, split.1)?
            }
        } else {
            return Err(ExportError::UnableToCreateDirectory(
                project.config.export.paths.root,
            ));
        }

        Ok(())
    }

    fn split_pairs(
        pairs: Vec<ImageLabelPair>,
        split: f32,
    ) -> (
        Vec<ImageLabelPair>,
        Vec<ImageLabelPair>,
        Vec<ImageLabelPair>,
    ) {
        let mut rng = thread_rng();
        let mut pairs = pairs;
        pairs.shuffle(&mut rng);

        let num_test_pairs = (split * pairs.len() as f32).round() as usize;

        let test_pairs = pairs.split_off(num_test_pairs);

        let num_val_pairs = (split * pairs.len() as f32).round() as usize;
        let validation_pairs = pairs.split_off(num_val_pairs);

        let train_pairs = pairs;

        (train_pairs, validation_pairs, test_pairs)
    }

    fn copy_files(export_path: &str, pairs: Vec<ImageLabelPair>) -> Result<(), ExportError> {
        for pair in pairs {
            let label_path = pair
                .label_path
                .ok_or(ExportError::FailedToUnwrapLabelPath)?;

            let label_path_str = label_path
                .to_str()
                .ok_or(ExportError::FailedToUnwrapLabelPath)?;

            fs::copy(label_path_str, format!("{}/{}.txt", export_path, pair.name)).ok();
        }

        Ok(())
    }

    fn create_yolo_yaml(
        project_name: &str,
        root_path: &str,
        train_path: &str,
        val_path: &str,
        test_path: &str,
        classes: HashMap<usize, String>,
    ) {
        let classes_as_yaml = classes
            .iter()
            .map(|(key, value)| format!("  {}: {}", key, value))
            .collect::<Vec<String>>()
            .join("\n");

        let yolo_yaml = format!(
            "# Generate by yolo_io - https://github.com/Ladvien/yolo_io
path: {root_path}
train: {train_path}
val: {val_path}
test: {test_path}

names:
{classes_as_yaml}
"
        );

        let yolo_yaml_path = format!("{root_path}/{project_name}.yaml");
        fs::write(yolo_yaml_path, yolo_yaml).unwrap();
    }
}
