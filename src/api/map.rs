use rocket::http::Status;
use rocket::{Response, State};
use std::io;

use crate::core::responses::RawResponse;
use crate::core::AppState;
use crate::docs;
use crate::features::bot;
use crate::features::bot::handlers::live_map::MAP_BASE_IMAGE_BYTES;
use crate::router;

router!(MapsRouter, [live_map]);

#[utoipa::path(
    tag = docs::tags::MAP_RENDERING_TAG,
    responses(
        (status = 200, description = "Renders a map PNG image with the current bus position.", content_type = "image/png"),
        (status = 500, description = "Image failed to render")
    )
)]
#[get("/live")]
async fn live_map(state: &State<AppState>) -> RawResponse<'_> {
    let map_image = bot::handlers::live_map::live_map_handler(state).await;

    if map_image.is_none() {
        return Response::build()
            .header(rocket::http::ContentType::PNG)
            .sized_body(
                MAP_BASE_IMAGE_BYTES.len(),
                io::Cursor::new(*MAP_BASE_IMAGE_BYTES),
            )
            .status(Status::Ok)
            .finalize()
            .into();
    }

    let mut buffer = Vec::new();

    map_image
        .unwrap()
        .write_to(&mut io::Cursor::new(&mut buffer), image::ImageFormat::Png)
        .unwrap();

    Response::build()
        .header(rocket::http::ContentType::PNG)
        .sized_body(buffer.len(), io::Cursor::new(buffer))
        .status(Status::Ok)
        .finalize()
        .into()
}
