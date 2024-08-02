use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
// A set of restrictions on editing for copied objects in associated courses
pub struct BlueprintRestrictions {
    // Restriction on main content (e.g. title, description).
    pub content: bool,
    // Restriction on points possible for assignments and graded learning objects
    pub points: bool,
    // Restriction on due dates for assignments and graded learning objects
    pub due_dates: bool,
    // Restriction on availability dates for an object
    pub availability_dates: bool,
}
