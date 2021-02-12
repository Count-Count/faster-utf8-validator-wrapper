#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use std::os::raw::c_char;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[inline(always)] // to match the really_inline on the underlying simdjson fn
pub fn validate(bytes: &[u8]) -> bool {
    unsafe { z_validate_utf8_avx2(bytes.as_ptr() as *const c_char, bytes.len() as size_t) != 0 }
}

#[cfg(test)]
mod tests {
    use super::validate;

    #[test]
    fn valid_utf8() {
        let hello = "hello world!";
        assert!(validate(hello.as_bytes()));
    }

    #[test]
    fn invalid_utf8() {
        let hello = b"hello world!\xCE";
        assert!(!validate(hello));
    }
}
