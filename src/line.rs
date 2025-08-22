use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Point {
    pub lat: f64,
    pub lng: f64,
    pub altitude: Option<f64>,
    pub timestamp: DateTime<Utc>,
    pub relative_seconds: i64,
}

impl Point {
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

pub struct Line {
    pub start: Point,
    pub end: Point,
}

impl Line {
    pub fn new(start: Point, end: Point) -> Self {
        Line { start, end }
    }

    pub fn get_point_at(
        &self,
        timestamp: &DateTime<Utc>,
    ) -> Result<Point, String> {

        if self.start.relative_seconds == self.end.relative_seconds {
            return Ok(self.start.clone());
        }

        if timestamp < &self.start.timestamp || timestamp > &self.end.timestamp {
            return Err("Timestamp is out of bounds".into());
        }

        let total_duration = self.end.relative_seconds - self.start.relative_seconds;
        let elapsed_duration = -self.start.relative_seconds;
        let progress: f64 = elapsed_duration as f64 / total_duration as f64;

        let lat = self.start.lat + (self.end.lat - self.start.lat) * progress;
        let lng = self.start.lng + (self.end.lng - self.start.lng) * progress;
        let altitude = if self.start.altitude.is_some() && self.end.altitude.is_some() {
            Some(
                self.start.altitude.unwrap()
                    + (self.end.altitude.unwrap() - self.start.altitude.unwrap())
                        * progress,
            )
        } else {
            None
        };

        Ok(Point {
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
    fn test_get_point_at() {
        let line = Line::new(
            Point {
                lat: 55.0000000,
                lng: -1.5000000,
                altitude: Some(75.0000000000000),
                timestamp: DateTime::parse_from_rfc3339("2025-07-11T16:20:00.000+01:00")
                    .unwrap()
                    .with_timezone(&Utc),
                relative_seconds: -60,
            },
            Point {
                lat: 57.0000000,
                lng: -2.0000000,
                altitude: Some(76.0000000000000),
                timestamp: DateTime::parse_from_rfc3339("2025-07-11T16:25:00.000+01:00")
                    .unwrap()
                    .with_timezone(&Utc),
                relative_seconds: 240,
            }
        );

        let timestamp = DateTime::parse_from_rfc3339("2025-07-11T16:21:00.000+01:00")
            .unwrap()
            .with_timezone(&Utc);

        let location = line.get_point_at(&timestamp);

        assert!(location.is_ok());
        let location = location.unwrap();
        assert_eq!(location.lat, 55.4000000);
        assert_eq!(location.lng, -1.6000000);
        assert_eq!(location.altitude, Some(75.2000000000000));
        assert_eq!(location.relative_seconds, 0);
    }
}

