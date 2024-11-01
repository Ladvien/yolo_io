use crate::types::PathWithKey;

pub fn get_filepaths_for_extension(path: &str, extensions: Vec<&str>) -> Vec<PathWithKey> {
    let file_paths = std::fs::read_dir(path);

    if file_paths.is_err() {
        return Vec::<PathWithKey>::new();
    }

    let mut paths = Vec::<PathWithKey>::new();

    for file_path in file_paths.unwrap() {
        let file_path = file_path.unwrap().path();

        if file_path.is_dir() {
            let filepaths =
                get_filepaths_for_extension(file_path.to_str().unwrap(), extensions.clone());

            paths.extend(filepaths);
        }

        if let Some(file_extension) = file_path.extension() {
            let stem = file_path.file_stem().unwrap().to_str().unwrap();
            // TODO: Convert to return a PathWithKey
            let extension_str = file_extension.to_str().unwrap();

            if extensions.contains(&extension_str) {
                paths.push(PathWithKey {
                    path: file_path.clone(),
                    key: String::from(stem),
                });
            }
        }
    }

    paths
}
