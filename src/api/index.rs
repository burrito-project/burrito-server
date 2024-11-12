use crate::{docs, router};

router!(IndexRouter, [humans_txt, robots_txt]);

#[utoipa::path(
    path = "humans.txt",
    tag = docs::tags::MISC_TAG,
    description =
        "The humans behind this project, following the [humans.txt](https://humanstxt.org/) format.",
)]
#[get("/humans.txt")]
pub fn humans_txt() -> &'static str {
    include_str!("../../public/humans.txt")
}

#[utoipa::path(
    path = "robots.txt",
    tag = docs::tags::MISC_TAG,
    description = "Crawling instructions for search engines and hackers.",
)]
#[get("/robots.txt")]
pub fn robots_txt() -> &'static str {
    include_str!("../../public/robots.txt")
}
