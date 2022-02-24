# `merge-streams`

Merge multiple streams into one.

Based on Yoshua Wuyts's [`futures-concurrency`](https://docs.rs/futures-concurrency/latest) crate and the corresponding [_Futures Concurrency III_](https://blog.yoshuawuyts.com/futures-concurrency-3/)
post.

# Example
Merge multiple streams to handle values as soon as they're ready, without ever dropping a single value:

```rust
use merge_streams::MergeStreams;
use futures_lite::future::block_on;
use futures_lite::{stream, StreamExt};
fn main() {
    block_on(async {
        let a = stream::once(1);
        let b = stream::once(2);
        let c = stream::once(3);
        let mut s = (a, b, c).merge();
        let mut counter = 0;
        s.for_each(|n| counter += n).await;
        assert_eq!(counter, 6);
    })
}
```

## License

Licensed under either of <a href="LICENSE-APACHE">Apache License, Version 2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
