use crate::features::cdn;

use super::entities::CloudinaryResponse;

use reqwest::Client;
use sha1::{Digest, Sha1};
use std::collections::HashMap;

enum ParamValue {
    Str(String),
    Int(i64),
}

pub struct CloudinaryImageService;

#[async_trait]
impl cdn::repositories::ProvideImageService for CloudinaryImageService {
    async fn upload_image(
        base64_data: String,
        path: &'static str,
    ) -> Result<String, cdn::errors::UploadError> {
        match CloudinaryImageService::upload_to_cloudinary(base64_data, path).await {
            Ok(response) => Ok(response.secure_url),
            Err(err) => Err(err),
        }
    }
}

impl CloudinaryImageService {
    /// Uploads an image to Cloudinary using the Upload API
    ///
    /// See <https://cloudinary.com/documentation/image_upload_api_reference> for more information
    async fn upload_to_cloudinary(
        base64_image: String,
        path: &'static str,
    ) -> Result<CloudinaryResponse, cdn::errors::UploadError> {
        let client = Client::new();

        let timestamp = chrono::Utc::now().timestamp();

        // Include only public_id and timestamp in the signature
        let mut params = HashMap::with_capacity(2);
        params.insert("folder", ParamValue::Str(path.to_owned()));
        params.insert("timestamp", ParamValue::Int(timestamp));

        let signature = CloudinaryImageService::generate_signature(params);

        let form = reqwest::multipart::Form::new()
            .text("folder", path)
            .text("timestamp", timestamp.to_string())
            .text("signature", signature)
            .text("api_key", crate::env::CLOUDINARY_API_KEY.to_owned())
            .text("file", base64_image);

        let res = client
            .post(format!(
                "https://api.cloudinary.com/v1_1/{}/image/upload",
                *crate::env::CLOUDINARY_CLOUD_NAME
            ))
            .multipart(form)
            .send()
            .await
            .map_err(|e| {
                // TODO: logging
                eprintln!("Error uploading image to Cloudinary: {:?}", e);
                cdn::errors::UploadError::ProviderError
            })?;

        let result = res.text().await.map_err(|e| {
            eprintln!("Error reading Cloudinary response: {:?}", e);

            cdn::errors::UploadError::ProviderError
        })?;

        let cloudinary_response: CloudinaryResponse =
            serde_json::from_str(&result).map_err(|e| {
                eprintln!("Error parsing Cloudinary response: {:?}", e);

                cdn::errors::UploadError::ProviderError
            })?;

        Ok(cloudinary_response)
    }

    /// Signs the request multipart params, as required by Cloudinary
    ///
    /// See <https://cloudinary.com/documentation/authentication_signatures> for more information
    fn generate_signature(params: HashMap<&str, ParamValue>) -> String {
        // Step 1: Sort the parameters by keys and concatenate them
        let mut sorted_keys: Vec<&&str> = params.keys().collect();
        sorted_keys.sort();

        let mut sorted_params = String::new();

        for key in sorted_keys {
            if !sorted_params.is_empty() {
                sorted_params.push('&');
            }
            let value = match &params[key] {
                ParamValue::Str(s) => s.clone(),
                ParamValue::Int(i) => i.to_string(),
            };
            sorted_params.push_str(&format!("{}={}", key, value));
        }

        // Step 2: Concatenate the sorted parameters and the API secret
        let string_to_sign = format!("{}{}", sorted_params, *crate::env::CLOUDINARY_API_SECRET);

        // Step 3: Generate an SHA-1 hash of the concatenated string
        let mut hasher = Sha1::new();
        hasher.update(string_to_sign.as_bytes());

        // Step 4: Return the hex-encoded result
        hex::encode(hasher.finalize())
    }
}
