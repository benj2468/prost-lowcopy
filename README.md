# LowCopy Protocol Buffers Rust

This project is a proof on concept of what a low-copy framework might look like for protocol buffers in rust. It is by no means complete, and a means by which I can share ideas and solicit feedback from the community before investing in code-gen for this work.

It's called low-copy because it is not 100% memory-less. There is current SOME overhead because of the mechanics of how protocol buffers are serialized.

Currently the idea is to construct an `Observer` struct, which has a reference to the underlying byte array:

```rust
#[derive(Default)]
pub struct PersonObserver<'a> {
    inner: &'a [u8],
    id: Option<usize>,
    given_name: Option<usize>,
    // Sized repeating fields that are not packed
    // I feel like there is no way to do this... 
    // without needing to iterate over the entire buffer again...
    middle_names: Vec<usize>,
    // A packed repeating field
    digits: Option<usize>,
    gender: Gender,
    children: Vec<usize>,
}
```

We will keep markers, `usize`s to reference locations inside that buffer that specific fields start.

- We construct these pointers when we first create the Observer, so that we can provide the decode error immediately. This means we can (kinda) safety unwrap when we actually go to parse it the second time. Not sure I like this mechanism. Open to ideas here. 

- I started with `&'a str` to hold strings for example, but that holds a pointer AND a length. The length is extra memory since the length is already encoded in the VARINT of the protobuf encoding. 

The reason it is not entirely non-allocated memory, are the non packed fields, such as length delimited repeating fields. For these, since we cannot just use a single pointer to the start of the packed group of repeated elements, we need to keep a pointer to each one, dynamically adding a pointer each time we find an element. 
    - The alternative here is that we just DONT keep these in memory, and whenever we want to read these values, we iterate over the entire buffer... 
    - OR we keep a start & stop, meaning we could have a pointer to the first time we see one, and a pointer to the last time we see one, this might reduce the overhead of searching each time we read, and also would remove the need for dynamic allocation. The down side is that the start and stop could be at most as large as the entire buffer though. In the worst case this is no different than the first alternative option.

Most of the parsing functions were adapted from the `prost` functions to take in immutable buffers, and return the shift that they underwent. This allows us to iterate over buffers, rather than mutate them (and consume) them as we go.

I think this might be a beneficial upstream change, but additional profiling should be done to ensure this is in fact more efficient.


## Developing

I use nix to manage my shells, once you have nix installed (recommend determinate)

```shell
nix develop
cargo run
```