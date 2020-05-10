// Copyright (c) 2020 const-cstr developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

//! Create static C-compatible strings from Rust string literals.
//!
//! Example
//! -------
//!
//! ```rust
//! use zombiezen_const_cstr::{const_cstr, ConstCStr};
//!
//! use std::os::raw::c_char;
//! use std::ffi::CStr;
//!
//! /// Declare a constant:
//! const HELLO_CSTR: ConstCStr = const_cstr!("Hello, world!");
//!
//! // Imagine this is an `extern "C"` function linked from some other lib.
//! unsafe fn print_c_string(cstr: *const c_char) {
//!     println!("{}", CStr::from_ptr(cstr).to_str().unwrap());
//! }
//!
//! fn main() {
//!     let goodnight_cstr = const_cstr!("Goodnight, sun!");
//!
//!     unsafe {
//!         print_c_string(HELLO_CSTR.as_ptr());
//!         print_c_string(goodnight_cstr.as_ptr());
//!     }
//! }
//! ```
//!
//! Prints:
//!
//! ```notest
//! Hello, world!
//! Goodnight, sun!
//! ```

use std::fmt::{self, Display};
use std::ffi::CStr;
use std::os::raw::c_char;

/// A reference to a C-compatible string constant.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstCStr {
    val: &'static str,
}

impl ConstCStr {
    /// Unsafely creates a constant C string reference from a string slice.
    ///
    /// Prefer using the `const_cstr!` macro than calling this function directly.
    ///
    /// # Safety
    ///
    /// The provided slice **must** be NUL-terminated and not contain any
    /// interior NUL bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use zombiezen_const_cstr::ConstCStr;
    ///
    /// let s = unsafe { ConstCStr::from_str_with_nul_unchecked("foo\0") };
    /// assert_eq!(s.as_str(), "foo");
    /// ```
    #[inline]
    pub const unsafe fn from_str_with_nul_unchecked(val: &'static str) -> ConstCStr {
        ConstCStr { val }
    }

    /// Returns the referenced string without the terminating NUL byte.
    #[inline]
    pub fn as_str(self) -> &'static str {
        &self.val[..self.val.len() - 1]
    }

    /// Returns the referenced string as a byte slice **without** the
    /// terminating NUL byte.
    #[inline]
    pub fn as_bytes(self) -> &'static [u8] {
        self.as_str().as_bytes()
    }

    /// Returns the referenced string as a byte slice, **with** the NUL terminating byte.
    #[inline]
    pub const fn as_bytes_with_nul(self) -> &'static [u8] {
        self.val.as_bytes()
    }

    /// Returns a pointer to the beginning of the string constant.
    ///
    /// Suitable for passing to any function that expects a C-compatible string.
    /// Since the underlying string is guaranteed to be `'static`,
    /// the pointer should always be valid.
    #[inline]
    pub const fn as_ptr(self) -> *const c_char {
        self.val.as_bytes().as_ptr() as *const c_char
    }

    /// Returns `&'static CStr` to the referenced string.
    #[inline]
    pub fn as_cstr(self) -> &'static CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(self.as_bytes_with_nul()) }
    }
}

impl Display for ConstCStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for ConstCStr {
    /// Returns an empty C string constant.
    #[inline]
    fn default() -> ConstCStr {
        const_cstr!("")
    }
}

impl AsRef<str> for ConstCStr {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<[u8]> for ConstCStr {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl AsRef<CStr> for ConstCStr {
    fn as_ref(&self) -> &CStr {
        self.as_cstr()
    }
}

impl From<ConstCStr> for &'static str {
    fn from(c: ConstCStr) -> &'static str {
        c.as_str()
    }
}

impl From<ConstCStr> for &'static [u8] {
    fn from(c: ConstCStr) -> &'static [u8] {
        c.as_bytes()
    }
}

impl From<ConstCStr> for *const c_char {
    fn from(c: ConstCStr) -> *const c_char {
        c.as_ptr()
    }
}

impl From<ConstCStr> for &'static CStr {
    fn from(c: ConstCStr) -> &'static CStr {
        c.as_cstr()
    }
}

/// Create a C-compatible constant string by appending a NUL byte to the
/// passed string.
///
/// See crate root documentation for example usage.
///
/// # Safety
///
/// The passed string must not contain any NUL bytes.
#[macro_export]
macro_rules! const_cstr {
    ($strval:expr) => {
        unsafe { $crate::ConstCStr::from_str_with_nul_unchecked(concat!($strval, "\0")) }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        const HELLO: ConstCStr = const_cstr!("Hello, World!");
        assert_eq!(HELLO.as_str(), "Hello, World!");
        assert_eq!(HELLO.as_bytes(), b"Hello, World!");
        assert_eq!(HELLO.as_bytes_with_nul(), b"Hello, World!\0");
        assert_eq!(
            unsafe { CStr::from_ptr(HELLO.as_ptr()) },
            CStr::from_bytes_with_nul(b"Hello, World!\0").unwrap(),
        );
        assert_eq!(
            HELLO.as_cstr(),
            CStr::from_bytes_with_nul(b"Hello, World!\0").unwrap(),
        );
    }
}
