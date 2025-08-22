pub mod file_system_autocomplete;
pub mod timezone_autocomplete;
pub mod line;
pub mod geotag;
pub mod parse_takeout;

use crate::{
    file_system_autocomplete::FileSystemAutocomplete,
    timezone_autocomplete::TimezoneAutocomplete,
    parse_takeout::TimelineData
};
use chrono_tz::{Tz};
use inquire::{validator::Validation, Text};
use std::{path::Path, str::FromStr};

fn main() {
    println!("----- GOOGLE MAPS GEOTAGGER -----");

    let timeline_path = Text::new("Path to Timeline.json file:")
        .with_autocomplete(FileSystemAutocomplete::files())
        .with_validator(|input: &str| {
            if input.is_empty() {
                Ok(Validation::Valid)
            } else if Path::new(input).exists() && Path::new(input).is_file() {
                Ok(Validation::Valid)
            } else {
                Ok(Validation::Invalid("File does not exist".into()))
            }
        })
        .prompt()
        .expect("Failed to read input");

    let timeline_path = Path::new(&timeline_path);

    println!("Parsing timeline file: {}", timeline_path.display());

    let parsed_json = match TimelineData::from_path(timeline_path) {
        Ok(json) => json,
        Err(e) => {
            eprintln!("Error parsing timeline file: {}", e);
            return;
        }
    };

    let photos_path = Text::new("Path to your photos directory:")
        .with_autocomplete(FileSystemAutocomplete::directories())
        .with_validator(|input: &str| {
            if input.is_empty() {
                Ok(Validation::Valid)
            } else if Path::new(input).exists() && Path::new(input).is_dir() {
                Ok(Validation::Valid)
            } else {
                Ok(Validation::Invalid("Directory does not exist".into()))
            }
        })
        .prompt()
        .expect("Failed to read input");

    let photos_path = Path::new(&photos_path);

    println!("Using photos directory: {}", photos_path.display());

    let photo_timezone = Text::new("What timezone were the photos taken in?")
        .with_autocomplete(TimezoneAutocomplete)
        .with_validator(|input: &str| {
            if input.is_empty() {
                Ok(Validation::Valid)
            } else if Tz::from_str(input).is_ok() {
                Ok(Validation::Valid)
            } else {
                Ok(Validation::Invalid("Invalid timezone".into()))
            }
        })
        .prompt()
        .expect("Failed to read input");

    let photo_timezone = Tz::from_str(&photo_timezone).expect("Failed to parse timezone");

    match geotag::geotag_photos(&parsed_json, photos_path, photo_timezone) {
        Ok(_) => println!("Geotagging completed successfully!"),
        Err(e) => eprintln!("Error geotagging photos: {}", e),
    }
}
