use rocket::Route;
use std::iter;

#[allow(dead_code)]
pub fn get_uptime() -> std::time::Duration {
    crate::startup.elapsed().unwrap()
}

/// For prepending a base route to a Vec<Route>
pub fn with_base(
    routes: Vec<Route>,
    base: &'static str,
) -> iter::Map<impl Iterator<Item = Route>, impl FnMut(Route) -> Route> {
    routes.into_iter().map(move |route| {
        let route = route.clone();
        route
            .map_base(|base_| format!("{}/{}", base, base_))
            .unwrap()
    })
}
