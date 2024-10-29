// mod common;

// #[cfg(test)]
// mod tests {

//     use std::path::PathBuf;

//     use crate::common::{
//         create_dir_and_write_file, create_image_file, create_yolo_project_config, image_data,
//         TEST_SANDBOX_DIR,
//     };
//     use image::{ImageBuffer, Rgb};
//     use rstest::rstest;
//     use yolo_io::{YoloProject, YoloProjectConfig};

//     #[rstest]
//     fn test_duplicates_found_when_duplicate_images_exist(
//         mut create_yolo_project_config: YoloProjectConfig,
//         image_data: ImageBuffer<Rgb<u8>, Vec<u8>>,
//     ) {
//         let filename = "dup_one";
//         let this_test_directory = format!("{}/{}/", TEST_SANDBOX_DIR, filename);

//         let image_file = PathBuf::from(format!("{}/test1.jpg", this_test_directory));
//         create_image_file(&image_file, &image_data);

//         let image_file_duplicate =
//             PathBuf::from(format!("{}/elsewhere/test2.jpg", this_test_directory));
//         create_image_file(&image_file_duplicate, &image_data);

//         let file1 = PathBuf::from(format!("{}/test1.txt", this_test_directory));
//         create_dir_and_write_file(&file1, "0 0.5 0.5 0.5 0.5");

//         create_yolo_project_config.source_paths.images = this_test_directory.clone();
//         create_yolo_project_config.source_paths.labels = this_test_directory.clone();

//         let project = YoloProject::new(&create_yolo_project_config);

//         assert!(false)
//     }
// }
