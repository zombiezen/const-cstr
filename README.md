# `const-cstr` [![](https://img.shields.io/crates/v/zombiezen-const-cstr.svg)](https://crates.io/crates/zombiezen-const-cstr)

Create static C-compatible strings from Rust string literals.

This is forked from [abonander/const-cstr][] to implement an API that conforms
to the [Rust API Guidelines][] and takes advantage of [`const` functions][] to
hide more implementation details.

[abonander/const-cstr]: https://github.com/abonander/const-cstr
[`const` functions]: https://doc.rust-lang.org/reference/items/functions.html#const-functions
[Rust API Guidelines]: https://rust-lang.github.io/api-guidelines/

## Usage

Cargo.toml:

```toml
[dependencies]
zombiezen-const-cstr = "1.0"
```

## Example

```rust
use zombiezen_const_cstr::{const_cstr, ConstCStr};

use std::os::raw::c_char;
use std::ffi::CStr;

/// Declare a constant:
const HELLO_CSTR: ConstCStr = const_cstr!("Hello, world!");

// Imagine this is an `extern "C"` function linked from some other lib.
unsafe fn print_c_string(cstr: *const c_char) {
    println!("{}", CStr::from_ptr(cstr).to_str().unwrap());
}

fn main() {
    let goodnight_cstr = const_cstr!("Goodnight, sun!");

    unsafe {
        print_c_string(HELLO_CSTR.as_ptr());
        print_c_string(goodnight_cstr.as_ptr());
    }
}
 ```

 Prints:

 ```notest
 Hello, world!
 Goodnight, sun!
 ```

## License

Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be dual licensed as above, without any
additional terms or conditions.
