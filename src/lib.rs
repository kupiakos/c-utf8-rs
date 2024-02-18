//! Operations on [UTF-8]-encoded [C strings][c_str].
//!
//! The [`CUtf8`] and [`CUtf8Buf`] types are guaranteed to be:
//!
//! - [Nul (Ø) terminated C strings][c_str] in order to more safely ensure that
//!   C APIs only access memory that is properly owned.
//!
//! - Encoded as valid [UTF-8], allowing for passing around native Rust [`str`]
//!   strings with ease.
//!
//! # Usage
//!
//! This crate is available [on crates.io](https://crates.io/crates/c_utf8) and
//! can be used by adding the following to your project's
//! [`Cargo.toml`](https://doc.rust-lang.org/cargo/reference/manifest.html):
//!
//! ```toml
//! [dependencies]
//! c_utf8 = "0.1"
//! ```
//!
//! and this to your crate root (`lib.rs` or `main.rs`):
//!
//! ```
//! #[macro_use] // enables c_utf8! macro
//! extern crate c_utf8;
//! # fn main() {}
//! ```
//!
//! # Examples
//!
//! A [`CUtf8`] slice can be created via the [`c_utf8!`](macro.c_utf8.html)
//! macro, which ensures it will _always_ end with a trailing 0 byte:
//!
//! ```
//! # #[macro_use] extern crate c_utf8;
//! use c_utf8::{c_utf8, CUtf8};
//!
//! const MESSAGE: &CUtf8 = c_utf8!("Heyo!");
//!
//! fn main() {
//!     let bytes = [72, 101, 121, 111, 33, 0];
//!     assert_eq!(MESSAGE.as_bytes_with_nul(), &bytes);
//! }
//! ```
//!
//! # Donate
//!
//! This project is made freely available (as in free beer), but unfortunately
//! not all beer is free! So, if you would like to buy me a beer (or coffee or
//! *more*), then consider supporting my work that's benefited your project
//! and thousands of others.
//!
//! <a href="https://www.patreon.com/nvzqz">
//!     <img src="https://c5.patreon.com/external/logo/become_a_patron_button.png" alt="Become a Patron!" height="35">
//! </a>
//! <a href="https://www.paypal.me/nvzqz">
//!     <img src="https://buymecoffee.intm.org/img/button-paypal-white.png" alt="Buy me a coffee" height="35">
//! </a>
//!
//! [UTF-8]:      https://en.wikipedia.org/wiki/UTF-8
//! [c_str]:      https://en.wikipedia.org/wiki/Null-terminated_string
//! [`str`]:      https://doc.rust-lang.org/std/primitive.str.html
//! [`CUtf8`]:    struct.CUtf8.html
//! [`CUtf8Buf`]: struct.CUtf8Buf.html

#![deny(missing_docs)]
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

/// Creates a [`&'static CUtf8`](struct.CUtf8.html) from a native Rust [`str`]
/// string literal, making it much easier to work with C APIs that are strict
/// about encoding input as UTF-8.
///
/// # Usage
///
/// Although the input string can have a 0 byte, it is **highly recommended** to
/// not have one. This is because C APIs will only work with the memory up to
/// the first 0 byte. In the future, it will be very likely be a **hard error**
/// to have a 0 byte within the string literal.
///
/// # Examples
///
/// The resulting string will _always_ end with a 0 byte:
///
/// ```
/// use c_utf8::c_utf8;
/// let string = c_utf8!("Hello!");
/// let bytes = *b"Hello!\0";
/// assert_eq!(string.as_bytes_with_nul(), &bytes);
/// ```
///
/// The macro can even be evaluated within a constant expression. This allows
/// for having instances of types with `&'static CUtf8` fields.
///
/// ```
/// # #[macro_use] extern crate c_utf8; use c_utf8::CUtf8; fn main() {
/// const APP_NAME: &CUtf8 = c_utf8!(env!("CARGO_PKG_NAME"));
///
/// assert_eq!(APP_NAME.as_str_with_nul(), "c_utf8\0");
/// # }
/// ```
///
/// A `c_utf8!` literal cannot contain any intermediate `\0`:
///
/// ```compile_fail
/// # use c_utf8::c_utf8;
/// let _fails = c_utf8!("Null\0in the middle");
/// ```
///
/// [`str`]: https://doc.rust-lang.org/std/primitive.str.html
#[macro_export]
macro_rules! c_utf8 {
    ($s:expr) => {
        // SAFETY:
        // - `$s` is guaranteed to be a UTF-8 `str`,
        // - The `concat!` guarantees the input is nul terminated
        // - `check_no_nul` ensures that `$s` contains no other nul terminators
        unsafe {
            const _: () = $crate::__internal_unstable::check_no_nul($s);
            $crate::CUtf8::from_str_unchecked(concat!($s, "\0"))
        }
    };
}

mod c_utf8;
#[cfg(feature = "alloc")]
mod c_utf8_buf;
mod error;

pub use self::c_utf8::*;
#[cfg(feature = "alloc")]
pub use self::c_utf8_buf::*;
pub use self::error::*;

#[path = "internal.rs"]
#[doc(hidden)]
pub mod __internal_unstable;
