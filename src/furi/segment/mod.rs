pub mod encode;
pub mod iter;
pub mod kanji;
pub mod s_owned;
pub mod s_ref;
pub mod traits;

pub use s_owned::Segment;
pub use s_ref::SegmentRef;
pub use traits::{AsSegment, AsSegmentRef};
