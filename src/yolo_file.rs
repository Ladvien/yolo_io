/*
   1. Check for empty file
   2. Check for corrupted format
   3. Check if duplicates exist in the same file.
   4. Check if invalid class ids exist
   5. Check if points are normalized 0.0 - 1.0

   <class> <x_center> <y_center> <width> <height>
   <class>: The class label of the object.
   <x_center>: The normalized x-coordinate of the bounding box center.
   <y_center>: The normalized y-coordinate of the bounding box center.
   <width>: The normalized width of the bounding box.
   <height>: The normalized height of the bounding box.
*/

use serde::{Deserialize, Serialize};
use std::{fs::read_to_string, path::Path};
use thiserror::Error;

use crate::{file_utils::get_file_stem, types::FileMetadata};

#[derive(Error, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum YoloFileParseError {
    #[error("Invalid format for file '{}'", .0.path)]
    InvalidFormat(YoloFileParseErrorDetails),
    #[error("File '{}' is empty", .0.path)]
    EmptyFile(YoloFileParseErrorDetails),
    #[error("Duplicate entries found in file '{}' on row {} and row {}", .0.path, .0.row.unwrap(), .0.other_row.unwrap())]
    DuplicateEntries(YoloFileParseErrorDetails),
    #[error("Unable to parse value '{}' in file '{}' on line {}", .0.class.clone().unwrap(), .0.path, .0.row.unwrap())]
    FailedToParseClassId(YoloFileParseErrorDetails),
    #[error("Invalid class id '{}' in file '{}'", .0.class.clone().unwrap(), .0.path)]
    ClassIdNotFound(YoloFileParseErrorDetails),
    #[error("Invalid data value for '{}' in file '{}' on line {}.  Value is '{}'", .0.column.clone().unwrap(), .0.path, .0.row.unwrap(), .0.value.unwrap())]
    LabelDataOutOfRange(YoloFileParseErrorDetails),
    #[error("Failed to parse '{}' column with value of '{}' on line {} in file '{}'", .0.column.clone().unwrap(), .0.class.clone().unwrap(), .0.row.unwrap(), .0.path)]
    FailedToParseColumn(YoloFileParseErrorDetails),
    #[error("Failed to get file stem for file '{}'", .0.path)]
    FailedToGetFileStem(YoloFileParseErrorDetails),
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct YoloFileParseErrorDetails {
    pub path: String,
    pub class: Option<String>,
    pub row: Option<usize>,
    pub other_row: Option<usize>,
    pub column: Option<String>,
    pub value: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct YoloClass {
    pub id: isize,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct YoloEntry {
    pub class: isize,
    pub x_center: f32,
    pub y_center: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct YoloFile {
    pub stem: String,
    pub path: String,
    pub entries: Vec<YoloEntry>,
}

impl YoloFile {
    pub fn new(metadata: &FileMetadata, path: &String) -> Result<YoloFile, YoloFileParseError> {
        let potential_file = read_to_string(path);

        let mut entries = Vec::<YoloEntry>::new();

        if let Ok(file) = potential_file {
            if file.is_empty() {
                let details = YoloFileParseErrorDetails {
                    path: path.to_string(),
                    class: None,
                    row: None,
                    other_row: None,
                    column: None,
                    value: None,
                };

                return Err(YoloFileParseError::EmptyFile(details));
            }

            for (index, line) in file.lines().enumerate() {
                let parts: Vec<&str> = line.split(" ").collect();

                if parts.len() != 5 {
                    return Err(YoloFileParseError::InvalidFormat(
                        YoloFileParseErrorDetails {
                            path: path.to_string(),
                            class: None,
                            row: None,
                            other_row: None,
                            column: None,
                            value: None,
                        },
                    ));
                }

                let class = parts[0].parse::<isize>().map_err(|_| {
                    YoloFileParseError::FailedToParseClassId(YoloFileParseErrorDetails {
                        path: path.to_string(),
                        class: Some(parts[0].to_string()),
                        row: Some(index),
                        other_row: None,
                        column: Some("class".to_string()),
                        value: None,
                    })
                })?;

                let found = metadata.classes.iter().any(|c| c.id == class);
                if !found {
                    return Err(YoloFileParseError::ClassIdNotFound(
                        YoloFileParseErrorDetails {
                            path: path.to_string(),
                            class: Some(class.to_string()),
                            row: Some(index),
                            other_row: None,
                            column: Some("class".to_string()),
                            value: None,
                        },
                    ));
                }

                let x_center = parts[1].parse::<f32>().map_err(|_| {
                    YoloFileParseError::FailedToParseColumn(YoloFileParseErrorDetails {
                        path: path.to_string(),
                        class: Some(class.to_string()),
                        row: Some(index),
                        other_row: None,
                        column: Some("x".to_string()),
                        value: None,
                    })
                })?;

                let y_center = parts[2].parse::<f32>().map_err(|_| {
                    YoloFileParseError::FailedToParseColumn(YoloFileParseErrorDetails {
                        path: path.to_string(),
                        class: Some(class.to_string()),
                        row: Some(index),
                        other_row: None,
                        column: Some("y".to_string()),
                        value: None,
                    })
                })?;

                let width = parts[3].parse::<f32>().map_err(|_| {
                    YoloFileParseError::FailedToParseColumn(YoloFileParseErrorDetails {
                        path: path.to_string(),
                        class: Some(class.to_string()),
                        row: Some(index),
                        other_row: None,
                        column: Some("w".to_string()),
                        value: None,
                    })
                })?;

                let height = parts[4].parse::<f32>().map_err(|_| {
                    YoloFileParseError::FailedToParseColumn(YoloFileParseErrorDetails {
                        path: path.to_string(),
                        class: Some(class.to_string()),
                        row: Some(index),
                        other_row: None,
                        column: Some("h".to_string()),
                        value: None,
                    })
                })?;

                // if !(0..=79).contains(&class) {
                //     return Err(Box::new(YoloFileParseError::ClassIdGreaterThanMax(
                //         path.to_string(),
                //         class,
                //     )));
                // }

                if !(0.0..=1.0).contains(&x_center) {
                    return Err(YoloFileParseError::LabelDataOutOfRange(
                        YoloFileParseErrorDetails {
                            path: path.to_string(),
                            class: Some(class.to_string()),
                            row: Some(index),
                            other_row: None,
                            column: Some("x".to_string()),
                            value: Some(x_center),
                        },
                    ));
                }

                if !(0.0..=1.0).contains(&y_center) {
                    return Err(YoloFileParseError::LabelDataOutOfRange(
                        YoloFileParseErrorDetails {
                            path: path.to_string(),
                            class: Some(class.to_string()),
                            row: Some(index),
                            other_row: None,
                            column: Some("y".to_string()),
                            value: Some(y_center),
                        },
                    ));
                }

                if !(0.0..=1.0).contains(&width) {
                    return Err(YoloFileParseError::LabelDataOutOfRange(
                        YoloFileParseErrorDetails {
                            path: path.to_string(),
                            class: Some(class.to_string()),
                            row: Some(index),
                            other_row: None,
                            column: Some("w".to_string()),
                            value: Some(width),
                        },
                    ));
                }

                if !(0.0..=1.0).contains(&height) {
                    return Err(YoloFileParseError::LabelDataOutOfRange(
                        YoloFileParseErrorDetails {
                            path: path.to_string(),
                            class: Some(class.to_string()),
                            row: Some(index),
                            other_row: None,
                            column: Some("h".to_string()),
                            value: Some(height),
                        },
                    ));
                }

                entries.push(YoloEntry {
                    class,
                    x_center,
                    y_center,
                    width,
                    height,
                });
            }

            // TODO: I need to move these checks into the inner loop.
            let mut label_coordinates = Vec::<(f32, f32, f32, f32)>::new();

            for entry in &entries {
                let x1 = entry.x_center - entry.width / 2.0;
                let x2 = entry.x_center + entry.width / 2.0;
                let y1 = entry.y_center - entry.height / 2.0;
                let y2 = entry.y_center + entry.height / 2.0;

                label_coordinates.push((x1, x2, y1, y2))
            }

            let tolerance = 0.01;
            if let Some(indices) = Self::get_duplicate_index(&label_coordinates, tolerance) {
                return Err(YoloFileParseError::DuplicateEntries(
                    YoloFileParseErrorDetails {
                        path: path.to_string(),
                        class: None,
                        row: Some(indices.0),
                        other_row: Some(indices.1),
                        column: None,
                        value: None,
                    },
                ));
            };
        }

        let stem = get_file_stem(Path::new(path))
            .map_err(|_| {
                YoloFileParseError::FailedToGetFileStem(YoloFileParseErrorDetails {
                    path: path.to_string(),
                    class: None,
                    row: None,
                    other_row: None,
                    column: None,
                    value: None,
                })
            })?
            .to_string();

        Ok(YoloFile {
            stem,
            path: path.to_string(),
            entries,
        })
    }

    fn get_duplicate_index(
        duplicated_labels: &[(f32, f32, f32, f32)],
        tolerance: f32,
    ) -> Option<(usize, usize)> {
        for (index, coordinates) in duplicated_labels.iter().enumerate() {
            let (x1, x2, y1, y2) = coordinates;

            for (duplicate_index, other_coordinates) in duplicated_labels.iter().enumerate() {
                if duplicate_index != index {
                    let (ox1, ox2, oy1, oy2) = other_coordinates;

                    if (x1 - ox1).abs() < tolerance
                        && (x2 - ox2).abs() < tolerance
                        && (y1 - oy1).abs() < tolerance
                        && (y2 - oy2).abs() < tolerance
                    {
                        return Some((index, duplicate_index));
                    }
                }
            }
        }
        None
    }
}
