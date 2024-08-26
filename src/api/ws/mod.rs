use rocket::Route;

mod status;

pub fn routes() -> Vec<Route> {
    status::routes()
        .iter()
        .map(|r| {
            r.clone()
                .map_base(|base| format!("{}{}", "/status", base))
                .unwrap()
        })
        .collect::<Vec<Route>>()
}
