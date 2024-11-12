use rocket::http::Status;
use rocket::{Response, State};
use std::io;

use crate::core::responses::RawResponse;
use crate::core::AppState;
use crate::docs;
use crate::features::bot;
use crate::router;

router!(MapsRouter, [live_map]);

const MAX_SIZE: u32 = 600;

// NOTE: All it's left for this to work is rendering the bus marker in the correct
//       position. See the map handler. Big respect if you manage to make it work.

#[utoipa::path(
    tag = docs::tags::MAP_RENDERING_TAG,
    description =
        "Renders a map PNG image with the current bus position. Still in development, so
        the map marker may not be accurate at all. <img src=\"/maps/live\" width=300>",
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

fn resize_image(image: image::DynamicImage, size: u32) -> image::DynamicImage {
    let height = size;
    if height == image.height() {
        return image;
    }
    let width = (image.width() as f64 / image.height() as f64 * height as f64) as u32;
    image.resize(width, height, image::imageops::FilterType::Triangle)
}
