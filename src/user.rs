use crate::canvas::Canvas;
use crate::enrollment::Enrollment;
use crate::error::CanvasError;
use crate::timestamps::deserialize_optional_timestamp;

use serde::{Deserialize, Serialize};
use std::fmt::Display;
use time::OffsetDateTime;

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
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AvatarState {
    None,
    Submitted,
    Approved,
    Locked,
    Reported,
    ReReported,
}

// This mini-object is used for secondary user responses, when we just want to
// provide enough information to display a user.
#[derive(Debug, Deserialize, Serialize)]
pub struct UserDisplay {
    // The ID of the user.
    pub id: u32,
    // A short name the user has selected, for use in conversations or other less
    // formal places through the site.
    pub short_name: String,
    // If avatars are enabled, this field will be included and contain a url to
    // retrieve the user's avatar.
    pub avatar_image_url: String,
    // URL to access user, either nested to a context or directly.
    pub html_url: String,
}

// This mini-object is returned in place of UserDisplay when returning student
// data for anonymous assignments, and includes an anonymous ID to identify a
// user within the scope of a single assignment.
#[derive(Debug, Deserialize, Serialize)]
pub struct AnonymousUserDisplay {
    // A unique short ID identifying this user within the scope of a particular
    // assignment.
    pub anonymous_id: String,
    // A URL to retrieve a generic avatar.
    pub avatar_image_url: String,
    // The anonymized display name for the student.
    pub display_name: String,
}

// A Canvas user, e.g. a student, teacher, administrator, observer, etc.
#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    // The ID of the user.
    pub id: u32,
    // The name of the user.
    pub name: String,
    // The name of the user that is should be used for sorting groups of users, such
    // as in the gradebook.
    pub sortable_name: String,
    // The last name of the user.
    pub last_name: String,
    // The first name of the user.
    pub first_name: String,
    // A short name the user has selected, for use in conversations or other less
    // formal places through the site.
    pub short_name: String,
    // The SIS ID associated with the user.  This field is only included if the user
    // came from a SIS import and has permissions to view SIS information.
    pub sis_user_id: Option<String>,
    // The id of the SIS import.  This field is only included if the user came from
    // a SIS import and has permissions to manage SIS information.
    pub sis_import_id: Option<u32>,
    // The integration_id associated with the user.  This field is only included if
    // the user came from a SIS import and has permissions to view SIS information.
    pub integration_id: Option<String>,
    // The unique login id for the user.  This is what the user uses to log in to
    // Canvas.
    pub login_id: Option<String>,
    // If avatars are enabled, this field will be included and contain a url to
    // retrieve the user's avatar.
    pub avatar_url: Option<String>,
    // Optional: If avatars are enabled and caller is admin, this field can be
    // requested and will contain the current state of the user's avatar.
    pub avatar_state: Option<AvatarState>,
    // Optional: This field can be requested with certain API calls, and will return
    // a list of the users active enrollments. See the List enrollments API for more
    // details about the format of these records.
    pub enrollments: Option<Vec<Enrollment>>,
    // Optional: This field can be requested with certain API calls, and will return
    // the users primary email address.
    pub email: Option<String>,
    // Optional: This field can be requested with certain API calls, and will return
    // the users locale in RFC 5646 format.
    pub locale: Option<String>,
    // Optional: This field is only returned in certain API calls, and will return a
    // timestamp representing the last time the user logged in to canvas.
    #[serde(deserialize_with = "deserialize_optional_timestamp", default)]
    pub last_login: Option<OffsetDateTime>,
    // Optional: This field is only returned in certain API calls, and will return
    // the IANA time zone name of the user's preferred timezone.
    pub time_zone: Option<String>,
    // Optional: The user's bio.
    pub bio: Option<String>,
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (id {})", self.name, self.id)
    }
}
