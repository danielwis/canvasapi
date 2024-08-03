use crate::{models::users::User, Canvas, CanvasResult, PaginatedVec};
pub struct UserHandler<'canvas> {
    canvas: &'canvas Canvas,
}
impl<'canvas> UserHandler<'canvas> {
    pub fn new(canvas: &'canvas Canvas) -> Self {
        Self { canvas }
    }
}

impl<'canvas> UserHandler<'canvas> {
    /// Get a specific user.
    pub async fn get(self, user_id: u32) -> CanvasResult<User> {
        self.canvas
            .get_endpoint(&format!("users/{user_id}"), None)
            .await
    }

    /// List the active courses for a specific user.
    pub async fn list_for_account(&self, account_id: u32) -> PaginatedVec<User> {
        self.canvas
            .stream_endpoint(&format!("accounts/{account_id}/users"))
            .await
    }
}
