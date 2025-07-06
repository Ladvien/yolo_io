use std::path::Path;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::types::PathWithKey;

/// Errors returned when reading from or writing to the file system.

#[derive(Debug, Error, PartialEq, Clone, Serialize, Deserialize)]
pub enum FileError {
    #[error("Unable to convert {0} to string.")]
    ConvertPathToString(String),
    #[error("Unable to get file stem for path {0}.")]
    GetFileStem(String),
    #[error("Unable to read file: {0}.")]
    ReadFile(String),
    #[error("Unable to write file.")]
    WriteFile,
}

/// Return the file stem for a path.
pub fn get_file_stem(file_path: &Path) -> Result<&str, FileError> {
    file_path
        .file_stem()
        .ok_or_else(|| FileError::GetFileStem(file_path.display().to_string()))?
        .to_str()
        .ok_or_else(|| FileError::ConvertPathToString(file_path.display().to_string()))
}

fn get_file_extension(file_path: &Path) -> Result<&str, FileError> {
    let file_extension = file_path
        .extension()
        .ok_or(FileError::ReadFile(file_path.display().to_string()))?;
    let extension_str = file_extension
        .to_str()
        .ok_or(FileError::ConvertPathToString(
            file_path.display().to_string(),
        ))?;

    Ok(extension_str)
}

fn get_filepath_as_string(file_path: &Path) -> Result<String, FileError> {
    let file_path = file_path.to_str().ok_or(FileError::ConvertPathToString(
        file_path.display().to_string(),
    ))?;

    Ok(file_path.to_string())
}

/// Recursively gather all files under `path` with the given extensions.
pub fn get_filepaths_for_extension(
    path: &str,
    extensions: Vec<&str>,
) -> Result<Vec<PathWithKey>, FileError> {
    let file_paths = std::fs::read_dir(path).map_err(|err| FileError::ReadFile(err.to_string()))?;

    let mut paths = Vec::<PathWithKey>::new();
    let extensions_lower: Vec<String> = extensions.iter().map(|ext| ext.to_lowercase()).collect();

    for file_path in file_paths {
        let file_path = file_path
            .map_err(|err| FileError::ReadFile(err.to_string()))?
            .path();

        if file_path.is_dir() {
            let file_path = get_filepath_as_string(&file_path)?;
            let filepaths = get_filepaths_for_extension(&file_path, extensions.clone())?;
            paths.extend(filepaths);

            // Skip trying to get extension and stem for directories
            continue;
        }

        let extension = match get_file_extension(&file_path) {
            Ok(extension) => extension.to_lowercase(),
            Err(_) => continue,
        };

        let stem = match get_file_stem(&file_path) {
            Ok(stem) => stem,
            Err(_) => continue,
        };

        if extensions_lower.contains(&extension) {
            paths.push(PathWithKey {
                path: file_path.clone(),
                key: String::from(stem),
            });
        }
    }

<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
    // Ensure deterministic order of returned paths
<<<<<<< HEAD
<<<<<<< HEAD
=======

>>>>>>> 41a5c29104dc33c0f0f2a3a1576287e6710baaeb
=======
    // Ensure deterministic ordering
>>>>>>> c9cf85d60740a6510ca489f36753e559018a9dbe
=======
>>>>>>> 4f08b15df24ace696343f6d3fd4485ad08bb764b
=======
    // Ensure deterministic ordering
>>>>>>> c3b6efd01ea4f59079e5734f0465ca98e4559444
=======
    // Ensure deterministic ordering
>>>>>>> f81ccc4939ee178da75b073df90b7d5c05d68f4f
=======
    // Ensure deterministic order of returned paths
>>>>>>> 0b309e9da26ac872d7ffa5dc0125e56dd2d7e65d
=======
    // Ensure deterministic order of returned paths
>>>>>>> d5f8f38db09703cc0d2b505bc98688e51c43f07b
    paths.sort_by(|a, b| a.path.cmp(&b.path));

    Ok(paths)
}
