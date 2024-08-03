// TODO: Look into Url type for url fields instead of Strings
// TODO: Look into proper timezone/locale types
pub mod error;
pub mod models;
pub mod timestamps;

use futures::Stream;
use std::pin::Pin;
pub type PaginatedVec<'a, T> = Pin<Box<dyn Stream<Item = T> + 'a>>;

use crate::error::CanvasError;

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

async fn convert_response<R: DeserializeOwned>(resp: reqwest::Response) -> CanvasResult<R> {
    resp.json::<R>().await.map_err(Into::into)
}

/* // Course
impl Canvas {
    pub async fn get_course(&self, course_id: u32) -> Result<Course, CanvasError> {
        let course = self
            .get_endpoint(&format!("courses/{course_id}"))
            .await?
            .json::<Course>()
            .await?;
        Ok(course)
    }

    pub async fn list_courses(&self) -> PaginatedVec<Course> {
        self.stream::<Course>("courses").await
    }

    pub async fn list_courses_for_user(&self, user_id: u32) -> Result<Vec<Course>, CanvasError> {
        let courses = self
            .get_endpoint(&format!("users/{user_id}/courses"))
            .await?
            .json::<Vec<Course>>()
            .await?;
        Ok(courses)
    }
}

// User
impl Canvas {
    pub async fn get_user(&self, user_id: u32) -> Result<User, CanvasError> {
        let user = self
            .get_endpoint(&format!("users/{user_id}"))
            .await?
            .json::<User>()
            .await?;
        Ok(user)
    }

    pub async fn list_users_in_account(&self, account_id: u32) -> Result<Vec<User>, CanvasError> {
        let users = self
            .get_endpoint(&format!("accounts/{account_id}/users"))
            .await?
            .json::<Vec<User>>()
            .await?;
        Ok(users)
    }
} */

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

    pub async fn get(
        &self,
        url: &str,
        headers: Option<HeaderMap>,
    ) -> CanvasResult<reqwest::Response> {
        let mut req = self.client.get(url);
        if let Some(headers) = headers {
            req = req.headers(headers);
        }
        req.send().await.map_err(Into::into)
    }

    pub async fn get_endpoint<R: DeserializeOwned>(
        &self,
        endpoint: &str,
        headers: Option<HeaderMap>,
    ) -> CanvasResult<R> {
        let resp = self.get(&self.url_from_endpoint(endpoint), headers).await?;
        convert_response(resp).await
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

    // TODO: Error handling, return results? This means using futures::StreamTryExt instead of
    // StreamExt
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
