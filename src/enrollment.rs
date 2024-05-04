use crate::timestamps::deserialize_optional_timestamp;

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EnrollmentType {
    StudentEnrollment,
    TeacherEnrollment,
    TaEnrollment,
    ObserverEnrollment,
    DesignerEnrollment,
    #[serde(untagged)]
    Custom(String),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EnrollmentState {
    Active,
    Invited,
    Inactive,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Grade {
    // The URL to the Canvas web UI page for the user's grades, if this is a student
    // enrollment.
    pub html_url: String,
    // The user's current grade in the class. Only included if user has permissions
    // to view this grade.
    pub current_grade: String,
    // The user's final grade for the class. Only included if user has permissions
    // to view this grade.
    pub final_grade: String,
    // The user's current score in the class. Only included if user has permissions
    // to view this score.
    pub current_score: u32,
    // The user's final score for the class. Only included if user has permissions
    // to view this score.
    pub final_score: u32,
    // The total points the user has earned in the class. Only included if user has
    // permissions to view this score and 'current_points' is passed in the
    // request's 'include' parameter.
    pub current_points: u32,
    // The user's current grade in the class including muted/unposted assignments.
    // Only included if user has permissions to view this grade, typically teachers,
    // TAs, and admins.
    pub unposted_current_grade: String,
    // The user's final grade for the class including muted/unposted assignments.
    // Only included if user has permissions to view this grade, typically teachers,
    // TAs, and admins..
    pub unposted_final_grade: String,
    // The user's current score in the class including muted/unposted assignments.
    // Only included if user has permissions to view this score, typically teachers,
    // TAs, and admins..
    pub unposted_current_score: u32,
    // The user's final score for the class including muted/unposted assignments.
    // Only included if user has permissions to view this score, typically teachers,
    // TAs, and admins..
    pub unposted_final_score: u32,
    // The total points the user has earned in the class, including muted/unposted
    // assignments. Only included if user has permissions to view this score
    // (typically teachers, TAs, and admins) and 'current_points' is passed in the
    // request's 'include' parameter.
    pub unposted_current_points: u32,
}

// TODO: Does a Enum make sense for grade fields?
#[derive(Debug, Deserialize, Serialize)]
pub struct Enrollment {
    // The ID of the enrollment.
    pub id: Option<u32>,
    // The unique id of the course.
    pub course_id: Option<u32>,
    // The SIS Course ID in which the enrollment is associated. Only displayed if
    // present. This field is only included if the user has permission to view SIS
    // information.
    pub sis_course_id: Option<String>,
    // The Course Integration ID in which the enrollment is associated. This field
    // is only included if the user has permission to view SIS information.
    pub course_integration_id: Option<String>,
    // The unique id of the user's section.
    pub course_section_id: Option<u32>,
    // The Section Integration ID in which the enrollment is associated. This field
    // is only included if the user has permission to view SIS information.
    pub section_integration_id: Option<String>,
    // The SIS Account ID in which the enrollment is associated. Only displayed if
    // present. This field is only included if the user has permission to view SIS
    // information.
    pub sis_account_id: Option<String>,
    // The SIS Section ID in which the enrollment is associated. Only displayed if
    // present. This field is only included if the user has permission to view SIS
    // information.
    pub sis_section_id: Option<String>,
    // The SIS User ID in which the enrollment is associated. Only displayed if
    // present. This field is only included if the user has permission to view SIS
    // information.
    pub sis_user_id: Option<String>,
    // The state of the user's enrollment in the course.
    pub enrollment_state: EnrollmentState,
    // User can only access his or her own course section.
    pub limit_privileges_to_course_section: bool,
    // The unique identifier for the SIS import. This field is only included if the
    // user has permission to manage SIS information.
    pub sis_import_id: Option<u32>,
    // The unique id of the user's account.
    pub root_account_id: Option<u32>,
    // The enrollment type. One of 'StudentEnrollment', 'TeacherEnrollment',
    // 'TaEnrollment', 'DesignerEnrollment', 'ObserverEnrollment'.
    #[serde(rename = "type")]
    pub enrollment_type: EnrollmentType,
    // The unique id of the user.
    pub user_id: u32,
    // The unique id of the associated user. Will be null unless type is
    // ObserverEnrollment.
    pub associated_user_id: Option<u32>,
    // The enrollment role, for course-level permissions. This field will match
    // `type` if the enrollment role has not been customized.
    pub role: EnrollmentType,
    // The id of the enrollment role.
    pub role_id: u32,
    // The created time of the enrollment, in ISO8601 format.
    #[serde(deserialize_with = "deserialize_optional_timestamp", default)]
    pub created_at: Option<OffsetDateTime>,
    // The updated time of the enrollment, in ISO8601 format.
    #[serde(deserialize_with = "deserialize_optional_timestamp", default)]
    pub updated_at: Option<OffsetDateTime>,
    // The start time of the enrollment, in ISO8601 format.
    #[serde(deserialize_with = "deserialize_optional_timestamp", default)]
    pub start_at: Option<OffsetDateTime>,
    // The end time of the enrollment, in ISO8601 format.
    #[serde(deserialize_with = "deserialize_optional_timestamp", default)]
    pub end_at: Option<OffsetDateTime>,
    // The last activity time of the user for the enrollment, in ISO8601 format.
    #[serde(deserialize_with = "deserialize_optional_timestamp", default)]
    pub last_activity_at: Option<OffsetDateTime>,
    // The last attended date of the user for the enrollment in a course, in ISO8601
    // format.
    #[serde(deserialize_with = "deserialize_optional_timestamp", default)]
    pub last_attended_at: Option<OffsetDateTime>,
    // The total activity time of the user for the enrollment, in seconds.
    pub total_activity_time: Option<u32>,
    // The URL to the Canvas web UI page for this course enrollment.
    pub html_url: Option<String>,
    // The URL to the Canvas web UI page containing the grades associated with this
    // enrollment.
    pub grades: Option<Grade>,
    // A description of the user.
    // TODO
    // user: {"id":u32,"name":"Student 1","sortable_name":"1, Student","short_name":"Stud 1"},

    // The user's override grade for the course.
    pub override_grade: Option<String>,
    // The user's override score for the course.
    pub override_score: Option<f32>,
    // The user's current grade in the class including muted/unposted assignments.
    // Only included if user has permissions to view this grade, typically teachers,
    // TAs, and admins.
    pub unposted_current_grade: Option<String>,
    // The user's final grade for the class including muted/unposted assignments.
    // Only included if user has permissions to view this grade, typically teachers,
    // TAs, and admins..
    pub unposted_final_grade: Option<String>,
    // The user's current score in the class including muted/unposted assignments.
    // Only included if user has permissions to view this score, typically teachers,
    // TAs, and admins..
    pub unposted_current_score: Option<f32>,
    // The user's final score for the class including muted/unposted assignments.
    // Only included if user has permissions to view this score, typically teachers,
    // TAs, and admins..
    pub unposted_final_score: Option<f32>,
    // optional: Indicates whether the course the enrollment belongs to has grading
    // periods set up. (applies only to student enrollments, and only available in
    // course endpoints)
    pub has_grading_periods: Option<bool>,
    // optional: Indicates whether the course the enrollment belongs to has the
    // Display Totals for 'All Grading Periods' feature enabled. (applies only to
    // student enrollments, and only available in course endpoints)
    pub totals_for_all_grading_periods_option: Option<bool>,
    // optional: The name of the currently active grading period, if one exists. If
    // the course the enrollment belongs to does not have grading periods, or if no
    // currently active grading period exists, the value will be null. (applies only
    // to student enrollments, and only available in course endpoints)
    pub current_grading_period_title: Option<String>,
    // optional: The id of the currently active grading period, if one exists. If
    // the course the enrollment belongs to does not have grading periods, or if no
    // currently active grading period exists, the value will be null. (applies only
    // to student enrollments, and only available in course endpoints)
    pub current_grading_period_id: Option<u32>,
    // The user's override grade for the current grading period.
    pub current_period_override_grade: Option<String>,
    // The user's override score for the current grading period.
    pub current_period_override_score: Option<f32>,
    // optional: The student's score in the course for the current grading period,
    // including muted/unposted assignments. Only included if user has permission to
    // view this score, typically teachers, TAs, and admins. If the course the
    // enrollment belongs to does not have grading periods, or if no currently
    // active grading period exists, the value will be null. (applies only to
    // student enrollments, and only available in course endpoints)
    pub current_period_unposted_current_score: Option<f32>,
    // optional: The student's score in the course for the current grading period,
    // including muted/unposted assignments and including ungraded assignments with
    // a score of 0. Only included if user has permission to view this score,
    // typically teachers, TAs, and admins. If the course the enrollment belongs to
    // does not have grading periods, or if no currently active grading period
    // exists, the value will be null. (applies only to student enrollments, and
    // only available in course endpoints)
    pub current_period_unposted_final_score: Option<f32>,
    // optional: The letter grade equivalent of
    // current_period_unposted_current_score, if available. Only included if user
    // has permission to view this grade, typically teachers, TAs, and admins. If
    // the course the enrollment belongs to does not have grading periods, or if no
    // currently active grading period exists, the value will be null. (applies only
    // to student enrollments, and only available in course endpoints)
    pub current_period_unposted_current_grade: Option<String>,
    // optional: The letter grade equivalent of current_period_unposted_final_score,
    // if available. Only included if user has permission to view this grade,
    // typically teachers, TAs, and admins. If the course the enrollment belongs to
    // does not have grading periods, or if no currently active grading period
    // exists, the value will be null. (applies only to student enrollments, and
    // only available in course endpoints)
    pub current_period_unposted_final_grade: Option<String>,
}
