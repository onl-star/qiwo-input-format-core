use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use crate::{CommitContext, FormatOptions, format_commit_text};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QiwoInputFormatOptions {
    pub auto_spacing_enabled: bool,
}

impl From<QiwoInputFormatOptions> for FormatOptions {
    fn from(value: QiwoInputFormatOptions) -> Self {
        Self {
            auto_spacing_enabled: value.auto_spacing_enabled,
        }
    }
}

/// Formats UTF-8 commit text for C callers.
///
/// # Safety
/// Non-null pointers must reference valid null-terminated UTF-8 strings.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn qiwo_input_format_commit_text(
    commit_text: *const c_char,
    before_cursor: *const c_char,
    after_cursor: *const c_char,
    options: QiwoInputFormatOptions,
) -> *mut c_char {
    let commit_text = unsafe { c_string_to_str(commit_text) }.unwrap_or("");
    let before_cursor = unsafe { c_string_to_str(before_cursor) };
    let after_cursor = unsafe { c_string_to_str(after_cursor) };
    let formatted = format_commit_text(
        commit_text,
        CommitContext {
            before_cursor,
            after_cursor,
        },
        options.into(),
    );

    to_c_string(&formatted)
}

/// Frees strings returned by qiwo_input_format_commit_text().
///
/// # Safety
/// `value` must be null or a pointer returned by qiwo_input_format_commit_text().
#[unsafe(no_mangle)]
pub unsafe extern "C" fn qiwo_input_format_free_string(value: *mut c_char) {
    if value.is_null() {
        return;
    }

    unsafe {
        let _ = CString::from_raw(value);
    }
}

unsafe fn c_string_to_str<'a>(value: *const c_char) -> Option<&'a str> {
    if value.is_null() {
        return None;
    }

    unsafe { CStr::from_ptr(value) }.to_str().ok()
}

fn to_c_string(value: &str) -> *mut c_char {
    CString::new(value)
        .unwrap_or_else(|_| CString::new("").expect("empty string has no nul byte"))
        .into_raw()
}
