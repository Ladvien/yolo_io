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

            let mut valid_pairs = project.get_valid_pairs();
            let train_pairs = valid_pairs.split_off(5);

            Self::copy_files(&train_path, train_pairs);
        } else {
            return Err(ExportError::UnableToCreateDirectory(
                project.config.export.paths.root,
            ));
        }

        Ok(())
    }

    fn copy_files(export_path: &str, pairs: Vec<ImageLabelPair>) -> Result<(), ExportError> {
        println!("Copying files to: {}", export_path);
        println!("Copying {} files", pairs.len());

        for pair in pairs {
            // WILO: Get back to work copying the files properly.

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
