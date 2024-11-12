use rocket::http::Status;
use rocket::{Response, State};
use std::io;

use crate::core::responses::RawResponse;
use crate::core::AppState;
use crate::docs;
use crate::features::bot;
use crate::router;

router!(
    MapsRouter,
    [
        live_map,
        bus_stops,
        unmsm_entrances,
        unmsm_buildings,
        bus_path
    ]
);

const MAX_SIZE: u32 = 600;

// NOTE: All it's left for this to work is rendering the bus marker in the correct
//       position. See the map handler. Big respect if you manage to make it work.

#[utoipa::path(
    tag = docs::tags::MAP_RENDERING_TAG,
    description =
        "Renders a map PNG image with the current bus position.
        \nThanks to this, you don't even need to build a client to display the bus location
        on a map. Still in development, so the marker may not be accurate at all.
        <img src=\"/map/live\" width=300>",
    responses(
        (status = 200, description = "Map PNG image", content_type = "image/png"),
        (status = 500, description = "Image failed to render")
    )
)]
#[get("/live?<size>")]
async fn live_map(size: Option<u32>, state: &State<AppState>) -> RawResponse<'_> {
    let mut map_image = bot::handlers::live_map::live_map_handler(state).await;
    let size = size.map(|size| size.clamp(1, MAX_SIZE));

    let mut buffer = Vec::new();

    if size.is_some() {
        map_image = resize_image(map_image, size.unwrap());
    }

    map_image
        .write_to(&mut io::Cursor::new(&mut buffer), image::ImageFormat::Png)
        .unwrap();

    Response::build()
        .header(rocket::http::ContentType::PNG)
        .sized_body(buffer.len(), io::Cursor::new(buffer))
        .status(Status::Ok)
        .finalize()
        .into()
}

#[utoipa::path(
    tag = docs::tags::MAP_RENDERING_TAG,
    description =
        "Returns a [GeoJSON](https://geojson.org/) object containing all the UNMSM bus stops names,
        areas, and coordinates. Stops numbers are defined by the official UNMSM route map.
        \nClients may use this data to render the bus stops in a map. Always up to date.",
    responses(
        (status = 200, content_type = "application/json", example = json!({
            "type": "FeatureCollection",
            "features": [
                {
                    "type": "Feature",
                    "properties": {
                        "num": 9,
                        "name": "Paradero Sistemas"
                    },
                    "geometry": {
                        "type": "Polygon",
                        "coordinates": [[]]
                    }
                }
            ]
        }))
    )
)]
#[get("/bus_stops.json")]
async fn bus_stops() -> &'static str {
    include_str!("../features/bus_stops/assets/bus_stops.json")
}

#[utoipa::path(
    tag = docs::tags::MAP_RENDERING_TAG,
    description =
        "Returns a [GeoJSON](https://geojson.org/) object containing all the official UNMSM campus
        entrances names, numbers and coordinates.
        \nClients may use this data to render the campus entrances in a map. Always up to date.",
    responses(
        (status = 200, content_type = "application/json", example = json!({
            "type": "FeatureCollection",
            "features": [
                {
                    "type": "Feature",
                    "properties": {"number": 6},
                    "geometry": {
                        "coordinates": [-77.08342230621332, -12.054217441072467],
                        "type": "Point"
                    }
                },
            ]
        }))
    )
)]
#[get("/entrances.json")]
async fn unmsm_entrances() -> &'static str {
    include_str!("../../public/geojson/entrances.json")
}

#[utoipa::path(
    tag = docs::tags::MAP_RENDERING_TAG,
    description =
        "Returns a [GeoJSON](https://geojson.org/) object containing all the UNMSM buildings
        (such as faculties, departments, etc) names, areas and coordinates.
        \nClients may use this data to render all the buildings in a map. Always up to date.",
    responses(
        (status = 200, content_type = "application/json", example = json!({
            "type": "FeatureCollection",
            "features": [
                {
                    "type": "Feature",
                    "properties": {
                        "name": "Facultad de Ingeniería de Sistemas e Informática"
                    },
                    "geometry": {
                        "type": "Polygon",
                        "coordinates": [[]]
                    }
                }
            ]
        }))
    )
)]
#[get("/buildings.json")]
async fn unmsm_buildings() -> &'static str {
    include_str!("../../public/geojson/buildings.json")
}

#[utoipa::path(
    tag = docs::tags::MAP_RENDERING_TAG,
    description =
        "Returns a [GeoJSON](https://geojson.org/) object containing the path of the burrito in the
        UNMSM campus.
        \nClients may use this data to draw the burrito path in a map. Always up to date.",
    responses(
        (status = 200, content_type = "application/json", example = json!({
            "type": "Feature",
            "properties": {},
            "geometry": {
                "coordinates": [
                    [-77.07966519701365, -12.059621078738317]
                ],
                "type": "LineString"
            }
        }))
    )
)]
#[get("/bus_path.json")]
async fn bus_path() -> &'static str {
    include_str!("../../public/geojson/bus_path.json")
}

fn resize_image(image: image::DynamicImage, size: u32) -> image::DynamicImage {
    let height = size;
    if height == image.height() {
        return image;
    }
    let width = (image.width() as f64 / image.height() as f64 * height as f64) as u32;
    image.resize(width, height, image::imageops::FilterType::Triangle)
}
