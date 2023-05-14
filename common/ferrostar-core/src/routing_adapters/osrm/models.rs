//! OSRM models from the API spec: http://project-osrm.org/docs/v5.5.1/api/
//!
//! Note that in some cases we optionally allow for extensions that have been made to the spec
//! by others which are now pseudo-standardized (ex: Mapbox). We omit some fields which are not
//! needed for navigation.

use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(transparent)]
pub struct Coordinate {
    tuple: (f64, f64),
}

impl Coordinate {
    pub fn latitude(&self) -> f64 {
        self.tuple.1
    }

    pub fn longitude(&self) -> f64 {
        self.tuple.0
    }
}

#[derive(Deserialize, Debug)]
pub struct RouteResponse {
    /// The response code.
    ///
    /// Ok indicates success. TODO: enumerate others?
    pub code: String,
    pub routes: Vec<Route>,
    pub waypoints: Vec<Waypoint>,
}

/// A route between two or more waypoints.
#[derive(Deserialize, Debug)]
pub struct Route {
    /// The distance traveled by the route, in meters.
    pub duration: f64,
    /// The estimated travel time, in seconds.
    pub distance: f64,
    /// The geometry of the route.
    ///
    /// NOTE: This library assumes that 1) an overview geometry will always be requested, and
    /// 2) that it will be a polyline (whether it is a polyline5 or polyline6 can be determined
    /// by the [crate::routing_adapters::RouteResponseParser]).
    pub geometry: String,
    /// The legs between the given waypoints.
    pub legs: Vec<RouteLeg>,
}

/// A route between exactly two waypoints.
#[derive(Deserialize, Debug)]
pub struct RouteLeg {
    pub annotation: Option<Annotation>,
    /// The estimated travel time, in seconds.
    pub duration: f64,
    /// The distance traveled this leg, in meters.
    pub distance: f64,
    /// A sequence of steps with turn-by-turn instructions.
    pub steps: Vec<RouteStep>,
}

/// An annotation of a route leg with fine-grained information about segments or nodes.
#[derive(Deserialize, Debug)]
pub struct Annotation {
    /// The duration between each pair of coordinates, in seconds.
    pub duration: Vec<f64>,
    /// The distance between each pair of coordinates, in meters.
    pub distance: Vec<f64>,

    /// The average speed used in the calculation between the two points in each pair of
    /// coordinates, in meters per second.
    ///
    /// NOTE: This annotation is not in the official spec, but is a common extension used by Mapbox
    /// and Valhalla.
    #[serde(default)]
    pub speed: Vec<f64>,
    /// The local posted speed limit between each pair of coordinates.
    ///
    /// NOTE: This annotation is not in the official spec, but is a common extension used by Mapbox
    /// and Valhalla.
    #[serde(default, rename = "maxspeed")]
    max_speed: Vec<MaxSpeed>,
}

/// The local posted speed limit between a pair of coordinates.
#[derive(Deserialize, Debug)]
pub struct MaxSpeed {
    // TODO
}

#[derive(Deserialize, Debug)]
pub struct RouteStep {
    /// The distance from the start of the current maneuver to the following step, in meters.
    pub distance: f64,
    /// The estimated travel time, in seconds.
    pub duration: f64,
    /// The (unsimplified) geometry of the route segment.
    ///
    /// NOTE: This library assumes that the geometry will always be a polyline.
    pub geometry: String,
    /// The name of the way along which travel proceeds.
    pub name: Option<String>,
    /// A reference number or code for the way (if one is available).
    #[serde(rename = "ref")]
    pub reference: Option<String>,
    /// A pronunciation hint for the name of the way.
    pub pronunciation: Option<String>,
    /// The mode of transportation.
    pub mode: Option<String>,
    /// The maneuver for this step
    pub maneuver: Option<StepManeuver>,
    /// TODO
    pub intersections: Vec<Intersections>,

    /// A list of exits (name or number), separated by semicolons.
    ///
    /// NOTE: This annotation is not in the official spec, but is a common extension used by Mapbox
    /// and Valhalla.
    pub exits: Option<String>,

    /// The side of the way on which traffic proceeds.
    ///
    /// NOTE: This annotation is not in the official spec, but is a common extension used by Mapbox
    /// and Valhalla.
    pub driving_side: Option<String>,
    // Mapbox and Valhalla extensions that might be useful later
    // pub rotary_name: Option<String>,
    // pub rotary_pronunciation: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct StepManeuver {
    /// The coordinate at which the maneuver takes place.
    pub location: Coordinate,
    /// The clockwise angle from true north to the direction of travel immediately *before*
    /// the maneuver.
    pub bearing_before: u16,
    /// The clockwise angle from true north to the direction of travel immediately *after*
    /// the maneuver.
    pub bearing_after: u16,
    /// A string indicating the type of maneuver.
    ///
    /// Note that even though there are `new name` and `notification` instructions, the
    /// `mode` and `name` (of the parent [RouteStep]) can change between *any* pair of instructions.
    /// They only offer a fallback in case there is nothing else to report.
    /// TODO: Model this as an enum. Note that new types may be introduced, and anything unknown to the client should be handled like a turn.
    #[serde(rename = "type")]
    pub maneuver_type: String,
    /// An optional string indicating the direction change of the maneuver.
    /// TODO: Model this as an enum.
    pub modifier: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Intersections {
    /// The location of the intersection
    pub location: Coordinate,
    /// A list of bearing values that are available at the intersection.
    ///
    /// These describe all available roads at the intersection.
    pub bearings: Vec<u16>,
    /// TODO
    pub classes: Vec<String>,
    /// A list of entry flags, corresponding 1:1 to the list of bearings.
    ///
    /// This value indicates whether the respective road could be entered on a valid route (not
    /// violating some restriction).
    pub entry: Vec<bool>,
    /// An index into the bearings/entry array used to calculate the bearing just before the turn.
    ///
    /// The clockwise angle from true north to the direction of travel immediately before the
    /// maneuver/passing the intersection.
    /// To get the bearing in the direction of driving, the bearing has to be rotated by
    /// 180 degrees. Not supplied for `depart` maneuvers.
    #[serde(rename = "in")]
    pub intersection_in: usize,
    /// An index into the bearings/entry array used to calculate the bearing just before the turn.
    ///
    /// The clockwise angle from true north to the direction of travel immediately after the
    /// maneuver/passing the intersection.  Not supplied for `arrive` maneuvers.
    #[serde(rename = "out")]
    pub intersection_out: usize,
    /// A list of turn [Lane]s available at the intersection (if info is available).
    ///
    /// Lanes are listed in left-to-right order.
    pub lanes: Vec<Lane>,
}

#[derive(Deserialize, Debug)]
pub struct Lane {
    /// An indication (ex: marking on the road, sign, etc.) for a turn lane.
    ///
    /// A lane may have multiple indications (ex: both straight and left),
    /// TODO: Turn this into an enum
    pub indications: Vec<String>,
    /// Whether the lane is a valid choice for the current maneuver
    pub valid: bool,
    // TODO: Mapbox and Valhalla extensions: `active` and `valid_indication`
}

#[derive(Deserialize, Debug)]
pub struct Waypoint {
    /// THe name of the street that the waypoint snapped to.
    pub name: String,
    /// The distance (in meters) between the snapped point and the original.
    pub distance: f64,
    /// The waypoint's location on the road network.
    pub location: Coordinate,
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Route
    // TODO: RouteLeg

    #[test]
    fn test_deserialize_annotation() {
        // Example from Mapbox's public docs, which include several annotations not supported at
        // the time of this writing.
        let data = r#"{
            "distance": [
                4.294596842089401,
                5.051172053200946,
                5.533254065167979,
                6.576513793849532,
                7.4449640160938015,
                8.468757534990829,
                15.202780313562714,
                7.056346577326572
            ],
            "duration": [
                1,
                1.2,
                2,
                1.6,
                1.8,
                2,
                3.6,
                1.7
            ],
            "speed": [
                4.3,
                4.2,
                2.8,
                4.1,
                4.1,
                4.2,
                4.2,
                4.2
            ],
            "congestion": [
                "low",
                "moderate",
                "moderate",
                "moderate",
                "heavy",
                "heavy",
                "heavy",
                "heavy"
            ],
            "maxspeed": [
              {
                "speed": 56,
                "unit": "km/h"
              },
              {
                "speed": 56,
                "unit": "km/h"
              },
              {
                "speed": 56,
                "unit": "km/h"
              },
              {
                "speed": 56,
                "unit": "km/h"
              },
              {
                "speed": 56,
                "unit": "km/h"
              },
              {
                "speed": 56,
                "unit": "km/h"
              },
              {
                "speed": 56,
                "unit": "km/h"
              },
              {
                "speed": 56,
                "unit": "km/h"
              }
            ]
        }"#;

        let annotation: Annotation =
            serde_json::from_str(data).expect("Failed to parse Annotation");

        assert_eq!(
            annotation.duration,
            vec![1.0, 1.2, 2.0, 1.6, 1.8, 2.0, 3.6, 1.7]
        );
        assert_eq!(
            annotation.distance,
            vec![
                4.294596842089401,
                5.051172053200946,
                5.533254065167979,
                6.576513793849532,
                7.4449640160938015,
                8.468757534990829,
                15.202780313562714,
                7.056346577326572
            ]
        );
        assert_eq!(
            annotation.speed,
            vec![4.3, 4.2, 2.8, 4.1, 4.1, 4.2, 4.2, 4.2]
        );
    }
}