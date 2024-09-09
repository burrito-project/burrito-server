use crate::features::cdn::providers::cloudinary::image_upload::CloudinaryImageService;

use super::errors::UploadError;

#[async_trait]
pub trait ProvideImageService {
    /// Uploads an image to a CDN and returns the URL
    async fn upload_image(base64_data: String, path: &'static str) -> Result<String, UploadError>;
}

pub struct ImageService;

#[async_trait]
impl ProvideImageService for ImageService {
    async fn upload_image(base64_data: String, path: &'static str) -> Result<String, UploadError> {
        CloudinaryImageService::upload_image(base64_data, path).await
    }
}
