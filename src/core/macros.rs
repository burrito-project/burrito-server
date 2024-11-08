#[macro_export]
macro_rules! router {
    ($name:ident, [$($route:ident),*]) => {
        #[derive(utoipa::OpenApi)]
        #[openapi(paths($($route),*))]
        pub struct $name;

        impl $crate::routes::ApiRouter for $name {
            fn routes() -> Vec<rocket::Route> {
                routes![$($route),*]
            }
        }
    };
}
