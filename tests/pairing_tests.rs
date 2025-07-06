mod common;

#[cfg(test)]
mod pairing_tests {

    use image::{ImageBuffer, Rgb};
    use rstest::rstest;
    use std::{fs, path::PathBuf};
    use yolo_io::{YoloProject, YoloProjectConfig};

    use crate::common::{
        create_dir_and_write_file, create_image_file, create_yolo_project_config, image_data,
        TEST_SANDBOX_DIR,
    };

    /*
    Test Scenarios
        Type
        Error = E
        Warn  = W
        Valid = V
        Mixed = M

                 | 1 Label | No Label | Label >2
        1 Image  |  V      |   E      |  M
        No Image |  E      |   -      |  M
        Image >2 |  M      |   E      |  V
     */
    #[rstest]
    fn test_project_validation_produces_one_valid_pair_for_one_image_one_label(
        image_data: ImageBuffer<Rgb<u8>, Vec<u8>>,
        mut create_yolo_project_config: YoloProjectConfig,
    ) {
        let filename = "one";
        let this_test_directory = format!("{}/{}/", TEST_SANDBOX_DIR, filename);

        let image_file = PathBuf::from(format!("{}/test1.jpg", this_test_directory));
        create_image_file(&image_file, &image_data);

        let file1 = PathBuf::from(format!("{}/test1.txt", this_test_directory));
        create_dir_and_write_file(&file1, "0 0.5 0.5 0.5 0.5");

        create_yolo_project_config.source_paths.images = this_test_directory.clone();
        create_yolo_project_config.source_paths.labels = this_test_directory.clone();

        let project =
            YoloProject::new(&create_yolo_project_config).expect("Unable to create project");

        let valid_pairs = project.get_valid_pairs();
        let invalid_pairs = project.get_invalid_pairs();

        let valid_pair = valid_pairs.into_iter().find(|pair| pair.name == "test1");
        let invalid_pair = invalid_pairs
            .into_iter()
            .find(|pair| matches!(pair, yolo_io::PairingError::LabelFileError(_)));

        assert!(valid_pair.is_some());
        assert!(invalid_pair.is_none());
    }

    #[rstest]
    fn test_project_validation_produces_one_invalid_pair_for_one_image_no_label(
        image_data: ImageBuffer<Rgb<u8>, Vec<u8>>,
        mut create_yolo_project_config: YoloProjectConfig,
    ) {
        let filename = "two";
        let this_test_directory = format!("{}/{}/", TEST_SANDBOX_DIR, filename);

        let image_file = PathBuf::from(format!("{}/test2.jpg", this_test_directory));
        create_image_file(&image_file, &image_data);

        create_yolo_project_config.source_paths.images = this_test_directory.clone();
        create_yolo_project_config.source_paths.labels = this_test_directory.clone();

        let project =
            YoloProject::new(&create_yolo_project_config).expect("Unable to create project");

        let valid_pairs = project.get_valid_pairs();
        let invalid_pairs = project.get_invalid_pairs();

        let valid_pair = valid_pairs.into_iter().find(|pair| pair.name == "test2");

        let invalid_pair = invalid_pairs.into_iter().find(|pair| {
            matches!(
                pair,
                yolo_io::PairingError::LabelFileMissingUnableToUnwrapImagePath
            )
        });

        assert!(valid_pair.is_none());
        assert!(invalid_pair.is_some());
    }

    #[rstest]
    fn test_project_validation_produces_one_valid_pair_for_one_image_two_labels(
        image_data: ImageBuffer<Rgb<u8>, Vec<u8>>,
        mut create_yolo_project_config: YoloProjectConfig,
    ) {
        let filename = "three";
        create_yolo_project_config.source_paths.images =
            format!("{}/{}", TEST_SANDBOX_DIR, filename);
        create_yolo_project_config.source_paths.labels =
            format!("{}/{}", TEST_SANDBOX_DIR, filename);

        let this_test_directory = format!("{}/{}/", TEST_SANDBOX_DIR, filename);

        let image_file = PathBuf::from(format!("{}/test3.jpg", this_test_directory));
        create_image_file(&image_file, &image_data);

        let file1 = PathBuf::from(format!("{}/{}/test3.txt", this_test_directory, "one"));
        create_dir_and_write_file(&file1, "0 0.5 0.5 0.5 0.5");

        let file2 = PathBuf::from(format!("{}/{}/test3.txt", this_test_directory, "two"));

        // '3' isn't a valid class ID.
        create_dir_and_write_file(&file2, "3 0.5 0.5 0.5 0.5");

        let project =
            YoloProject::new(&create_yolo_project_config).expect("Unable to create project");
        let valid_pairs = project.get_valid_pairs();
        let invalid_pairs = project.get_invalid_pairs();

        let valid_pair = valid_pairs.into_iter().find(|pair| pair.name == "test3");
        let invalid_pair = invalid_pairs
            .into_iter()
            .find(|pair| matches!(pair, yolo_io::PairingError::LabelFileError(_)));

        assert!(valid_pair.is_some());
        assert!(invalid_pair.is_some());
    }

    #[rstest]
    fn test_project_validation_produces_one_invalid_pair_for_no_image_one_label(
        mut create_yolo_project_config: YoloProjectConfig,
    ) {
        let filename = "four";
        let this_test_directory = format!("{}/{}/", TEST_SANDBOX_DIR, filename);

        let file1 = PathBuf::from(format!("{}/test4.txt", this_test_directory));
        create_dir_and_write_file(&file1, "0 0.5 0.5 0.5 0.5");

        create_yolo_project_config.source_paths.images = this_test_directory.clone();
        create_yolo_project_config.source_paths.labels = this_test_directory.clone();

        let project =
            YoloProject::new(&create_yolo_project_config).expect("Unable to create project");

        let valid_pairs = project.get_valid_pairs();
        let invalid_pairs = project.get_invalid_pairs();

        let valid_pair = valid_pairs.into_iter().find(|pair| pair.name == "test4");
        let invalid_pair = invalid_pairs
            .into_iter()
            .find(|pair| matches!(pair, yolo_io::PairingError::ImageFileMissing(_)));

        assert!(valid_pair.is_none());
        assert!(invalid_pair.is_some());
    }

    #[rstest]
    fn test_project_validation_produces_one_invalid_pair_for_no_image_no_label(
        mut create_yolo_project_config: YoloProjectConfig,
    ) {
        // THIS ERROR IS IMPOSSIBLE TO TRIGGER
        let filename = "five";
        let this_test_directory = format!("{}/{}/", TEST_SANDBOX_DIR, filename);

        fs::create_dir_all(&this_test_directory).expect("Unable to create directory");

        create_yolo_project_config.source_paths.images = this_test_directory.clone();
        create_yolo_project_config.source_paths.labels = this_test_directory.clone();

        let project =
            YoloProject::new(&create_yolo_project_config).expect("Unable to create project");

        let valid_pairs = project.get_valid_pairs();
        let valid_pair = valid_pairs.into_iter().find(|pair| pair.name == "test5");

        let invalid_pairs = project.get_invalid_pairs();

        let invalid_pair = invalid_pairs
            .into_iter()
            .find(|pair| matches!(pair, yolo_io::PairingError::BothFilesMissing));

        assert!(valid_pair.is_none());
        assert!(invalid_pair.is_none());
    }

    #[rstest]
    fn test_pairing_with_mixed_case_extensions(
        image_data: ImageBuffer<Rgb<u8>, Vec<u8>>,
        mut create_yolo_project_config: YoloProjectConfig,
    ) {
        let filename = "mixed_ext";
        let this_test_directory = format!("{}/{}/", TEST_SANDBOX_DIR, filename);

        let image_file = PathBuf::from(format!("{}/testMiXeD.JpG", this_test_directory));
        create_image_file(&image_file, &image_data);

        let label_file = PathBuf::from(format!("{}/testMiXeD.TxT", this_test_directory));

        create_dir_and_write_file(&label_file, "0 0.5 0.5 0.5 0.5");

        create_yolo_project_config.source_paths.images = this_test_directory.clone();
        create_yolo_project_config.source_paths.labels = this_test_directory.clone();

        let project =
            YoloProject::new(&create_yolo_project_config).expect("Unable to create project");

        let valid_pairs = project.get_valid_pairs();
        let valid_pair = valid_pairs
            .into_iter()
            .find(|pair| pair.name == "testMiXeD");

        assert!(valid_pair.is_some());
    }
}
