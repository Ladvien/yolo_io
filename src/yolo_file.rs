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

pub struct YoloEntry {
    pub class: i32,
    pub x_center: f32,
    pub y_center: f32,
    pub width: f32,
    pub height: f32,
}

pub struct YoloFile {
    entries: Vec<YoloEntry>,
}

impl YoloFile {
    pub fn new() -> YoloFile {
        YoloFile {
            entries: Vec::new(),
        }
    }

    pub fn load(&mut self, path: &str) -> Result<(), Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split(" ").collect();
            if parts.len() != 5 {
                return Err("Invalid format".into());
            }

            let class = parts[0].parse::<i32>()?;
            let x_center = parts[1].parse::<f32>()?;
            let y_center = parts[2].parse::<f32>()?;
            let width = parts[3].parse::<f32>()?;
            let height = parts[4].parse::<f32>()?;

            if class < 0 || class > 79 {
                return Err("Invalid class id".into());
            }

            if x_center < 0.0 || x_center > 1.0 {
                return Err("Invalid x_center".into());
            }

            if y_center < 0.0 || y_center > 1.0 {
                return Err("Invalid y_center".into());
            }

            if width < 0.0 || width > 1.0 {
                return Err("Invalid width".into());
            }

            if height < 0.0 || height > 1.0 {
                return Err("Invalid height".into());
            }

            self.entries.push(YoloEntry {
                class,
                x_center,
                y_center,
                width,
                height,
            });
        }

        Ok(())
    }
}
