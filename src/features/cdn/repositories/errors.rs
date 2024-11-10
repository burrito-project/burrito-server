use strum_macros::Display;

#[derive(Debug, thiserror::Error, Display)]
#[allow(dead_code)]
pub enum UploadError {
    InvalidBase64,
    TooLarge,
    ProviderError,
    Other,
}
