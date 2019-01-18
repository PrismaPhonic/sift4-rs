# sift4

This is a rust implementation of the sift4 string distance algorithm. Sift4 is
very fast and unlike sift3 more closely resembles what you would get from the
Levenshtein distance algorithm.  

# The Algorithm
 [Sift4 distance](https://siderite.blogspot.com/2014/11/super-fast-and-accurate-string-distance.html)

## Add as dependency
sift4 is available on [crates.io](https://crates.io/crates/sift4).

```toml
[dependencies]
sift4 = "0.1"
```

## Usage
```rust
use sift4::*; 

let distance = sift4("London", "Londo");
assert_eq!(1, distance);
```
