use std::io::Cursor;

#[cfg(feature = "image")]
use image::{DynamicImage, ImageReader};

use reqwest::Client;
use serde::Serialize;

use crate::error::*;

#[cfg(feature = "ril")]
use ril::{Image, Rgb, ImageFormat};

/// A struct containing information necessary to request images. Use [`HFClient::new`] when constructing this.
#[derive(Clone, Debug)]
pub struct HFClient {
    pub token: String,
    pub url: String,
    pub request_client: Client,
}

impl HFClient {
    pub fn new(token: impl Into<String>, url: String) -> Self {
        let token = token.into();

        Self {
            token,
            url,
            request_client: Client::new(),
        }
    }

    /// Requests and decodes an image from the Inference API.
    #[cfg(feature = "image")]
    pub async fn request_inference(&self, payload: InferencePayload) -> Result<DynamicImage> {
        let request = self.request_client.post(&self.url)
            .bearer_auth(&self.token)
            .body(serde_json::to_string(&payload)?)
            .build()?;

        let response = self.request_client.execute(request)
            .await?
            .error_for_status()?;

        let raw = response.bytes().await?;
        let image = ImageReader::new(Cursor::new(raw))
            .with_guessed_format()?
            .decode()?;
        Ok(image)
    }

    /// Requests and decodes an image from the Inference API.
    #[cfg(feature = "ril")]
    pub async fn request_inference(&self, payload: InferencePayload) -> Result<Image<Rgb>> {
        let request = self.request_client.post(&self.url)
            .bearer_auth(&self.token)
            .body(serde_json::to_string(&payload)?)
            .build()?;

        let response = self.request_client.execute(request)
            .await?
            .error_for_status()?;

        let raw = response.bytes().await?;
        let image = Image::from_reader(ImageFormat::Png, Cursor::new(raw))?;
        Ok(image)
    }
}

/// The data that will be sent to the Inference API.
#[derive(Serialize, Default, Clone, Debug)]
pub struct InferencePayload {
    #[serde(rename = "inputs")]
    pub prompt: String,

    pub parameters: Option<TTIParams>,

    pub use_cache: Option<bool>,
    pub wait_for_model: Option<bool>,
}

/// Represents a [text-to-image](https://huggingface.co/tasks/text-to-image) arguments object.
#[derive(Serialize, Default, Clone, Debug)]
pub struct TTIParams {
    /// An optional negative prompt for the image generation.
    pub negative_prompt: Option<String>,

    /// API uses a default of 1024 if None
    pub width: Option<u32>,
    
    /// API uses a default of 1024 if None
    pub height: Option<u32>,

    /// The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.
    pub num_inference_steps: Option<u32>,

    /// Guidance scale: Higher guidance scale encourages to generate images that are closely linked to the text `prompt`, usually at the expense of lower image quality.
    pub guidance_scale: Option<f32>,
}