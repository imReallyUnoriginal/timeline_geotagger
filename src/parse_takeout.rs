use chrono::{DateTime, Utc};
use serde::Deserialize;

use std::{error::Error, fs::File, path::Path};

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
        _trip: Trip
    },
    Note {
        #[serde(rename = "note")]
        _note: Note
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
pub struct TimelineData {
    #[serde(rename = "semanticSegments")]
    semantic_segments: Vec<SemanticSegment>,
    #[serde(rename = "rawSignals")]
    raw_signals: Vec<RawSignal>,
    #[serde(rename = "userLocationProfile")]
    _user_location_profile: UserLocationProfile,
}

#[derive(Debug)]
pub struct LocationData {
    pub lat: f64,
    pub lng: f64,
    pub altitude: Option<f64>,
    pub timestamp: DateTime<Utc>,
    pub relative_seconds: i64,
}

impl LocationData {
    pub fn parse_lat_lng(lat_lng: &str) -> Option<(f64, f64)> {
        let trimmed = lat_lng.trim().replace("°", "");
        let parts: Vec<&str> = trimmed.split(',').collect();
        if parts.len() == 2 {
            let lat = parts[0].trim().parse::<f64>().ok()?;
            let lng = parts[1].trim().parse::<f64>().ok()?;
            Some((lat, lng))
        } else {
            None
        }
    }
}

impl TimelineData {
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

    pub fn get_location_at(&self, timestamp: &DateTime<Utc>) -> Option<LocationData> {
        let mut locations = self.get_location_from_raw_signals(timestamp);

        if locations[0].is_none() || locations[1].is_none() {
            locations = self.get_location_from_semantic_segments(timestamp);
        }

        TimelineData::get_location_between_points(&locations, timestamp)
    }

    fn get_location_from_raw_signals(
        &self,
        timestamp: &DateTime<Utc>,
    ) -> [Option<LocationData>; 2] {
        let mut locations: [Option<LocationData>; 2] = [None, None];

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

            let raw_timestamp = match DateTime::parse_from_rfc3339(raw_timestamp) {
                Ok(dt) => dt.with_timezone(&Utc),
                Err(_) => continue, // Skip if timestamp parsing fails
            };

            let diff = (raw_timestamp - *timestamp).num_seconds();

            let lat_lng = LocationData::parse_lat_lng(&lat_lng);
            if lat_lng.is_none() {
                continue; // Skip if lat_lng parsing fails
            }

            let (lat, lng) = lat_lng.unwrap();

            if diff < 0
                && (locations[0].is_none()
                    || locations[0].as_ref().unwrap().relative_seconds < diff)
            {
                locations[0] = Some(LocationData {
                    lat,
                    lng,
                    altitude: altitude_meters.to_owned(),
                    timestamp: raw_timestamp,
                    relative_seconds: diff,
                });
            }

            if diff > 0
                && (locations[1].is_none()
                    || locations[1].as_ref().unwrap().relative_seconds > diff)
            {
                locations[1] = Some(LocationData {
                    lat,
                    lng,
                    altitude: altitude_meters.to_owned(),
                    timestamp: raw_timestamp,
                    relative_seconds: diff,
                });
            }
        }

        locations
    }

    fn get_location_from_semantic_segments(
        &self,
        timestamp: &DateTime<Utc>,
    ) -> [Option<LocationData>; 2] {
        let mut locations: [Option<LocationData>; 2] = [None, None];

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

                let point_timestamp = match DateTime::parse_from_rfc3339(point_timestamp.as_str()) {
                    Ok(dt) => dt.with_timezone(&Utc),
                    Err(_) => continue, // Skip if timestamp parsing fails
                };
    
                let diff = (point_timestamp - *timestamp).num_seconds();
    
                let lat_lng = LocationData::parse_lat_lng(&lat_lng);
                if lat_lng.is_none() {
                    continue; // Skip if lat_lng parsing fails
                }
    
                let (lat, lng) = lat_lng.unwrap();
    
                if diff < 0
                    && (locations[0].is_none()
                        || locations[0].as_ref().unwrap().relative_seconds < diff)
                {
                    locations[0] = Some(LocationData {
                        lat,
                        lng,
                        altitude: None,
                        timestamp: point_timestamp,
                        relative_seconds: diff,
                    });
                }
    
                if diff > 0
                    && (locations[1].is_none()
                        || locations[1].as_ref().unwrap().relative_seconds > diff)
                {
                    locations[1] = Some(LocationData {
                        lat,
                        lng,
                        altitude: None,
                        timestamp: point_timestamp,
                        relative_seconds: diff,
                    });
                }
            }
        }

        locations
    }

    fn get_location_between_points(
        locations: &[Option<LocationData>; 2],
        timestamp: &DateTime<Utc>,
    ) -> Option<LocationData> {
        if locations[0].is_none() || locations[1].is_none() {
            return None;
        }

        let past_location = locations[0].as_ref().unwrap();
        let future_location = locations[1].as_ref().unwrap();

        let total_duration = future_location.relative_seconds - past_location.relative_seconds;
        let elapsed_duration = -past_location.relative_seconds;
        let progress: f64 = elapsed_duration as f64 / total_duration as f64;

        let lat = past_location.lat + (future_location.lat - past_location.lat) * progress;
        let lng = past_location.lng + (future_location.lng - past_location.lng) * progress;
        let altitude = if past_location.altitude.is_some() && future_location.altitude.is_some() {
            Some(
                past_location.altitude.unwrap()
                    + (future_location.altitude.unwrap() - past_location.altitude.unwrap())
                        * progress,
            )
        } else {
            None
        };

        Some(LocationData {
            lat,
            lng,
            altitude,
            timestamp: timestamp.clone(),
            relative_seconds: 0,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_path() {
        let path = "tests/basic_example.json";
        let result = TimelineData::from_path(path);
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
    fn test_get_location_from_raw_signals() {
        let path = "tests/basic_example.json";
        let data = TimelineData::from_path(path).unwrap();

        let timestamp = DateTime::parse_from_rfc3339("2025-08-11T16:26:00.000+01:00")
            .unwrap()
            .with_timezone(&Utc);

        let locations = data.get_location_from_raw_signals(&timestamp);
        assert!(locations[0].is_some());
        let location = locations[0].as_ref().unwrap();
        assert_eq!(location.lat, 54.7973628);
        assert_eq!(location.lng, -1.5921431);
        assert_eq!(location.altitude, Some(75.5999984741211));
        assert_eq!(location.relative_seconds, -10);

        assert!(locations[1].is_some());
        let location = locations[1].as_ref().unwrap();
        assert_eq!(location.lat, 54.7973675);
        assert_eq!(location.lng, -1.592149);
        assert_eq!(location.altitude, Some(75.5999984741211));
        assert_eq!(location.relative_seconds, 39);
    }
    
    #[test]
    fn test_get_location_from_semantic_segments() {
        let path = "tests/basic_example.json";
        let data = TimelineData::from_path(path).unwrap();

        let timestamp = DateTime::parse_from_rfc3339("2023-08-29T12:37:20.000+01:00")
            .unwrap()
            .with_timezone(&Utc);

        let locations = data.get_location_from_semantic_segments(&timestamp);
        assert!(locations[0].is_some());
        let location = locations[0].as_ref().unwrap();
        assert_eq!(location.lat, 50.1451596);
        assert_eq!(location.lng, 5.6022914);
        assert_eq!(location.altitude, None);
        assert_eq!(location.relative_seconds, -20);

        assert!(locations[1].is_some());
        let location = locations[1].as_ref().unwrap();
        assert_eq!(location.lat, 50.1417194);
        assert_eq!(location.lng, 5.5951741);
        assert_eq!(location.altitude, None);
        assert_eq!(location.relative_seconds, 160);

        // Between 2 paths
        let timestamp = DateTime::parse_from_rfc3339("2023-08-29T13:02:00.000+01:00")
            .unwrap()
            .with_timezone(&Utc);

        let locations = data.get_location_from_semantic_segments(&timestamp);
        assert!(locations[0].is_some());
        let location = locations[0].as_ref().unwrap();
        assert_eq!(location.lat, 50.1351735);
        assert_eq!(location.lng, 5.5929213);
        assert_eq!(location.altitude, None);
        assert_eq!(location.relative_seconds, -600);

        assert!(locations[1].is_some());
        let location = locations[1].as_ref().unwrap();
        assert_eq!(location.lat, 50.1173907);
        assert_eq!(location.lng, 5.62853);
        assert_eq!(location.altitude, None);
        assert_eq!(location.relative_seconds, 180);
    }

    #[test]
    fn test_get_location_between_points() {
        let locations = [
            Some(LocationData {
                lat: 55.0000000,
                lng: -1.5000000,
                altitude: Some(75.0000000000000),
                timestamp: DateTime::parse_from_rfc3339("2025-07-11T16:20:00.000+01:00")
                    .unwrap()
                    .with_timezone(&Utc),
                relative_seconds: -60,
            }),
            Some(LocationData {
                lat: 57.0000000,
                lng: -2.0000000,
                altitude: Some(76.0000000000000),
                timestamp: DateTime::parse_from_rfc3339("2025-07-11T16:25:00.000+01:00")
                    .unwrap()
                    .with_timezone(&Utc),
                relative_seconds: 240,
            }),
        ];

        let timestamp = DateTime::parse_from_rfc3339("2025-07-11T16:21:00.000+01:00")
            .unwrap()
            .with_timezone(&Utc);

        let location = TimelineData::get_location_between_points(&locations, &timestamp);

        assert!(location.is_some());
        let location = location.unwrap();
        assert_eq!(location.lat, 55.4000000);
        assert_eq!(location.lng, -1.6000000);
        assert_eq!(location.altitude, Some(75.2000000000000));
        assert_eq!(location.relative_seconds, 0);
    }

    #[test]
    fn test_get_location_at() {
        let path = "tests/basic_example.json";
        let data = TimelineData::from_path(path).unwrap();

        // From raw signals
        let timestamp = DateTime::parse_from_rfc3339("2025-08-11T16:26:00.000+01:00")
            .unwrap()
            .with_timezone(&Utc);

        let location = data.get_location_at(&timestamp);
        assert!(location.is_some());
        let location = location.unwrap();
        assert_eq!(location.lat, 54.797363759183675);
        assert_eq!(location.lng, -1.5921443040816325);
        assert_eq!(location.altitude, Some(75.5999984741211));
        assert_eq!(location.relative_seconds, 0);

        // From semantic segments
        let timestamp = DateTime::parse_from_rfc3339("2023-08-29T12:37:20.000+01:00")
            .unwrap()
            .with_timezone(&Utc);

        let location = data.get_location_at(&timestamp);
        assert!(location.is_some());
        let location = location.unwrap();
        assert_eq!(location.lat, 50.144777355555554);
        assert_eq!(location.lng, 5.601500588888889);
        assert_eq!(location.altitude, None);
        assert_eq!(location.relative_seconds, 0);
    }
}
