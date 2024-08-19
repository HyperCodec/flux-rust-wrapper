use std::io::Cursor;

use image::{DynamicImage, ImageReader};
use reqwest::Client;
use serde::Serialize;

use crate::error::*;

const API_URL_START: &str = "https://api-inference.huggingface.co/models/black-forest-labs/FLUX";

pub const SCHNELL: &str = "schnell";
pub const DEV: &str = "dev";

/// A helper function for building the API url for different versions and branches.
pub fn build_url(version: &str, branch: &str) -> String {
    format!("{API_URL_START}.{version}-{branch}")
}

/// A struct containing information necessary to request images. Use [`FluxClient::new`] when constructing this.
#[derive(Clone, Debug)]
pub struct FluxClient {
    pub token: String,
    pub url: String,
    pub request_client: Client,
}

impl FluxClient {
    pub fn new(token: impl Into<String>, version: u32, branch: impl AsRef<str>) -> Self {
        let token = token.into();
        let url = build_url(&version.to_string(), branch.as_ref());

        Self {
            token,
            url,
            request_client: Client::new(),
        }
    }

    /// Requests and decodes an image from the flux inference API.
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
}

/// The data that will be sent to the Huggingface Inference API.
#[derive(Serialize, Default, Clone, Debug)]
pub struct InferencePayload {
    #[serde(rename = "inputs")]
    pub prompt: String,

    pub parameters: TTIParams,

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