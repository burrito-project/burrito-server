use rocket::http::Status;
use rocket::{Response, Route, State};
use std::io;

use crate::features::bot;
use crate::features::bot::handlers::live_map::MAP_BASE_IMAGE_BYTES;
use crate::{core::responses::RawResponse, entities::AppState};

pub fn routes() -> Vec<Route> {
    routes![live_map]
}

#[get("/")]
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
