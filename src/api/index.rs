use rocket::Route;
use serde::Serialize;
use serde_json::{json, Value};

pub fn routes() -> Vec<Route> {
    routes![help_index, humans_txt]
}

#[derive(Serialize)]
struct QueryParamDocs<'a, T> {
    name: &'a str,
    r#type: &'a str,
    default: T,
    description: &'a str,
}

#[derive(Serialize)]
struct RouteDocs<'a> {
    path: &'a str,
    description: &'a str,
    query_params: &'a [QueryParamDocs<'a, Value>],
}

#[get("/humans.txt")]
pub fn humans_txt() -> &'static str {
    include_str!("../../public/humans.txt")
}

#[get("/")]
pub fn help_index() -> serde_json::Value {
    let status_params = [QueryParamDocs {
        name: "count",
        r#type: "u32",
        default: json!(1),
        description: "The number of bus position records to return.",
    }];

    let routes = vec![
        RouteDocs {
            path: "/status",
            description: "Get the UNMSM transport bus status, positions, velocity, last stop, etc.",
            query_params: &status_params,
        },
        RouteDocs {
            path: "/ping",
            description: "API health check.",
            query_params: &[],
        },
        RouteDocs {
            path: "/help",
            description: "API documentation.",
            query_params: &[],
        },
    ];

    json!({
        "message": "Welcome to the UNMSM burrito tracker API",
        "routes": routes,
    })
}
