use itertools::{EitherOrBoth, Itertools};
use std::path::PathBuf;

use crate::types::{
    DuplicateImageLabelPair, FileMetadata, ImageLabelPair, PairingError, PairingResult, PathWithKey,
};
use crate::YoloFile;

/// Pair images and labels based on matching file stems.
pub fn pair(
    file_metadata: FileMetadata,
    stems: Vec<String>,
    label_filenames: Vec<PathWithKey>,
    image_filenames: Vec<PathWithKey>,
) -> Vec<PairingResult> {
    let mut pairs: Vec<PairingResult> = Vec::new();

    for stem in stems {
        let mut image_paths_for_stem = image_filenames
<<<<<<< HEAD
            .iter()
=======
            .clone()
            .into_iter()
>>>>>>> 5664eeae26253c3b7baffffbabeffeaeec214498
            .filter(|image| image.key == *stem)
            .map(|image| image.path.clone())
            .collect::<Vec<PathBuf>>();
        image_paths_for_stem.sort();
        let image_paths_for_stem = image_paths_for_stem
            .iter()
            .map(|image| match image.to_str() {
                Some(path) => Ok(path.to_string()),
                None => Err(()),
            })
            .collect::<Vec<Result<String, ()>>>();

<<<<<<< HEAD
        let mut label_paths_for_stem = label_filenames
            .iter()
=======
        image_paths_for_stem.sort_by(|a, b| {
            let a_str = a.as_ref().map(|s| s.as_str()).unwrap_or("");
            let b_str = b.as_ref().map(|s| s.as_str()).unwrap_or("");
            a_str.cmp(b_str)
        });

        let mut label_paths_for_stem = label_filenames
            .clone()
            .into_iter()
>>>>>>> 5664eeae26253c3b7baffffbabeffeaeec214498
            .filter(|label| label.key == *stem)
            .map(|label| label.path.clone())
            .collect::<Vec<PathBuf>>();
        label_paths_for_stem.sort();
        let label_paths_for_stem = label_paths_for_stem
            .iter()
            .map(|label| match label.to_str() {
                Some(path) => Ok(path.to_string()),
                None => Err(()),
            })
            .collect::<Vec<Result<String, ()>>>();

        label_paths_for_stem.sort_by(|a, b| {
            let a_str = a.as_ref().map(|s| s.as_str()).unwrap_or("");
            let b_str = b.as_ref().map(|s| s.as_str()).unwrap_or("");
            a_str.cmp(b_str)
        });

        let invalid_pairs = process_label_path(&file_metadata, label_paths_for_stem.clone());

        // Remove invalid paths from label_paths_for_stem
        let label_paths_for_stem = label_paths_for_stem
            .into_iter()
            .filter(|path| path.is_ok())
            .collect::<Vec<Result<String, ()>>>();

        let unconfirmed_pairs = image_paths_for_stem
            .into_iter()
            .zip_longest(label_paths_for_stem.into_iter());

        let mut primary_pair: Option<ImageLabelPair> = None;

        for pair in unconfirmed_pairs {
            let result = evaluate_pair(stem.clone(), pair.clone(), &file_metadata);

            match result {
                PairingResult::Valid(pair) => match primary_pair {
                    Some(ref primary) => {
                        let error = if primary.label_file != pair.label_file {
                            PairingError::DuplicateLabelMismatch(DuplicateImageLabelPair {
                                name: stem.clone(),
                                primary: primary.clone(),
                                duplicate: pair.clone(),
                            })
                        } else {
                            PairingError::Duplicate(DuplicateImageLabelPair {
                                name: stem.clone(),
                                primary: primary.clone(),
                                duplicate: pair.clone(),
                            })
                        };
                        pairs.push(PairingResult::Invalid(error));
                    }
                    None => {
                        primary_pair = Some(pair.clone());
                        pairs.push(PairingResult::Valid(pair));
                    }
                },
                PairingResult::Invalid(error) => {
                    pairs.push(PairingResult::Invalid(error));
                }
            }
        }

        pairs.extend(invalid_pairs);
    }

    pairs
}

/// Validate all label files for a single stem.
pub fn process_label_path(
    file_metadata: &FileMetadata,
    label_paths_for_stem: Vec<Result<String, ()>>,
) -> Vec<PairingResult> {
    let mut invalid_pairs = Vec::<PairingResult>::new();

    if label_paths_for_stem.is_empty() {
        invalid_pairs.push(PairingResult::Invalid(
            PairingError::LabelFileMissingUnableToUnwrapImagePath,
        ));
    } else {
        for label_path in &label_paths_for_stem {
            if let Ok(path) = label_path {
                let yolo_file = YoloFile::new(file_metadata, path);
                match yolo_file {
                    Ok(_) => {}
                    Err(error) => {
                        invalid_pairs
                            .push(PairingResult::Invalid(PairingError::LabelFileError(error)));
                    }
                }
            } else {
                invalid_pairs.push(PairingResult::Invalid(
                    PairingError::LabelFileMissingUnableToUnwrapImagePath,
                ));
            }
        }
    }

    invalid_pairs
}

/// Build a [`PairingResult`] from a potential image/label pair.
pub fn evaluate_pair(
    stem: String,
    pair: EitherOrBoth<Result<String, ()>>,
    metadata: &FileMetadata,
) -> PairingResult {
    match pair {
        EitherOrBoth::Both(image_path, label_path) => match (image_path, label_path) {
            (Ok(image_path), Ok(label_path)) => {
                let label_file = match YoloFile::new(metadata, &label_path) {
                    Ok(file) => Some(file),
                    Err(error) => {
                        return PairingResult::Invalid(PairingError::LabelFileError(error))
                    }
                };

                PairingResult::Valid(ImageLabelPair {
                    name: stem,
                    image_path: Some(PathBuf::from(image_path)),
                    label_file,
                })
            }
            (Ok(image_path), Err(_)) => {
                PairingResult::Invalid(PairingError::LabelFileMissing(image_path))
            }
            (Err(_), Ok(label_path)) => {
                PairingResult::Invalid(PairingError::ImageFileMissing(label_path))
            }
            (Err(_), Err(_)) => PairingResult::Invalid(PairingError::BothFilesMissing),
        },
        EitherOrBoth::Left(image_path) => match image_path {
            Ok(image_path) => PairingResult::Invalid(PairingError::LabelFileMissing(image_path)),
            Err(_) => PairingResult::Invalid(PairingError::LabelFileMissingUnableToUnwrapImagePath),
        },
        EitherOrBoth::Right(label_path) => match label_path {
            Ok(label_path) => PairingResult::Invalid(PairingError::ImageFileMissing(label_path)),
            Err(_) => PairingResult::Invalid(PairingError::ImageFileMissingUnableToUnwrapLabelPath),
        },
    }
}
