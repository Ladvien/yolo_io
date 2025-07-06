use itertools::{EitherOrBoth, Itertools};
use std::path::PathBuf;

use crate::types::{
    DuplicateImageLabelPair, FileMetadata, ImageLabelPair, PairingError, PairingResult, PathWithKey,
};
use crate::YoloFile;

pub fn pair(
    file_metadata: FileMetadata,
    stems: Vec<String>,
    label_filenames: Vec<PathWithKey>,
    image_filenames: Vec<PathWithKey>,
) -> Vec<PairingResult> {
    let mut pairs = Vec::new();

    for stem in stems {
<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
=======
>>>>>>> c3b6efd01ea4f59079e5734f0465ca98e4559444
        let image_paths_for_stem = image_filenames
            .clone()
            .into_iter()
=======
        let mut image_paths_for_stem = image_filenames
            .iter()
>>>>>>> f81ccc4939ee178da75b073df90b7d5c05d68f4f
=======
        let mut image_paths_for_stem = image_filenames
            .iter()
>>>>>>> 0b309e9da26ac872d7ffa5dc0125e56dd2d7e65d
            .filter(|image| image.key == *stem)
            .map(|image| match image.clone().path.to_str() {
<<<<<<< HEAD
=======
=======
>>>>>>> 4f08b15df24ace696343f6d3fd4485ad08bb764b
        let mut image_paths_for_stem = image_filenames
            .clone()
            .into_iter()
            .filter(|image| image.key == *stem)
<<<<<<< HEAD
            .map(|image| image.path.clone())
            .collect::<Vec<PathBuf>>();
<<<<<<< HEAD
        image_paths_for_stem.sort();
        let mut image_paths_for_stem = image_paths_for_stem
            .iter()
            .map(|image| match image.to_str() {
>>>>>>> 41a5c29104dc33c0f0f2a3a1576287e6710baaeb
=======
        let image_paths_for_stem = image_filenames
            .clone()
            .into_iter()
            .filter(|image| image.key == *stem)
            .map(|image| match image.clone().path.to_str() {
>>>>>>> c9cf85d60740a6510ca489f36753e559018a9dbe
=======
            .map(|image| match image.clone().path.to_str() {
>>>>>>> 4f08b15df24ace696343f6d3fd4485ad08bb764b
=======
>>>>>>> c3b6efd01ea4f59079e5734f0465ca98e4559444
=======
        let image_paths_for_stem = image_filenames
            .clone()
            .into_iter()
            .filter(|image| image.key == *stem)
            .map(|image| match image.clone().path.to_str() {
>>>>>>> d5f8f38db09703cc0d2b505bc98688e51c43f07b
                Some(path) => Ok(path.to_string()),
                None => Err(()),
            })
            .collect::<Vec<Result<String, ()>>>();
<<<<<<< HEAD

<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
        let label_paths_for_stem = label_filenames
            .clone()
            .into_iter()
            .filter(|label| label.key == *stem)
            .map(|label| match label.clone().path.to_str() {
=======
=======
>>>>>>> 4f08b15df24ace696343f6d3fd4485ad08bb764b
=======
>>>>>>> f81ccc4939ee178da75b073df90b7d5c05d68f4f
=======
        image_paths_for_stem.sort_by(|a, b| a.to_string_lossy().cmp(&b.to_string_lossy()));

        let mut image_paths_for_stem = image_paths_for_stem
            .iter()
            .map(|image| match image.to_str() {
                Some(p) => Ok(p.to_string()),
                None => Err(()),
            })
            .collect::<Vec<Result<String, ()>>>();
>>>>>>> 0b309e9da26ac872d7ffa5dc0125e56dd2d7e65d
        image_paths_for_stem.sort_by(|a, b| {
            let a_str = a.as_ref().map(|s| s.as_str()).unwrap_or("");
            let b_str = b.as_ref().map(|s| s.as_str()).unwrap_or("");
            a_str.cmp(b_str)
        });

        let mut label_paths_for_stem = label_filenames
<<<<<<< HEAD
<<<<<<< HEAD
            .clone()
            .into_iter()
=======
            .iter()
>>>>>>> f81ccc4939ee178da75b073df90b7d5c05d68f4f
=======
            .iter()
>>>>>>> 0b309e9da26ac872d7ffa5dc0125e56dd2d7e65d
            .filter(|label| label.key == *stem)
<<<<<<< HEAD
            .map(|label| label.path.clone())
            .collect::<Vec<PathBuf>>();
<<<<<<< HEAD
        label_paths_for_stem.sort();
        let mut label_paths_for_stem = label_paths_for_stem
            .iter()
            .map(|label| match label.to_str() {
>>>>>>> 41a5c29104dc33c0f0f2a3a1576287e6710baaeb
=======
        let label_paths_for_stem = label_filenames
            .clone()
            .into_iter()
            .filter(|label| label.key == *stem)
            .map(|label| match label.clone().path.to_str() {
>>>>>>> c9cf85d60740a6510ca489f36753e559018a9dbe
=======
            .map(|label| match label.clone().path.to_str() {
>>>>>>> 4f08b15df24ace696343f6d3fd4485ad08bb764b
=======
        let label_paths_for_stem = label_filenames
            .clone()
            .into_iter()
            .filter(|label| label.key == *stem)
            .map(|label| match label.clone().path.to_str() {
>>>>>>> c3b6efd01ea4f59079e5734f0465ca98e4559444
=======
        let label_paths_for_stem = label_filenames
            .clone()
            .into_iter()
            .filter(|label| label.key == *stem)
            .map(|label| match label.clone().path.to_str() {
>>>>>>> d5f8f38db09703cc0d2b505bc98688e51c43f07b
                Some(path) => Ok(path.to_string()),
                None => Err(()),
            })
            .collect::<Vec<Result<String, ()>>>();
<<<<<<< HEAD

<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
=======
=======
>>>>>>> 4f08b15df24ace696343f6d3fd4485ad08bb764b
=======
>>>>>>> f81ccc4939ee178da75b073df90b7d5c05d68f4f
=======
        label_paths_for_stem.sort_by(|a, b| a.to_string_lossy().cmp(&b.to_string_lossy()));

        let mut label_paths_for_stem = label_paths_for_stem
            .iter()
            .map(|label| match label.to_str() {
                Some(p) => Ok(p.to_string()),
                None => Err(()),
            })
            .collect::<Vec<Result<String, ()>>>();
>>>>>>> 0b309e9da26ac872d7ffa5dc0125e56dd2d7e65d
        label_paths_for_stem.sort_by(|a, b| {
            let a_str = a.as_ref().map(|s| s.as_str()).unwrap_or("");
            let b_str = b.as_ref().map(|s| s.as_str()).unwrap_or("");
            a_str.cmp(b_str)
        });
<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
>>>>>>> 41a5c29104dc33c0f0f2a3a1576287e6710baaeb
=======
>>>>>>> c9cf85d60740a6510ca489f36753e559018a9dbe
        let (invalid_pairs, valid_label_paths) =
            process_label_path(&file_metadata, label_paths_for_stem);
=======
=======
        let (invalid_pairs, valid_label_paths) =
            process_label_path(&file_metadata, label_paths_for_stem);
>>>>>>> c3b6efd01ea4f59079e5734f0465ca98e4559444
=======
=======
=======
        let (invalid_pairs, valid_label_paths) =
            process_label_path(&file_metadata, label_paths_for_stem);
>>>>>>> d5f8f38db09703cc0d2b505bc98688e51c43f07b

        let (invalid_pairs, valid_label_paths) =
            process_label_path(&file_metadata, label_paths_for_stem);
>>>>>>> 0b309e9da26ac872d7ffa5dc0125e56dd2d7e65d

        let (invalid_pairs, valid_label_paths) =
            process_label_path(&file_metadata, label_paths_for_stem);
>>>>>>> f81ccc4939ee178da75b073df90b7d5c05d68f4f

        let invalid_pairs = process_label_path(&file_metadata, label_paths_for_stem.clone());
>>>>>>> 4f08b15df24ace696343f6d3fd4485ad08bb764b

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
                    Some(ref primary_pair) => {
                        pairs.push(PairingResult::Invalid(PairingError::Duplicate(
                            DuplicateImageLabelPair {
                                name: stem.clone(),
                                primary: primary_pair.clone(),
                                duplicate: pair.clone(),
                            },
                        )));
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
<<<<<<< HEAD

<<<<<<< HEAD
=======
>>>>>>> 0b309e9da26ac872d7ffa5dc0125e56dd2d7e65d
=======
>>>>>>> d5f8f38db09703cc0d2b505bc98688e51c43f07b
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
