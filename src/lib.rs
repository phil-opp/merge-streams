//! Merge multiple streams into one.
//! 
//! Based on Yoshua Wuyts's
//! [`futures-concurrency`](https://docs.rs/futures-concurrency/latest)
//! crate and the corresponding
//! [_Futures Concurrency III_](https://blog.yoshuawuyts.com/futures-concurrency-3/)
//! post.
//! 
//! The main trait of this crate is [`MergeStreams`], which provides a `merge`
//! function on tuples, arrays, and vectors of streams. The [`StreamExt`]
//! trait provides a `Stream::merge` method to make merging two streams more
//! convenient.
//! 
//! # Example
//!
//! Merge multiple streams to handle values as soon as they're ready, without
//! ever dropping a single value:
//!
//! ```
//! use merge_streams::MergeStreams;
//! use futures_lite::future::block_on;
//! use futures_lite::{stream, StreamExt};
//!
//! fn main() {
//!     block_on(async {
//!         let a = stream::once(1);
//!         let b = stream::once(2);
//!         let c = stream::once(3);
//!         let mut s = (a, b, c).merge();
//!
//!         let mut counter = 0;
//!         s.for_each(|n| counter += n).await;
//!         assert_eq!(counter, 6);
//!     })
//! }
//! ```
//!

#![deny(missing_debug_implementations, nonstandard_style)]
#![warn(missing_docs, unreachable_pub)]

mod merge;
mod stream;

pub use merge::MergeStreams;
pub use stream::{IntoStream, StreamExt};

mod utils;

