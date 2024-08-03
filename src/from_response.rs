use reqwest::Response;
use serde::de::DeserializeOwned;
use std::future::Future;

use crate::CanvasResult;

pub trait FromResponse: Sized {
    fn from_response(resp: Response) -> impl Future<Output = CanvasResult<Self>> + Send;
}

impl<T: DeserializeOwned> FromResponse for T {
    async fn from_response(resp: Response) -> CanvasResult<Self> {
        resp.json::<T>().await.map_err(Into::into)
    }
}
