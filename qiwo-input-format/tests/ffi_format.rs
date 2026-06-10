use std::ffi::{CStr, CString};

use qiwo_input_format::{
    QiwoInputFormatOptions, qiwo_input_format_commit_text, qiwo_input_format_free_string,
};

fn format_via_ffi(
    commit_text: &str,
    before_cursor: Option<&str>,
    after_cursor: Option<&str>,
    enabled: bool,
) -> String {
    let commit_text = CString::new(commit_text).unwrap();
    let before_cursor = before_cursor.map(|value| CString::new(value).unwrap());
    let after_cursor = after_cursor.map(|value| CString::new(value).unwrap());

    let result = unsafe {
        qiwo_input_format_commit_text(
            commit_text.as_ptr(),
            before_cursor
                .as_ref()
                .map_or(std::ptr::null(), |value| value.as_ptr()),
            after_cursor
                .as_ref()
                .map_or(std::ptr::null(), |value| value.as_ptr()),
            QiwoInputFormatOptions {
                auto_spacing_enabled: enabled,
            },
        )
    };

    assert!(!result.is_null());
    let output = unsafe { CStr::from_ptr(result).to_string_lossy().into_owned() };
    unsafe {
        qiwo_input_format_free_string(result);
    }
    output
}

#[test]
fn c_abi_formats_utf8_commit_text() {
    assert_eq!(
        format_via_ffi("中文ABC123测试", None, None, true),
        "中文 ABC123 测试"
    );
}

#[test]
fn c_abi_accepts_null_optional_context() {
    assert_eq!(
        format_via_ffi(",中文测试,", None, None, true),
        ", 中文测试,"
    );
}

#[test]
fn c_abi_formats_available_context_boundaries() {
    assert_eq!(
        format_via_ffi("中文", Some("abc"), Some("123"), true),
        " 中文 "
    );
}

#[test]
fn c_abi_disabled_option_leaves_commit_text_unchanged() {
    assert_eq!(
        format_via_ffi("中文ABC123测试", Some("abc"), Some("中文"), false),
        "中文ABC123测试"
    );
}
