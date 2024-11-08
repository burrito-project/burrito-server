#[macro_export]
macro_rules! router {
    ($name:ident, [$($route:ident),*]) => {
        #[derive(utoipa::OpenApi)]
        #[openapi(paths($($route),*))]
        pub struct $name;

        impl ApiRouter for $name {
            fn routes() -> Vec<Route> {
                routes![$($route),*]
            }
        }
    };
}
