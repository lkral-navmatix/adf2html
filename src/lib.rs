pub mod document;
pub mod marks;
pub mod nodes;

pub use marks::*;
pub use nodes::*;

pub use chrono_tz::{Tz, UTC};
use std::cell::RefCell;

thread_local! {
    pub(crate) static TIMEZONE: RefCell<Tz> = RefCell::new(UTC);  // Default to UTC
}

pub(crate) trait ToHtml {
    fn to_html(&self) -> String;
}
