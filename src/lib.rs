// TODO: Look into Url type for url fields instead of Strings
// TODO: Look into proper timezone/locale types
pub mod blueprint_course;
pub mod canvas;
pub mod course;
pub mod enrollment;
pub mod error;
pub mod grading_period;
pub mod permission;
pub mod timestamps;
pub mod user;

use futures::Stream;
use std::pin::Pin;
pub type PaginatedVec<'a, T> = Pin<Box<dyn Stream<Item = T> + 'a>>;
