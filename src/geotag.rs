use std::{fs, path::Path};

use chrono::{NaiveDateTime, Timelike};
use chrono_tz::Tz;
use little_exif::{exif_tag::ExifTag, metadata::Metadata};

use crate::parse_takeout::TimelineData;

/// Convert decimal degrees to degrees, minutes, seconds
fn decimal_to_dms(decimal: f64) -> (u32, u32, f64) {
    let degrees = decimal.abs().floor() as u32;
    let minutes_float = (decimal.abs() - degrees as f64) * 60.0;
    let minutes = minutes_float.floor() as u32;
    let seconds = (minutes_float - minutes as f64) * 60.0;
    (degrees, minutes, seconds)
}

/// Get GPS latitude hemisphere reference (N/S)
fn get_latitude_ref(latitude: f64) -> String {
    if latitude >= 0.0 { "N".to_string() } else { "S".to_string() }
}

/// Get GPS longitude hemisphere reference (E/W)
fn get_longitude_ref(longitude: f64) -> String {
    if longitude >= 0.0 { "E".to_string() } else { "W".to_string() }
}

pub fn geotag_photos(
    timeline_data: &TimelineData,
    photos_path: &Path,
    photo_timezone: Tz
) -> Result<(), Box<dyn std::error::Error>> {
    let photos = fs::read_dir(photos_path)?
        .filter_map(Result::ok)
        .filter(|entry| {
            entry.path().is_file()
            && entry
                .path()
                .extension()
                .map_or(false, |ext| {
                    let string = ext.to_string_lossy().to_lowercase();
                    string == "jpg" || string == "jpeg" || string == "png"
                })
        })
        .collect::<Vec<_>>();

    if photos.is_empty() {
        return Err("No photos found in the specified directory".into());
    }

    for photo in photos {
        geotag_photo(timeline_data, &photo.path(), photo_timezone)?;
    }

    Ok(())
}

fn geotag_photo(
    timeline_data: &TimelineData,
    photo_path: &Path,
    photo_timezone: Tz
) -> Result<(), Box<dyn std::error::Error>> {
    let extension = photo_path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("");

    if !["jpg", "jpeg", "png"].contains(&extension.to_lowercase().as_str()) {
        println!("Skipping non-image file: {}", photo_path.display());
        return Ok(());
    }

    let metadata = Metadata::new_from_path(photo_path);

    if metadata.is_err() {
        return Err(format!(
            "Failed to read metadata for photo: {}",
            photo_path.display()
        )
        .into());
    }

    let mut metadata = metadata.unwrap();
    let photo_time = metadata
        .get_tag(&ExifTag::DateTimeOriginal(String::new()))
        .next();

    let Some(ExifTag::DateTimeOriginal(photo_time)) = photo_time else {
        return Err(format!(
            "Photo {} does not have DateTimeOriginal tag",
            photo_path.display()
        )
        .into());
    };

    let photo_time = NaiveDateTime::parse_from_str(&photo_time, "%Y:%m:%d %H:%M:%S")
        .map_err(|e| format!("Failed to parse photo time: {}", e))?
        .and_local_timezone(photo_timezone)
        .unwrap()
        .to_utc();

    let location = timeline_data.get_location_at(&photo_time).ok_or_else(|| {
        format!(
            "No location found for photo {} at time {}",
            photo_path.display(),
            photo_time
        )
    })?;

    // Convert decimal degrees to DMS format (required by EXIF GPS standard)
    // GPS coordinates must be stored as degrees, minutes, seconds in rational format
    let (lat_degrees, lat_minutes, lat_seconds) = decimal_to_dms(location.lat);
    let (lng_degrees, lng_minutes, lng_seconds) = decimal_to_dms(location.lng);

    // Set GPS coordinates in DMS format (3 rational numbers: degrees, minutes, seconds)
    metadata.set_tag(ExifTag::GPSLatitude(vec![
        lat_degrees.into(),
        lat_minutes.into(),
        lat_seconds.into(),
    ]));
    metadata.set_tag(ExifTag::GPSLatitudeRef(get_latitude_ref(location.lat)));
    
    metadata.set_tag(ExifTag::GPSLongitude(vec![
        lng_degrees.into(),
        lng_minutes.into(),
        lng_seconds.into(),
    ]));
    metadata.set_tag(ExifTag::GPSLongitudeRef(get_longitude_ref(location.lng)));
    if let Some(altitude) = location.altitude {
        metadata.set_tag(ExifTag::GPSAltitude(vec![altitude.abs().into()]));
        metadata.set_tag(ExifTag::GPSAltitudeRef(vec![if altitude >= 0.0 {
            0 // Above sea level
        } else {
            1 // Below sea level
        }]));
    }
    metadata.set_tag(ExifTag::GPSTimeStamp(vec![
        photo_time.hour().into(),
        photo_time.minute().into(),
        photo_time.second().into(),
    ]));
    metadata.set_tag(ExifTag::GPSDateStamp(photo_time.date_naive().format("%Y:%m:%d").to_string()));
    metadata.set_tag(ExifTag::GPSVersionID(vec![2, 2, 0, 0]));

    match metadata.write_to_file(photo_path) {
        Ok(()) => println!("Successfully wrote metadata to photo: {}", photo_path.display()),
        Err(e) => eprintln!("Failed to write metadata to photo: {}", e),
    }

    Ok(())
}
