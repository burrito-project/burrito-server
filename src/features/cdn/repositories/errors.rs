#[derive(Debug)]
#[allow(dead_code)]
pub enum UploadError {
    InvalidBase64,
    TooLarge,
    ProviderError,
    Other,
}
