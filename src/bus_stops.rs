use geo::{Centroid, Contains, GeodesicDistance, Polygon};
use geojson::{FeatureCollection, GeoJson};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

pub struct LatLng {
    pub lat: f64,
    pub lng: f64,
}

impl LatLng {
    pub const fn new(lat: f64, lng: f64) -> Self {
        LatLng { lat, lng }
    }
}

const BUS_STOPS_GEOJSON_STR: &str = include_str!("../static/geojson/bus_stops.json");

// global variable to store the parsed geojson (read only)

lazy_static! {
    static ref BUS_STOPS: FeatureCollection = {
        let geo_json = BUS_STOPS_GEOJSON_STR.parse::<GeoJson>().unwrap();
        FeatureCollection::try_from(geo_json).unwrap()
    };
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BusStopInfo {
    pub name: String,
    pub number: i32,
    pub has_reached: bool,
    pub timestamp: SystemTime,
    pub distance: f64,
}

impl BusStopInfo {
    pub fn for_new_position(&self, new_pos: LatLng) -> Self {
        // If the last state is marked as reached, then we already passed it
        // and the bus is on its way to the next stop
        if self.has_reached {
            get_next_bus_stop(self, new_pos)
        }
        // And if not, we just update the distance to reach the bus stop
        else {
            let new_distance = get_distance_to_bus_stop(self, new_pos);
            BusStopInfo {
                distance: new_distance,
                has_reached: self.has_reached,
                name: self.name.to_owned(),
                number: self.number,
                timestamp: std::time::SystemTime::now(),
            }
        }
    }
}

pub trait OptionalBuStopInfo {
    fn for_new_position(&self, new_pos: LatLng) -> Self;
}

impl OptionalBuStopInfo for Option<BusStopInfo> {
    fn for_new_position(&self, new_pos: LatLng) -> Self {
        self.as_ref().map(|pos| pos.for_new_position(new_pos))
    }
}

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

// Given a point, it returns the bus stop (paradero) located in that point, if some.
pub fn get_bus_stop_for_point(lat: f64, lng: f64) -> Option<BusStopInfo> {
    BUS_STOPS.features.iter().find_map(|f| {
        let poly = feature_to_polygon(f);
        let props = f.properties.as_ref().unwrap();

        let name = props.get("name").unwrap().as_str().unwrap().to_string();
        let number = props.get("num").unwrap().as_i64().unwrap() as i32;

        if poly.contains(&geo::Point::new(lng, lat)) {
            let distance = poly
                .centroid()
                .unwrap()
                .geodesic_distance(&geo::Point::new(lng, lat));

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
    let distance = poly
        .centroid()
        .unwrap()
        .geodesic_distance(&geo::Point::new(current_pos.lng, current_pos.lat));

    BusStopInfo {
        name,
        number,
        has_reached: false,
        timestamp: SystemTime::now(),
        distance,
    }
}

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

    centroid.geodesic_distance(&geo::Point::new(current_post.lng, current_post.lat))
}
