use yolo_io::*;

fn main() {
    let images_path = "examples/images";
    let labels_path = "examples/labels";
    let config = YoloProjectConfig::new("examples/config.yaml").unwrap();
    let project = YoloProject::new(&config);

    // let report = YoloDataQualityReport::generate(project.clone().unwrap());

    // match report {
    //     Some(report) => {
    //         let mut file = fs::File::create("report.json").expect("Unable to create file");
    //         file.write_all(report.as_bytes())
    //             .expect("Unable to write data to file");
    //     }
    //     None => todo!(),
    // }
}
