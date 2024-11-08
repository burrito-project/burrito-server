use rocket::Route;

pub fn routes() -> Vec<Route> {
    routes![humans_txt]
}

#[get("/humans.txt")]
pub fn humans_txt() -> &'static str {
    include_str!("../../public/humans.txt")
}
