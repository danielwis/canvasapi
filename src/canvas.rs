use crate::error::CanvasError;
use crate::PaginatedVec;

use futures::{stream, StreamExt};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use serde::de::DeserializeOwned;

pub struct Canvas {
    client: reqwest::Client,
    api_url: String,
}

#[derive(Debug, Default)]
pub struct PaginationInfo {
    current_url: Option<String>,
    next_url: Option<String>,
    prev_url: Option<String>,
    first_url: Option<String>,
    last_url: Option<String>,
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

    fn api_url(&self, endpoint: &str) -> String {
        format!("{}/api/v1/{}", self.api_url, endpoint)
    }

    pub(crate) async fn get(&self, url: &str) -> reqwest::Result<reqwest::Response> {
        self.client.get(url).send().await
    }

    pub(crate) async fn get_endpoint(&self, endpoint: &str) -> reqwest::Result<reqwest::Response> {
        self.get(&self.api_url(endpoint)).await
    }

    fn parse_pagination_info(link_header: Option<&HeaderValue>) -> Result<PaginationInfo, ()> {
        let mut info: PaginationInfo = Default::default();
        let Some(links) = link_header else {
            return Ok(info);
        };

        let Ok(links) = links.to_str() else {
            return Err(());
        };

        for link in links.split(',') {
            let (link, mut rel) = link.split_once(';').ok_or(())?;
            rel = rel.strip_prefix(" rel=\"").unwrap();
            rel = rel.strip_suffix('\"').unwrap();
            let mut link = link.chars();
            link.next();
            link.next_back();
            let link = link.as_str();
            match rel {
                "current" => info.current_url = Some(link.to_string()),
                "prev" => info.prev_url = Some(link.to_string()),
                "next" => info.next_url = Some(link.to_string()),
                "first" => info.first_url = Some(link.to_string()),
                "last" => info.last_url = Some(link.to_string()),
                _ => { /* warn, unknown rel value */ }
            }
        }

        Ok(info)
    }

    // TODO: Error handling, return results?
    pub(crate) async fn stream<T: DeserializeOwned>(&self, endpoint: &str) -> PaginatedVec<'_, T> {
        let first_url = Some(self.api_url(endpoint));

        Box::pin(
            stream::unfold(first_url, move |state| async {
                let Some(state) = state else {
                    return None;
                };
                let resp = self.get(&state).await.unwrap();
                let pag_info = Canvas::parse_pagination_info(resp.headers().get("link")).unwrap();

                let items = resp.json::<Vec<T>>().await.unwrap();

                Some((stream::iter(items), pag_info.next_url))
            })
            .flatten(),
        )
    }
}
