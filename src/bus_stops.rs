use std::time::SystemTime;
use geo::{Centroid, Contains, GeodesicDistance, Polygon};
use geojson::{FeatureCollection, GeoJson};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

pub struct LatLng {
    pub lat: f64,
    pub lng: f64,
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

fn feature_to_polygon(feature: &geojson::Feature) -> geo::Polygon {
    match feature.geometry.as_ref().map(|g| &g.value) {
        Some(geojson::Value::Polygon(p)) => {
            let points = p.first().unwrap().iter().map(|pair| geo::Coord { x: pair[0], y: pair[1] }).collect::<Vec<_>>();
            Polygon::new(geo::LineString::new(points), vec![])
        },
        _ => unreachable!(),
    }
}

// Given a point, it returns the bus stop (paradero) located in that point, if some.
pub fn get_bus_stop_for_point(lat: f64, lng: f64) -> Option<BusStopInfo> {
    BUS_STOPS.features.iter().find_map(|f| {
        let poly = feature_to_polygon(f);

        let name = f.properties.as_ref().unwrap().get("name").unwrap().as_str().unwrap().to_string();
        let number = f.properties.as_ref().unwrap().get("num").unwrap().as_i64().unwrap() as i32;

        if poly.contains(&geo::Point::new(lng, lat)) {
            let distance = poly.centroid().unwrap().geodesic_distance(&geo::Point::new(lng, lat));

            Some(BusStopInfo { name, number, has_reached: true, timestamp: SystemTime::now(), distance })

        } else {
            None
        }
    })
}

pub fn get_next_bus_stop(current: &BusStopInfo, current_post: LatLng) -> BusStopInfo {
    let next = match current.number {
        1..=8 => current.number + 1,
        9 => 1,
        _ => unreachable!(),
    };

    let next_stop = BUS_STOPS.features.iter().find(|f| {
        f.properties.as_ref().unwrap().get("num").unwrap().as_i64().unwrap() as i32 == next
    }).unwrap();
    let name = next_stop.properties.as_ref().unwrap().get("name").unwrap().as_str().unwrap().to_string();
    let number = next_stop.properties.as_ref().unwrap().get("num").unwrap().as_i64().unwrap() as i32;

    let poly = feature_to_polygon(next_stop);
    let distance = poly.centroid().unwrap().geodesic_distance(&geo::Point::new(current_post.lng, current_post.lat));

    BusStopInfo { name, number, has_reached: false, timestamp: SystemTime::now(), distance }
}

pub fn get_distance_to_bus_stop(current: &BusStopInfo, current_post: LatLng) -> f64 {
    let poly = feature_to_polygon(BUS_STOPS.features.iter().find(|f| {
        f.properties.as_ref().unwrap().get("num").unwrap().as_i64().unwrap() as i32 == current.number
    }).unwrap());

    let centroid = poly.centroid().unwrap();

    centroid.geodesic_distance(&geo::Point::new(current_post.lng, current_post.lat))
}
