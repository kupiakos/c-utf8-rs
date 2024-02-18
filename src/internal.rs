//! Internal/unstable types that can change without a breaking change to the crate.

/// Panics (in `const`) if there is a nul character in `x`.
pub const fn check_no_nul(x: &str) {
    let bytes = x.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'\0' {
            panic!("nul located in the middle of a c_utf8! literal");
        }
        i += 1;
    }
}
