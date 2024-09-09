#[derive(Debug)]
pub enum UploadError {
    InvalidBase64,
    TooLarge,
    ProviderError,
    Other,
}
