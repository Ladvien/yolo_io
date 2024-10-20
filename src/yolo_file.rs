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

use std::{error::Error, fs::read_to_string};

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum YoloFileParseError {
    #[error("Invalid format for file '{0}'")]
    InvalidFormat(String),
    #[error("File '{0}' is empty")]
    EmptyFile(String),
    #[error("Duplicate entries found in file '{0}'")]
    DuplicateEntries(String),
    #[error("Unable to parse value '{1}' in file '{0}' on line {2}")]
    FailedToParseClassId(String, String, usize),
    #[error("Invalid class id '{1}' in file '{0}'")]
    ClassIdNotFound(String, i32),
    #[error("Invalid data value for '{2}' in file '{0}' on line {1}.  Value is '{3}'")]
    LabelDataOutOfRange(String, usize, String, String),
    #[error("Class ID is greater than 79 in file '{0}' on line {1}")]
    ClassIdGreaterThanMax(String, i32),
    #[error("Failed to parse '{1}' on line {2} in file '{0}'")]
    FailedToParseColumn(String, String, usize),
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct YoloClass {
    pub id: usize,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct YoloEntry {
    pub class: i32,
    pub x_center: f32,
    pub y_center: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct YoloFile {
    entries: Vec<YoloEntry>,
}

impl YoloFile {
    pub fn new(classes: Vec<YoloClass>, path: &str) -> Result<YoloFile, Box<dyn Error>> {
        let potential_file = read_to_string(path);

        let mut entries = Vec::<YoloEntry>::new();

        if let Ok(file) = potential_file {
            if file.is_empty() {
                return Err(Box::new(YoloFileParseError::EmptyFile(path.to_string())));
            }

            for (index, line) in file.lines().enumerate() {
                let parts: Vec<&str> = line.split(" ").collect();

                if parts.len() != 5 {
                    return Err(Box::new(YoloFileParseError::InvalidFormat(
                        path.to_string(),
                    )));
                }

                let class = parts[0].parse::<i32>().map_err(|_| {
                    Box::new(YoloFileParseError::FailedToParseClassId(
                        path.to_string(),
                        parts[0].to_string(),
                        index,
                    ))
                })?;

                let found = classes.iter().any(|c| c.id == class as usize);
                if !found {
                    return Err(Box::new(YoloFileParseError::ClassIdNotFound(
                        path.to_string(),
                        class,
                    )));
                }

                let x_center = parts[1].parse::<f32>().map_err(|_| {
                    Box::new(YoloFileParseError::FailedToParseColumn(
                        path.to_string(),
                        'x'.to_string(),
                        index,
                    ))
                })?;

                let y_center = parts[2].parse::<f32>().map_err(|_| {
                    Box::new(YoloFileParseError::FailedToParseColumn(
                        path.to_string(),
                        'y'.to_string(),
                        index,
                    ))
                })?;

                let width = parts[3].parse::<f32>().map_err(|_| {
                    Box::new(YoloFileParseError::FailedToParseColumn(
                        path.to_string(),
                        'w'.to_string(),
                        index,
                    ))
                })?;

                let height = parts[4].parse::<f32>().map_err(|_| {
                    Box::new(YoloFileParseError::FailedToParseColumn(
                        path.to_string(),
                        'h'.to_string(),
                        index,
                    ))
                })?;

                // if !(0..=79).contains(&class) {
                //     return Err(Box::new(YoloFileParseError::ClassIdGreaterThanMax(
                //         path.to_string(),
                //         class,
                //     )));
                // }

                if !(0.0..=1.0).contains(&x_center) {
                    return Err(Box::new(YoloFileParseError::LabelDataOutOfRange(
                        path.to_string(),
                        index,
                        "x".to_string(),
                        x_center.to_string(),
                    )));
                }

                if !(0.0..=1.0).contains(&y_center) {
                    return Err(Box::new(YoloFileParseError::LabelDataOutOfRange(
                        path.to_string(),
                        index,
                        "y".to_string(),
                        y_center.to_string(),
                    )));
                }

                if !(0.0..=1.0).contains(&width) {
                    return Err(Box::new(YoloFileParseError::LabelDataOutOfRange(
                        path.to_string(),
                        index,
                        "w".to_string(),
                        width.to_string(),
                    )));
                }

                if !(0.0..=1.0).contains(&height) {
                    return Err(Box::new(YoloFileParseError::LabelDataOutOfRange(
                        path.to_string(),
                        index,
                        "h".to_string(),
                        height.to_string(),
                    )));
                }

                entries.push(YoloEntry {
                    class,
                    x_center,
                    y_center,
                    width,
                    height,
                });
            }

            // TODO: Check for duplicate labels with tolerance
            let mut class_ids = Vec::<i32>::new();
            for entry in &entries {}
        }

        Ok(YoloFile { entries })
    }
}
