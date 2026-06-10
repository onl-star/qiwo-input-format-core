use qiwo_input_format::{format_commit_text, CommitContext, FormatOptions};

fn enabled(commit_text: &str) -> String {
    format_commit_text(
        commit_text,
        CommitContext::default(),
        FormatOptions::default(),
    )
}

fn disabled(commit_text: &str) -> String {
    format_commit_text(
        commit_text,
        CommitContext::default(),
        FormatOptions {
            auto_spacing_enabled: false,
        },
    )
}

#[test]
fn default_options_enable_auto_spacing() {
    assert_eq!(enabled("中文ABC测试"), "中文 ABC 测试");
}

#[test]
fn disabled_options_leave_commit_text_unchanged() {
    assert_eq!(disabled("中文ABC123测试"), "中文ABC123测试");
}

#[test]
fn spaces_between_han_and_latin_in_both_directions() {
    assert_eq!(enabled("中文abc"), "中文 abc");
    assert_eq!(enabled("abc中文"), "abc 中文");
}

#[test]
fn spaces_between_han_and_ascii_digits_in_both_directions() {
    assert_eq!(enabled("中文123"), "中文 123");
    assert_eq!(enabled("123中文"), "123 中文");
}

#[test]
fn empty_commit_text_stays_empty() {
    assert_eq!(enabled(""), "");
    assert_eq!(disabled(""), "");
}
