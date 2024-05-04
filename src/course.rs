use crate::canvas::Canvas;
use crate::enrollment::Enrollment;
use crate::timestamps::deserialize_optional_timestamp;
use crate::{
    blueprint_course::BlueprintRestrictions, error::CanvasError, grading_period::GradingPeriod,
    permission::Permission,
};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Display;
use time::OffsetDateTime;

impl Canvas {
    pub async fn get_course(&self, course_id: u32) -> Result<Course, CanvasError> {
        let course = self
            .get(&format!("courses/{course_id}"))
            .await?
            .json::<Course>()
            .await?;
        Ok(course)
    }

    pub async fn list_courses(&self) -> Result<Vec<Course>, CanvasError> {
        let courses = self.get("courses").await?.json::<Vec<Course>>().await?;
        Ok(courses)
    }

    pub async fn list_courses_for_user(&self, user_id: u32) -> Result<Vec<Course>, CanvasError> {
        let courses = self
            .get(&format!("users/{user_id}/courses"))
            .await?
            .json::<Vec<Course>>()
            .await?;
        Ok(courses)
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PageType {
    Feed,
    Wiki,
    Modules,
    Assignments,
    Syllabus,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GradePassbackSetting {
    NightlySync,
    Disabled,
    Empty,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CourseFormat {
    OnCampus,
    Online,
    Blended,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkflowState {
    Unpublished,
    Available,
    Completed,
    Deleted,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Term {
    pub id: u32,
    pub name: String,
    pub start_at: Option<OffsetDateTime>,
    pub end_at: Option<OffsetDateTime>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CourseProgress {
    // total number of requirements from all modules
    pub requirement_count: u32,
    // total number of requirements the user has completed from all modules
    pub requirement_completed_count: u32,
    // url to next module item that has an unmet requirement. null if the user has
    // completed the course or the current module does not require sequential
    // progress
    pub next_requirement_url: String,
    // date the course was completed. null if the course has not been completed by
    // this user
    pub completed_at: Option<OffsetDateTime>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CalendarLink {
    pub ics: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Course {
    // the unique identifier for the course
    pub id: u32,
    // the SIS identifier for the course, if defined. This field is only included if
    // the user has permission to view SIS information.
    pub sis_course_id: Option<String>,
    // the UUID of the course
    pub uuid: String,
    // the integration identifier for the course, if defined. This field is only
    // included if the user has permission to view SIS information.
    pub integration_id: Option<u32>,
    // the unique identifier for the SIS import. This field is only included if the
    // user has permission to manage SIS information.
    pub sis_import_id: Option<u32>,
    // the full name of the course. If the requesting user has set a nickname for
    // the course, the nickname will be shown here.
    pub name: String,
    // the course code
    pub course_code: String,
    // the actual course name. This field is returned only if the requesting user
    // has set a nickname for the course.
    pub original_name: Option<String>,
    // the current state of the course one of 'unpublished', 'available',
    // 'completed', or 'deleted'
    pub workflow_state: WorkflowState,
    // the account associated with the course
    pub account_id: u32,
    // the root account associated with the course
    pub root_account_id: u32,
    // the enrollment term associated with the course
    pub enrollment_term_id: u32,
    // A list of grading periods associated with the course
    pub grading_periods: Option<Vec<GradingPeriod>>,
    // the grading standard associated with the course
    pub grading_standard_id: Option<u32>,
    // the grade_passback_setting set on the course
    pub grade_passback_setting: Option<GradePassbackSetting>,
    // the date the course was created.
    #[serde(deserialize_with = "deserialize_optional_timestamp")]
    pub created_at: Option<OffsetDateTime>,
    // the start date for the course, if applicable
    #[serde(deserialize_with = "deserialize_optional_timestamp")]
    pub start_at: Option<OffsetDateTime>,
    // the end date for the course, if applicable
    #[serde(deserialize_with = "deserialize_optional_timestamp")]
    pub end_at: Option<OffsetDateTime>,
    // the course-set locale, if applicable
    pub locale: Option<String>,
    // A list of enrollments linking the current user to the course. for student
    // enrollments, grading information may be included if include[]=total_scores
    pub enrollments: Option<Vec<Enrollment>>,
    // optional: the total number of active and invited students in the course
    pub total_students: Option<u32>,
    // course calendar
    pub calendar: Option<CalendarLink>,
    // the type of page that users will see when they first visit the course -
    // 'feed': Recent Activity Dashboard - 'wiki': Wiki Front Page - 'modules':
    // Course Modules/Sections Page - 'assignments': Course Assignments List -
    // 'syllabus': Course Syllabus Page other types may be added in the future
    pub default_view: PageType,
    // optional: user-generated HTML for the course syllabus
    pub syllabus_body: Option<String>,
    // optional: the number of submissions needing grading returned only if the
    // current user has grading rights and include[]=needs_grading_count
    pub needs_grading_count: Option<u32>,
    // optional: the enrollment term object for the course returned only if
    // include[]=term
    pub term: Option<Term>,
    // optional: information on progress through the course returned only if
    // include[]=course_progress
    pub course_progress: Option<CourseProgress>,
    // weight final grade based on assignment group percentages
    pub apply_assignment_group_weights: bool,
    // optional: the permissions the user has for the course. returned only for a
    // single course and include[]=permissions
    pub permissions: Option<HashMap<Permission, bool>>,
    pub is_public: Option<bool>,
    pub is_public_to_auth_users: bool,
    pub public_syllabus: bool,
    pub public_syllabus_to_auth: bool,
    // optional: the public description of the course
    pub public_description: Option<String>,
    pub storage_quota_mb: u32,
    pub storage_quota_used_mb: Option<u32>,
    pub hide_final_grades: bool,
    pub license: Option<String>,
    pub allow_student_assignment_edits: Option<bool>,
    pub allow_wiki_comments: Option<bool>,
    pub allow_student_forum_attachments: Option<bool>,
    pub open_enrollment: Option<bool>,
    pub self_enrollment: Option<bool>,
    pub restrict_enrollments_to_course_dates: bool,
    pub course_format: Option<CourseFormat>,
    // optional: this will be true if this user is currently prevented from viewing
    // the course because of date restriction settings
    pub access_restricted_by_date: Option<bool>,
    // The course's IANA time zone name.
    pub time_zone: String,
    // optional: whether the course is set as a Blueprint Course (blueprint fields
    // require the Blueprint Courses feature)
    pub blueprint: bool,
    // optional: Set of restrictions applied to all locked course objects
    blueprint_restrictions: Option<BlueprintRestrictions>,
    // optional: Sets of restrictions differentiated by object type applied to
    // locked course objects
    // TODO: Enable the following
    // blueprint_restrictions_by_object_type: {"assignment":{"content":true,"points":true},"wiki_page":{"content":true}},
    // optional: whether the course is set as a template (requires the Course
    // Templates feature)
    pub template: bool,
}

impl Display for Course {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (id {})", self.name, self.id)
    }
}
