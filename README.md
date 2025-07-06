# yolo_io
A Rust library for loading, validating, and exporting YOLO project files.

## Work in Progress
This crate is very much a work-in-progress.  Features outlined may not be completed, working properly, or even listed.

## Features
1. Automatic pairing based on filename
   1. Pairing is simple; if three files exist, one image, three labels, it will list one valid pair and two errors.  The valid pair will not be flagged as having possible other pairings.
2. Flags incomplete pairs
3. Checks for corrupt label files:
   1. Check for empty file
   2. Check for corrupted format
   3. Check if duplicates exist in the same file.
   4. Check if invalid class ids exist
   5. TODO: Compare labels in duplicate label files.  What if they are different?
4. Export YOLO project
   1. Unlike RectLabel, let's make it so there isn't other types of imports.  We import a YOLO project, we export a YOLO project.  Don't create a separate folder for annotations.
5. Data quality validation report
   1. Exports a list of all labels with issues.


## Reads
- [YOLO Format](https://docs.ultralytics.com/yolov5/tutorials/train_custom_data/#21-create-datasetyaml)

## Configuration

`yolo_io` expects a YAML configuration file when building a project. The
`type` field inside this file denotes the project format. Today only the
`"yolo"` type is recognized, but this key remains so other formats can be
supported later.
