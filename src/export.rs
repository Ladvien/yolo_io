use hashbrown::HashMap;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fs;

use thiserror::Error;

use crate::{
    types::{ImageLabelPair, Paths, Split},
    YoloProject,
};

#[derive(Error, Debug)]
pub enum ExportError {
    #[error("Unable to create '{0}' directory")]
    UnableToCreateDirectory(String),
    #[error("Failed to unwrap label path.")]
    FailedToUnwrapLabelPath,
    #[error("Failed to copy file '{0}' to '{0}'.")]
    FailedToCopyFile(String, String),
}

pub struct YoloProjectExporter {
    pub project: YoloProject,
}

impl YoloProjectExporter {
    pub fn export(project: YoloProject) -> Result<(), ExportError> {
        let paths = &project.config.export.paths;

        paths.create_all_directories()?;

        let project_name = &project.config.project_name;
        let classes = &project.config.export.class_map;

        Self::create_yolo_yaml(project_name, paths, classes);

        let (train_pairs, validation_pairs, test_pairs) =
            Self::split_pairs(project.get_valid_pairs(), project.config.export.split);

        let test_image_path = &paths.get_test_images_path();
        let test_label_path = &paths.get_test_label_images_path();

        let train_image_path = &paths.get_train_images_path();
        let train_label_path = &paths.get_train_label_images_path();

        let val_image_path = &paths.get_validation_images_path();
        let val_label_path = &paths.get_validation_label_images_path();

        let splits: Vec<(&str, &str, Vec<ImageLabelPair>)> = vec![
            (test_image_path, test_label_path, test_pairs),
            (train_image_path, train_label_path, train_pairs),
            (val_image_path, val_label_path, validation_pairs),
        ];

        for (images_path, labels_path, pairs) in splits {
            Self::copy_files(images_path, labels_path, pairs)?;
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

        let num_test_pairs = ((1.0 - split.test) * pairs.len() as f32).round() as usize;
        let test_pairs = pairs.split_off(num_test_pairs);

        let num_val_pairs = ((1.0 - split.validation) * pairs.len() as f32).round() as usize;
        let validation_pairs = pairs.split_off(num_val_pairs);

        let train_pairs = pairs;

        (train_pairs, validation_pairs, test_pairs)
    }

    fn copy_files(
        export_images_path: &str,
        export_labels_path: &str,
        pairs: Vec<ImageLabelPair>,
    ) -> Result<(), ExportError> {
        for pair in pairs {
            println!("pair: {:?}", pair);

            let image_path = pair
                .image_path
                .ok_or(ExportError::FailedToUnwrapLabelPath)?;

            let image_path = image_path
                .as_os_str()
                .to_str()
                .ok_or(ExportError::FailedToUnwrapLabelPath)?;

            let label_file = pair
                .label_file
                .ok_or(ExportError::FailedToUnwrapLabelPath)?;

            let label_path = label_file.path;

            let image_stem = pair.name.clone();
            let label_stem = pair.name;

            let new_image_path = format!("{}/{}", export_images_path, image_stem);
            let new_label_path = format!("{}/{}", export_labels_path, label_stem);

            fs::copy(image_path, new_image_path.clone()).map_err(|_| {
                ExportError::FailedToCopyFile(image_path.to_string(), new_image_path)
            })?;
            fs::copy(label_path.clone(), new_label_path.clone()).map_err(|_| {
                ExportError::FailedToCopyFile(label_path.to_string(), new_label_path)
            })?;
        }

        Ok(())
    }

    fn create_yolo_yaml(project_name: &str, paths: &Paths, classes: &HashMap<isize, String>) {
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
