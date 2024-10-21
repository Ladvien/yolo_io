#[cfg(test)]
mod tests {
    use std::{
        fs,
        path::{Path, PathBuf},
    };

    use image::{ImageBuffer, Rgb};
    use rstest::{fixture, rstest};
    use yolo_io::{YoloProject, YoloProjectConfig};

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
    fn test_project_validation_flags_empty_label_file() {}
}
