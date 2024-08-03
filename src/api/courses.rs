use crate::{models::courses::Course, Canvas, CanvasResult, PaginatedVec};
pub struct CourseHandler<'canvas> {
    canvas: &'canvas Canvas,
}
impl<'canvas> CourseHandler<'canvas> {
    pub fn new(canvas: &'canvas Canvas) -> Self {
        Self { canvas }
    }
}

impl<'canvas> CourseHandler<'canvas> {
    /// Get a specific course.
    pub async fn get(self, course_id: u32) -> CanvasResult<Course> {
        self.canvas
            .get_endpoint(&format!("courses/{course_id}"), None)
            .await
    }

    /// List the current user's active courses.
    ///
    /// The current user is the one to which the API token belongs.
    pub async fn list(&self) -> PaginatedVec<Course> {
        self.canvas.stream_endpoint("courses").await
    }

    /// List the active courses for a specific user.
    pub async fn list_for_user(&self, user_id: u32) -> PaginatedVec<Course> {
        self.canvas
            .stream_endpoint(&format!("users/{user_id}/courses"))
            .await
    }
}
