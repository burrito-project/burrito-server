use image::DynamicImage;
use include_bytes_plus::include_bytes;
use lazy_static::lazy_static;

use crate::core::AppState;
use crate::features::bus_status;
use crate::features::bus_stops::schemas::LatLng;

lazy_static! {
    pub static ref MAP_BASE_IMAGE_BYTES: &'static [u8] =
        include_bytes!("src/features/bot/assets/map_base.jpg").as_slice();
    static ref MAP_BASE_IMAGE: image::DynamicImage =
        image::load_from_memory(*MAP_BASE_IMAGE_BYTES).unwrap();
    static ref MAP_MARKER_IMAGE: image::DynamicImage = {
        let bytes = include_bytes!("src/features/bot/assets/burrito_marker.png");
        image::load_from_memory(bytes.as_slice()).unwrap()
    };
}

// This bottom, top, left, right are relative to the original world map
const MAP_BOTTOM_RIGHT_COORDS: LatLng = LatLng::new(-12.061349549708325, -77.0779032447336);
const MAP_BOTTOM_LEFT_COORDS: LatLng = LatLng::new(-12.063045246181971, -77.08851784320025);
const MAP_TOP_RIGHT_COORDS: LatLng = LatLng::new(-12.049784581812744, -77.07985106789113);
const MAP_TOP_LEFT_COORDS: LatLng = LatLng::new(-12.051563821207822, -77.09050397949446);

/// Returns a live map image with the burrito's location
///
/// Returns none if the burrito is not locatable
pub async fn live_map_handler(state: &AppState) -> Option<DynamicImage> {
    let burrito_status = bus_status::handlers::get_burrito_status_handler(1, state).await;
    let pos = burrito_status.positions[0].clone();

    if !pos.sts.is_locatable() {
        return None;
    }

    let mut base_image: image::DynamicImage = MAP_BASE_IMAGE.clone();
    let marker: image::DynamicImage = MAP_MARKER_IMAGE.clone();

    let a = MAP_BOTTOM_LEFT_COORDS.lng - MAP_TOP_LEFT_COORDS.lng;
    let b = MAP_BOTTOM_RIGHT_COORDS.lat - MAP_BOTTOM_LEFT_COORDS.lat;
    let c = MAP_BOTTOM_RIGHT_COORDS.lng - MAP_TOP_RIGHT_COORDS.lng;
    let d = MAP_TOP_RIGHT_COORDS.lat - MAP_TOP_LEFT_COORDS.lat;

    let x = MAP_TOP_RIGHT_COORDS.lng - MAP_BOTTOM_LEFT_COORDS.lng;
    let y = MAP_TOP_LEFT_COORDS.lat - MAP_BOTTOM_RIGHT_COORDS.lat;

    let marker_pos_x =
        base_image.width() as f64 * ((pos.lg - MAP_TOP_LEFT_COORDS.lng) / (a + x + c));
    let marker_pos_y =
        base_image.height() as f64 * ((pos.lt - MAP_BOTTOM_LEFT_COORDS.lat) / (b + y + d));

    println!("🐢 marker_pos: {}x{}", marker_pos_x, marker_pos_y);
    image::imageops::overlay(
        &mut base_image,
        &marker,
        marker_pos_x as i64,
        marker_pos_y as i64,
    );

    Some(base_image)
}
