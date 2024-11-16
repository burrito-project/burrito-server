use geo::{Centroid, Contains, Distance, Geodesic, Polygon};
use geojson::{FeatureCollection, GeoJson};
use lazy_static::lazy_static;
use std::time::SystemTime;

use crate::features::bus_stops::schemas::LatLng;

use super::schemas::BusStopInfo;

const BUS_STOPS_GEOJSON_STR: &str = include_str!("assets/bus_stops.json");

// global variable to store the parsed bus_stops geojson (read only)

lazy_static! {
    static ref BUS_STOPS: FeatureCollection = {
        let geo_json = BUS_STOPS_GEOJSON_STR.parse::<GeoJson>().unwrap();
        FeatureCollection::try_from(geo_json).unwrap()
    };
}

/// Useful trait to mainpulate nullable BusStopInfo
pub trait OptionalBuStopInfo {
    fn for_new_position(&self, new_pos: LatLng) -> Self;
}

impl OptionalBuStopInfo for Option<BusStopInfo> {
    fn for_new_position(&self, new_pos: LatLng) -> Self {
        self.as_ref().map(|pos| pos.for_new_position(new_pos))
    }
}

/// Transforms a GeoJSON feature into a geo::Polygon for interop with geo crate
fn feature_to_polygon(feature: &geojson::Feature) -> geo::Polygon {
    match feature.geometry.as_ref().map(|g| &g.value) {
        Some(geojson::Value::Polygon(p)) => {
            let points = p
                .first()
                .unwrap()
                .iter()
                .map(|pair| geo::Coord {
                    x: pair[0],
                    y: pair[1],
                })
                .collect::<Vec<_>>();
            Polygon::new(geo::LineString::new(points), vec![])
        }
        _ => unreachable!(),
    }
}

/// Given a point, it returns the bus stop located in that point, if some
pub fn get_bus_stop_for_point(lat: f64, lng: f64) -> Option<BusStopInfo> {
    BUS_STOPS.features.iter().find_map(|f| {
        let poly = feature_to_polygon(f);
        let props = f.properties.as_ref().unwrap();

        let name = props.get("name").unwrap().as_str().unwrap().to_string();
        let number = props.get("num").unwrap().as_i64().unwrap() as i32;

        if poly.contains(&geo::Point::new(lng, lat)) {
            let distance = Geodesic::distance(poly.centroid().unwrap(), geo::Point::new(lng, lat));

            return Some(BusStopInfo {
                name,
                number,
                has_reached: true,
                timestamp: SystemTime::now(),
                distance,
            });
        }
        None
    })
}

/// Bus stops numbering allows us to know what's the next bus stop given the current one.
/// Since a route is circular, there always exists a next bus stop
pub fn get_next_bus_stop(current: &BusStopInfo, current_pos: LatLng) -> BusStopInfo {
    let next_stop_num = match current.number {
        1..=8 => current.number + 1,
        9 => 1,
        _ => unreachable!(),
    };

    let next_stop = BUS_STOPS
        .features
        .iter()
        .find(|f| {
            let props = f.properties.as_ref().unwrap();
            props.get("num").unwrap().as_i64().unwrap() as i32 == next_stop_num
        })
        .unwrap();

    let props = next_stop.properties.as_ref().unwrap();
    let name = props.get("name").unwrap().as_str().unwrap().to_string();
    let number = props.get("num").unwrap().as_i64().unwrap() as i32;

    let poly = feature_to_polygon(next_stop);
    let distance = Geodesic::distance(
        poly.centroid().unwrap(),
        geo::Point::new(current_pos.lng, current_pos.lat),
    );

    BusStopInfo {
        name,
        number,
        has_reached: false,
        timestamp: SystemTime::now(),
        distance,
    }
}

/// Returns the distance to reach the bus stop, given the current position
/// Since a bus stop is an area, we calculate the distance to its centroid
///
/// The result should only be meaningful if the bus stop is not reached yet
pub fn get_distance_to_bus_stop(current: &BusStopInfo, current_post: LatLng) -> f64 {
    let poly = feature_to_polygon(
        BUS_STOPS
            .features
            .iter()
            .find(|f| {
                let props = f.properties.as_ref().unwrap();
                props.get("num").unwrap().as_i64().unwrap() as i32 == current.number
            })
            .unwrap(),
    );

    let centroid = poly.centroid().unwrap();

    Geodesic::distance(
        centroid,
        geo::Point::new(current_post.lng, current_post.lat),
    )
}
