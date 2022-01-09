#[cfg(feature = "std")]
pub use std::ffi::{CStr, CString};

#[cfg(not(feature = "std"))]
pub use cstr_core::CStr;

#[cfg(all(not(feature = "std"), feature = "alloc"))]
pub use cstr_core::CString;

#[cfg(feature = "alloc")]
extern crate alloc;

use esp_idf_sys::c_types;

#[cfg(feature = "alloc")]
pub fn set_str(buf: &mut [u8], s: &str) {
    assert!(s.len() < buf.len());
    let cs = CString::new(s).unwrap();
    let ss: &[u8] = cs.as_bytes_with_nul();
    buf[..ss.len()].copy_from_slice(ss);
}

#[cfg(feature = "alloc")]
pub fn from_cstr_ptr<'a>(ptr: *const c_types::c_char) -> alloc::borrow::Cow<'a, str> {
    unsafe { CStr::from_ptr(ptr) }.to_string_lossy()
}

#[cfg(feature = "alloc")]
pub fn from_cstr(buf: &[u8]) -> alloc::borrow::Cow<'_, str> {
    // We have to find the first '\0' ourselves, because the passed buffer might
    // be wider than the ASCIIZ string it contains
    let len = buf.iter().position(|e| *e == 0).unwrap() + 1;

    unsafe { CStr::from_bytes_with_nul_unchecked(&buf[0..len]) }.to_string_lossy()
}

#[cfg(feature = "alloc")]
pub struct RawCstrs(alloc::vec::Vec<CString>);

#[cfg(feature = "alloc")]
impl RawCstrs {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn as_ptr(&mut self, s: impl AsRef<str>) -> *const c_types::c_char {
        let cs = CString::new(s.as_ref()).unwrap();

        let cstr_ptr = cs.as_ptr();

        self.0.push(cs);

        cstr_ptr
    }

    pub fn as_nptr<S>(&mut self, s: Option<S>) -> *const c_types::c_char
    where
        S: AsRef<str>,
    {
        s.map(|s| self.as_ptr(s)).unwrap_or(core::ptr::null())
    }
}

#[cfg(feature = "alloc")]
impl Default for RawCstrs {
    fn default() -> Self {
        RawCstrs::new()
    }
}
