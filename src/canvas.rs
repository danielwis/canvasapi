use crate::error::CanvasError;

use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};

pub struct Canvas {
    client: reqwest::Client,
    api_url: String,
}

impl Canvas {
    // TODO: Include API version as an argument
    pub fn init(api_url: &str, api_token: &str) -> Result<Self, CanvasError> {
        // TODO: Warn/error if URL...
        // - contains an API version
        // - contains "http://"
        // - does not contain "://"
        // - is empty
        // See the `Canvas` object at https://github.com/ucfopen/canvasapi/ for more info
        Ok(Self {
            client: reqwest::Client::builder()
                .default_headers({
                    let mut headers = HeaderMap::new();
                    headers.insert(
                        AUTHORIZATION,
                        HeaderValue::from_str(&format!("Bearer {}", api_token))
                            .expect("Token must consist of only visible ASCII charcters"),
                    );
                    headers
                })
                .build()?,
            api_url: api_url.to_string(),
        })
    }

    pub async fn get(&self, endpoint: &str) -> reqwest::Result<reqwest::Response> {
        self.client
            .get(format!("{}/api/v1/{}", self.api_url, endpoint))
            .send()
            .await
    }
}
