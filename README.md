# yolo_io
A Rust library for loading, validating, and exporting YOLO project files.


## Features
1. Automatic pairing based on filename
   1. Pairing is simple; if three files exist, one image, three labels, it will list one valid pair and two errors.  The valid pair will not be flagged as having possible other pairings.
2. Flags incomplete pairs
3. Checks for corrupt label files:
   1. Check for empty file
   2. Check for corrupted format
   3. Check if duplicates exist in the same file.
   4. Check if invalid class ids exist
4. Export YOLO project
   1. Unlike RectLabel, let's make it so there isn't other types of imports.  We import a YOLO project, we export a YOLO project.  Don't create a separate folder for annotations.
5. Data quality validation report
   1. Exports a list of all labels with issues.


## Reads
- [YOLO Format](https://yolov8.org/yolov8-label-format/)