mod common;

#[cfg(test)]
mod duplicate_tests {

    use std::path::PathBuf;

    use crate::common::{
        create_dir_and_write_file, create_image_file, create_yolo_project_config, image_data,
        TEST_SANDBOX_DIR,
    };
    use image::{ImageBuffer, Rgb};
    use rstest::rstest;
    use yolo_io::{YoloProject, YoloProjectConfig};

    #[rstest]
    fn test_duplicate_pairs_found(
        mut create_yolo_project_config: YoloProjectConfig,
        image_data: ImageBuffer<Rgb<u8>, Vec<u8>>,
    ) {
        let filename = "dup_one";
        let this_test_directory = format!("{}/{}/", TEST_SANDBOX_DIR, filename);

        // Remove the directory if it exists
        let _ = std::fs::remove_dir_all(&this_test_directory);

        let image_file = PathBuf::from(format!("{}/test1.jpg", this_test_directory));
        create_image_file(&image_file, &image_data);

        let image_file_duplicate =
            PathBuf::from(format!("{}/elsewhere/test1.jpg", this_test_directory));
        create_image_file(&image_file_duplicate, &image_data);

        let label_file = PathBuf::from(format!("{}/test1.txt", this_test_directory));
        create_dir_and_write_file(&label_file, "0 0.5 0.5 0.5 0.5");

        let label_file_duplicate =
            PathBuf::from(format!("{}/elsewhere/test1.txt", this_test_directory));
        create_dir_and_write_file(&label_file_duplicate, "0 0.5 0.5 0.5 0.5");

        create_yolo_project_config.source_paths.images = this_test_directory.clone();
        create_yolo_project_config.source_paths.labels = this_test_directory.clone();

        let project =
            YoloProject::new(&create_yolo_project_config).expect("Unable to create project");

        let valid_pairs = project.get_valid_pairs();
        let invalid_pairs = project.get_invalid_pairs();

        let valid_pair = valid_pairs.into_iter().find(|pair| pair.name == "test1");
        let invalid_pair = invalid_pairs.into_iter().find(|pair| {
            matches!(
                pair,
                yolo_io::PairingError::Duplicate(_)
                    | yolo_io::PairingError::DuplicateLabelMismatch(_)
            )
        });

        assert!(valid_pair.is_some());
        assert!(invalid_pair.is_some());
    }

    #[rstest]
    fn test_multiple_duplicate_pairs_found(
        mut create_yolo_project_config: YoloProjectConfig,
        image_data: ImageBuffer<Rgb<u8>, Vec<u8>>,
    ) {
        let filename = "dup_two";

        let file_key = "test_duplicate";

        let this_test_directory = format!("{}/{}/", TEST_SANDBOX_DIR, filename);

        let image_file = PathBuf::from(format!("{}/{}.png", this_test_directory, file_key));
        create_image_file(&image_file, &image_data);

        let image_file_duplicate = PathBuf::from(format!(
            "{}/elsewhere/{}.png",
            this_test_directory, file_key
        ));
        create_image_file(&image_file_duplicate, &image_data);

        let label_file = PathBuf::from(format!("{}/{}.txt", this_test_directory, file_key));
        create_dir_and_write_file(&label_file, "0 0.5 0.5 0.5 0.5");

        let label_file_duplicate = PathBuf::from(format!(
            "{}/elsewhere/{}.txt",
            this_test_directory, file_key
        ));
        create_dir_and_write_file(&label_file_duplicate, "0 0.5 0.5 0.5 0.5");

        let image_file2 = PathBuf::from(format!("{}/{}.png", this_test_directory, file_key));
        create_image_file(&image_file2, &image_data);

        let image_file_duplicate2 = PathBuf::from(format!(
            "{}/not_there/{}.png",
            this_test_directory, file_key
        ));
        create_image_file(&image_file_duplicate2, &image_data);

        let label_file2 = PathBuf::from(format!("{}/{}.txt", this_test_directory, file_key));
        create_dir_and_write_file(&label_file2, "0 0.5 0.5 0.5 0.5");

        let label_file_duplicate2 = PathBuf::from(format!(
            "{}/not_there/{}.txt",
            this_test_directory, file_key
        ));
        create_dir_and_write_file(&label_file_duplicate2, "0 0.5 0.5 0.5 0.5");

        create_yolo_project_config.source_paths.images = this_test_directory.clone();
        create_yolo_project_config.source_paths.labels = this_test_directory.clone();

        let project = YoloProject::new(&create_yolo_project_config).unwrap();

        let valid_pairs = project.get_valid_pairs();
        let invalid_pairs = project.get_invalid_pairs();

        let valid_pair = valid_pairs.into_iter().find(|pair| pair.name == file_key);
        let invalid_pairs = invalid_pairs
            .into_iter()
            .filter(|pair| {
                matches!(
                    pair,
                    yolo_io::PairingError::Duplicate(_)
                        | yolo_io::PairingError::DuplicateLabelMismatch(_)
                )
            })
            .collect::<Vec<_>>();

        assert!(valid_pair.is_some());
        assert_eq!(invalid_pairs.len(), 2);
    }

    #[rstest]
<<<<<<< HEAD
    fn test_duplicate_label_files_with_different_data(
        mut create_yolo_project_config: YoloProjectConfig,
        image_data: ImageBuffer<Rgb<u8>, Vec<u8>>,
    ) {
        let filename = "dup_different";
        let this_test_directory = format!("{}/{}/", TEST_SANDBOX_DIR, filename);

        let image_file = PathBuf::from(format!("{}/test.jpg", this_test_directory));
        create_image_file(&image_file, &image_data);

        let image_file_duplicate =
            PathBuf::from(format!("{}/elsewhere/test.jpg", this_test_directory));
        create_image_file(&image_file_duplicate, &image_data);

        let label_file = PathBuf::from(format!("{}/test.txt", this_test_directory));
        create_dir_and_write_file(&label_file, "0 0.5 0.5 0.5 0.5");

        let label_file_duplicate =
            PathBuf::from(format!("{}/elsewhere/test.txt", this_test_directory));
        create_dir_and_write_file(&label_file_duplicate, "0 0.6 0.6 0.5 0.5");
=======
    fn test_duplicate_pairs_with_different_labels(
        mut create_yolo_project_config: YoloProjectConfig,
        image_data: ImageBuffer<Rgb<u8>, Vec<u8>>,
    ) {
        let filename = "dup_three";
        let this_test_directory = format!("{}/{}/", TEST_SANDBOX_DIR, filename);

        let image_file = PathBuf::from(format!("{}/test1.jpg", this_test_directory));
        create_image_file(&image_file, &image_data);

        let image_file_duplicate = PathBuf::from(format!("{}/else/test1.jpg", this_test_directory));
        create_image_file(&image_file_duplicate, &image_data);

        let label_file = PathBuf::from(format!("{}/test1.txt", this_test_directory));
        create_dir_and_write_file(&label_file, "0 0.5 0.5 0.5 0.5");

        let label_file_duplicate = PathBuf::from(format!("{}/else/test1.txt", this_test_directory));
        create_dir_and_write_file(&label_file_duplicate, "1 0.5 0.5 0.5 0.5");
>>>>>>> 4f08b15df24ace696343f6d3fd4485ad08bb764b

        create_yolo_project_config.source_paths.images = this_test_directory.clone();
        create_yolo_project_config.source_paths.labels = this_test_directory.clone();

        let project = YoloProject::new(&create_yolo_project_config).unwrap();

<<<<<<< HEAD
        let valid_pairs = project.get_valid_pairs();
        let invalid_pairs = project.get_invalid_pairs();

        let valid_pair = valid_pairs.into_iter().find(|pair| pair.name == "test");
        let duplicate_error = invalid_pairs
            .into_iter()
            .find(|pair| matches!(pair, yolo_io::PairingError::Duplicate(_)));

        assert!(valid_pair.is_some());
        assert!(duplicate_error.is_some());
=======
        let invalid_pairs = project.get_invalid_pairs();
        let mismatch = invalid_pairs
            .into_iter()
            .find(|pair| matches!(pair, yolo_io::PairingError::DuplicateLabelMismatch(_)));

        assert!(mismatch.is_some());
>>>>>>> 4f08b15df24ace696343f6d3fd4485ad08bb764b
    }
}
