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

/// Errors that can occur when parsing a YOLO label file.

#[derive(Error, Clone, PartialEq, Debug, Serialize, Deserialize)]
/// Detailed reasons a label file failed to parse.
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
/// Additional information associated with a [`YoloFileParseError`].
pub struct YoloFileParseErrorDetails {
    /// Path of the file that failed.
    pub path: String,
    /// Class value being parsed when the error occurred.
    pub class: Option<String>,
    /// Line number of the offending entry.
    pub row: Option<usize>,
    /// Line number of a duplicate entry if relevant.
    pub other_row: Option<usize>,
    /// Column name associated with the error.
    pub column: Option<String>,
    /// The offending numeric value if available.
    pub value: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
/// Class definition used when validating label files.
pub struct YoloClass {
    /// Numeric class identifier.
    pub id: isize,
    /// Human readable class name.
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// A single label entry in a YOLO file.
pub struct YoloEntry {
    /// Class identifier.
    pub class: isize,
    /// Normalized x coordinate of the bounding box centre.
    pub x_center: f32,
    /// Normalized y coordinate of the bounding box centre.
    pub y_center: f32,
    /// Normalized bounding box width.
    pub width: f32,
    /// Normalized bounding box height.
    pub height: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// Representation of a `.txt` label file in YOLO format.
pub struct YoloFile {
    /// File stem without extension.
    pub stem: String,
    /// Full path to the label file on disk.
    pub path: String,
    /// Parsed label entries.
    pub entries: Vec<YoloEntry>,
}

impl YoloFile {
    /// Read and validate a label file.
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

            let tolerance = metadata.duplicate_tolerance;
            let mut seen_bboxes: Vec<(f32, f32, f32, f32)> = Vec::new();

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

                let bbox = (
                    x_center - width / 2.0,
                    x_center + width / 2.0,
                    y_center - height / 2.0,
                    y_center + height / 2.0,
                );

                if tolerance > 0.0 {
                    for (prev_index, (px1, px2, py1, py2)) in seen_bboxes.iter().enumerate() {
                        if (bbox.0 - *px1).abs() <= tolerance
                            && (bbox.1 - *px2).abs() <= tolerance
                            && (bbox.2 - *py1).abs() <= tolerance
                            && (bbox.3 - *py2).abs() <= tolerance
                        {
                            return Err(YoloFileParseError::DuplicateEntries(
                                YoloFileParseErrorDetails {
                                    path: path.to_string(),
                                    class: None,
                                    row: Some(prev_index),
                                    other_row: Some(index),
                                    column: None,
                                    value: None,
                                },
                            ));
                        }
                    }
                }

                seen_bboxes.push(bbox);

                entries.push(YoloEntry {
                    class,
                    x_center,
                    y_center,
                    width,
                    height,
                });
            }
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
}
