# sift4
[![Build Status](https://travis-ci.org/PrismaPhonic/sift4-rs.svg?branch=master)](https://travis-ci.org/PrismaPhonic/sift4-rs)
[![crates.io](http://meritbadge.herokuapp.com/sift4)](https://crates.io/crates/sift4)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![Released API docs](https://docs.rs/sift4/badge.svg)](https://docs.rs/sift4)

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

For now there is only the simple algorithm, which uses a max offset of 5:

```rust
extern crate sift4;

let distance = sift4::simple("London", "Londo");
assert_eq!(1, distance);
```
