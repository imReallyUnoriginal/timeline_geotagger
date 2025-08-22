use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::{error::Error, fs::File, path::Path};

use crate::line::{Line, LineBuilder, Point};

#[derive(Deserialize, Debug)]
struct FrequentPlace {
    #[serde(rename = "placeId")]
    _place_id: String,
    #[serde(rename = "placeLocation")]
    _place_location: String,
    #[serde(rename = "label")]
    _label: Option<String>,
}

#[derive(Deserialize, Debug)]
struct ModeDistribution {
    #[serde(rename = "mode")]
    _mode: String,
    #[serde(rename = "rate")]
    _rate: f64,
}

#[derive(Deserialize, Debug)]
struct FrequentTrip {
    #[serde(rename = "waypointIds")]
    _waypoint_ids: Vec<String>,
    #[serde(rename = "modeDistribution")]
    _mode_distribution: Vec<ModeDistribution>,
    #[serde(rename = "startTimeMinutes")]
    _start_time_minutes: u32,
    #[serde(rename = "endTimeMinutes")]
    _end_time_minutes: u32,
    #[serde(rename = "durationMinutes")]
    _duration_minutes: u32,
    #[serde(rename = "confidence")]
    _confidence: f64,
    #[serde(rename = "commuteDirection")]
    _commute_direction: String,
}

#[derive(Deserialize, Debug)]
struct ModeAffinity {
    #[serde(rename = "mode")]
    _mode: String,
    #[serde(rename = "affinity")]
    _affinity: f64,
}

#[derive(Deserialize, Debug)]
struct Persona {
    #[serde(rename = "travelModeAffinities")]
    _travel_mode_affinities: Vec<ModeAffinity>,
}

#[derive(Deserialize, Debug)]
struct UserLocationProfile {
    #[serde(rename = "frequentPlaces")]
    _frequent_places: Vec<FrequentPlace>,
    #[serde(rename = "frequentTrips")]
    _frequent_trips: Vec<FrequentTrip>,
    #[serde(rename = "persona")]
    _persona: Persona,
}

#[derive(Deserialize, Debug)]
struct TimelinePoint {
    point: String,
    time: String,
}

#[derive(Deserialize, Debug)]
struct LocationStruct {
    #[serde(rename = "latLng")]
    _lat_lng: String,
}

#[derive(Deserialize, Debug)]
struct ActivityCandidate {
    #[serde(rename = "type")]
    _activity_type: String,
    #[serde(rename = "probability")]
    _probability: f64,
}

#[derive(Deserialize, Debug)]
struct ActivitySegment {
    #[serde(rename = "start")]
    _start: LocationStruct,
    #[serde(rename = "end")]
    _end: LocationStruct,
    #[serde(rename = "distanceMeters")]
    _distance_meters: f64,
    #[serde(rename = "topCandidate")]
    _top_candidate: ActivityCandidate,
}

#[derive(Deserialize, Debug)]
struct PlaceCandidate {
    #[serde(rename = "placeId")]
    _place_id: String,
    #[serde(rename = "semanticType")]
    _semantic_type: String,
    #[serde(rename = "probability")]
    _probability: f64,
    #[serde(rename = "placeLocation")]
    _place_location: LocationStruct,
}

#[derive(Deserialize, Debug)]
struct VisitSegment {
    #[serde(rename = "hierarchyLevel")]
    _hierarchy_level: i32,
    #[serde(rename = "probability")]
    _probability: f64,
    #[serde(rename = "topCandidate")]
    _top_candidate: PlaceCandidate,
}

#[derive(Deserialize, Debug)]
struct Place {
    #[serde(rename = "placeId")]
    _place_id: String,
}

#[derive(Deserialize, Debug)]
struct PlaceIdentifier {
    #[serde(rename = "identifier")]
    _identifier: Place,
}

#[derive(Deserialize, Debug)]
struct Trip {
    #[serde(rename = "distanceFromOriginKms")]
    _distance_from_origin_kms: i32,
    #[serde(rename = "destinations")]
    _destinations: Vec<PlaceIdentifier>,
}

#[derive(Deserialize, Debug)]
struct Note {
    #[serde(rename = "note")]
    _note: String,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum TimelineMemory {
    Trip {
        #[serde(rename = "trip")]
        _trip: Trip,
    },
    Note {
        #[serde(rename = "note")]
        _note: Note,
    },
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum SemanticSegment {
    Path {
        #[serde(rename = "startTime")]
        _start_time: String,
        #[serde(rename = "endTime")]
        _end_time: String,
        #[serde(rename = "timelinePath")]
        timeline_path: Vec<TimelinePoint>,
    },
    Activity {
        #[serde(rename = "startTime")]
        _start_time: String,
        #[serde(rename = "endTime")]
        _end_time: String,
        #[serde(rename = "startTimeTimezoneUtcOffsetMinutes")]
        _start_time_timezone_utc_offset_minutes: i32,
        #[serde(rename = "endTimeTimezoneUtcOffsetMinutes")]
        _end_time_timezone_utc_offset_minutes: i32,
        #[serde(rename = "activity")]
        _activity: ActivitySegment,
    },
    PlaceVisit {
        #[serde(rename = "startTime")]
        _start_time: String,
        #[serde(rename = "endTime")]
        _end_time: String,
        #[serde(rename = "startTimeTimezoneUtcOffsetMinutes")]
        _start_time_timezone_utc_offset_minutes: i32,
        #[serde(rename = "endTimeTimezoneUtcOffsetMinutes")]
        _end_time_timezone_utc_offset_minutes: i32,
        #[serde(rename = "visit")]
        _visit: VisitSegment,
    },
    Memory {
        #[serde(rename = "startTime")]
        _start_time: String,
        #[serde(rename = "endTime")]
        _end_time: String,
        #[serde(rename = "timelineMemory")]
        _timeline_memory: TimelineMemory,
    },
}

#[derive(Deserialize, Debug)]
struct WifiDeviceRecord {
    #[serde(rename = "mac")]
    _mac: u64, // MAC address as a number
    #[serde(rename = "rawRssi")]
    _raw_rssi: i32, // Received Signal Strength Indicator
}

#[derive(Deserialize, Debug)]
struct ProbableActivity {
    #[serde(rename = "type")]
    _activity_type: String,
    #[serde(rename = "confidence")]
    _confidence: f64,
}

#[derive(Deserialize, Debug)]
enum RawSignal {
    #[serde(rename = "position")]
    Position {
        #[serde(rename = "LatLng")]
        lat_lng: String,
        #[serde(rename = "accuracyMeters")]
        _accuracy_meters: i32,
        #[serde(rename = "altitudeMeters")]
        altitude_meters: Option<f64>,
        #[serde(rename = "source")]
        _source: String,
        #[serde(rename = "timestamp")]
        timestamp: String,
        #[serde(rename = "speedMetersPerSecond")]
        _speed_meters_per_second: Option<f64>,
    },
    #[serde(rename = "wifiScan")]
    WifiScan {
        #[serde(rename = "deliveryTime")]
        _delivery_time: String,
        #[serde(rename = "devicesRecords")]
        _devices_records: Vec<WifiDeviceRecord>,
    },
    #[serde(rename = "activityRecord")]
    Activity {
        #[serde(rename = "probableActivities")]
        _probable_activities: Vec<ProbableActivity>,
        #[serde(rename = "timestamp")]
        _timestamp: String,
    },
}

#[derive(Deserialize, Debug)]
pub struct Timeline {
    #[serde(rename = "semanticSegments")]
    semantic_segments: Vec<SemanticSegment>,
    #[serde(rename = "rawSignals")]
    raw_signals: Vec<RawSignal>,
    #[serde(rename = "userLocationProfile")]
    _user_location_profile: UserLocationProfile,
}

impl Timeline {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        // Stolen from https://github.com/paritytech/substrate/pull/10137

        let file = File::open(path)?;
        // SAFETY: `mmap` is fundamentally unsafe since technically the file can change
        //         underneath us while it is mapped; in practice it's unlikely to be a problem
        let bytes = unsafe {
            memmap2::Mmap::map(&file).map_err(|e| format!("Error mmaping timeline file: {}", e))?
        };
        let timeline_data = serde_json::from_slice(&bytes)?;

        Ok(timeline_data)
    }

    pub fn get_point_at(&self, timestamp: &DateTime<Utc>) -> Result<Point, String> {
        let mut line = self.get_line_from_raw_signals(timestamp);

        if line.is_none() {
            line = self.get_line_from_semantic_segments(timestamp);
        }

        match line {
            Some(line) => line.get_point_at(timestamp),
            None => Err("No valid line found".into()),
        }
    }

    fn get_line_from_raw_signals(&self, timestamp: &DateTime<Utc>) -> Option<Line> {
        let mut line_builder = LineBuilder::new();

        for raw_signal in &self.raw_signals {
            let RawSignal::Position {
                lat_lng,
                altitude_meters,
                timestamp: raw_timestamp,
                _accuracy_meters: _,
                _source: _,
                _speed_meters_per_second: _,
            } = raw_signal
            else {
                continue; // Skip non-position signals
            };

            let point = Point::from_timeline(
                lat_lng,
                raw_timestamp,
                altitude_meters,
                timestamp,
            );

            if let Ok(point) = point {
                line_builder.add_point(point);
            }
        }

        line_builder.build()
    }

    fn get_line_from_semantic_segments(&self, timestamp: &DateTime<Utc>) -> Option<Line> {
        let mut line_builder = LineBuilder::new();

        for segment in &self.semantic_segments {
            let SemanticSegment::Path {
                _start_time: _,
                _end_time: _,
                timeline_path,
            } = segment
            else {
                continue; // Skip non-position signals
            };

            for point in timeline_path.iter() {
                let TimelinePoint {
                    point: lat_lng,
                    time: point_timestamp,
                } = point;

                let point = Point::from_timeline(
                    &lat_lng,
                    point_timestamp,
                    &None,
                    timestamp
                );

                if let Ok(point) = point {
                    line_builder.add_point(point);
                }
            }
        }

        line_builder.build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_path() {
        let path = "tests/basic_example.json";
        let result = Timeline::from_path(path);
        assert!(
            result.is_ok(),
            "Failed to parse takeout data: {:?}",
            result.err()
        );

        let data = result.unwrap();
        assert!(
            !data.semantic_segments.is_empty(),
            "Semantic Segments should not be empty"
        );
        assert!(
            !data.raw_signals.is_empty(),
            "Raw Signals should not be empty"
        );
    }

    #[test]
    fn test_get_line_from_raw_signals() {
        let path = "tests/basic_example.json";
        let data = Timeline::from_path(path).unwrap();

        let timestamp = DateTime::parse_from_rfc3339("2025-08-11T16:26:00.000+01:00")
            .unwrap()
            .with_timezone(&Utc);

        let line = data.get_line_from_raw_signals(&timestamp);
        assert!(line.is_some());
        let location = line.as_ref().unwrap();
        assert_eq!(location.start.lat, 54.7973628);
        assert_eq!(location.start.lng, -1.5921431);
        assert_eq!(location.start.altitude, Some(75.5999984741211));
        assert_eq!(location.start.relative_seconds, -10);
        assert_eq!(location.end.lat, 54.7973675);
        assert_eq!(location.end.lng, -1.592149);
        assert_eq!(location.end.altitude, Some(75.5999984741211));
        assert_eq!(location.end.relative_seconds, 39);
    }

    #[test]
    fn test_get_line_from_semantic_segments() {
        let path = "tests/basic_example.json";
        let data = Timeline::from_path(path).unwrap();

        let timestamp = DateTime::parse_from_rfc3339("2023-08-29T12:37:20.000+01:00")
            .unwrap()
            .with_timezone(&Utc);

        let line = data.get_line_from_semantic_segments(&timestamp);
        assert!(line.is_some());
        let line = line.unwrap();
        assert_eq!(line.start.lat, 50.1451596);
        assert_eq!(line.start.lng, 5.6022914);
        assert_eq!(line.start.altitude, None);
        assert_eq!(line.start.relative_seconds, -20);
        assert_eq!(line.end.lat, 50.1417194);
        assert_eq!(line.end.lng, 5.5951741);
        assert_eq!(line.end.altitude, None);
        assert_eq!(line.end.relative_seconds, 160);

        // Between 2 paths
        let timestamp = DateTime::parse_from_rfc3339("2023-08-29T13:02:00.000+01:00")
            .unwrap()
            .with_timezone(&Utc);

        let line = data.get_line_from_semantic_segments(&timestamp);
        assert!(line.is_some());
        let line = line.unwrap();
        assert_eq!(line.start.lat, 50.1351735);
        assert_eq!(line.start.lng, 5.5929213);
        assert_eq!(line.start.altitude, None);
        assert_eq!(line.start.relative_seconds, -600);
        assert_eq!(line.end.lat, 50.1173907);
        assert_eq!(line.end.lng, 5.62853);
        assert_eq!(line.end.altitude, None);
        assert_eq!(line.end.relative_seconds, 180);
    }

    #[test]
    fn test_get_point_at() {
        let path = "tests/basic_example.json";
        let data = Timeline::from_path(path).unwrap();

        // From raw signals
        let timestamp = DateTime::parse_from_rfc3339("2025-08-11T16:26:00.000+01:00")
            .unwrap()
            .with_timezone(&Utc);

        let point = data.get_point_at(&timestamp);
        assert!(point.is_ok());
        let point = point.unwrap();
        assert_eq!(point.lat, 54.797363759183675);
        assert_eq!(point.lng, -1.5921443040816325);
        assert_eq!(point.altitude, Some(75.5999984741211));
        assert_eq!(point.relative_seconds, 0);

        // From semantic segments
        let timestamp = DateTime::parse_from_rfc3339("2023-08-29T12:37:20.000+01:00")
            .unwrap()
            .with_timezone(&Utc);

        let point = data.get_point_at(&timestamp);
        assert!(point.is_ok());
        let point = point.unwrap();
        assert_eq!(point.lat, 50.144777355555554);
        assert_eq!(point.lng, 5.601500588888889);
        assert_eq!(point.altitude, None);
        assert_eq!(point.relative_seconds, 0);
    }
}
