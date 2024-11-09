#[macro_export]
macro_rules! router {
    ($name:ident, [$($routes:ident),*]) => {
        #[derive(utoipa::OpenApi)]
        #[openapi(paths($($routes),*))]
        pub struct $name;

        impl $crate::routes::ApiRouter for $name {
            fn routes() -> Vec<rocket::Route> {
                routes![$($routes),*]
            }
        }
    };
}
