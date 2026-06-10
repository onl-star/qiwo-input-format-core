use qiwo_input_format::{CommitContext, FormatOptions, format_commit_text};

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

#[test]
fn existing_whitespace_prevents_duplicate_spaces() {
    assert_eq!(enabled("中文 abc"), "中文 abc");
    assert_eq!(enabled("abc 中文"), "abc 中文");
    assert_eq!(enabled("中文 123"), "中文 123");
    assert_eq!(enabled("123 中文"), "123 中文");
    assert_eq!(enabled(", 中文"), ", 中文");
}

#[test]
fn latin_letters_and_digits_stay_grouped() {
    assert_eq!(enabled("ABC123"), "ABC123");
    assert_eq!(enabled("v1"), "v1");
    assert_eq!(enabled("中文ABC123测试"), "中文 ABC123 测试");
}

#[test]
fn half_width_punctuation_before_han_gets_following_space_only() {
    assert_eq!(enabled(",中文测试,"), ", 中文测试,");
    assert_eq!(enabled(":中文"), ": 中文");
    assert_eq!(enabled("中文,"), "中文,");
}

#[test]
fn excluded_character_classes_do_not_trigger_spacing() {
    assert_eq!(enabled("あ中文"), "あ中文");
    assert_eq!(enabled("한中文"), "한中文");
    assert_eq!(enabled("😀中文"), "😀中文");
    assert_eq!(enabled("Ａ中文"), "Ａ中文");
    assert_eq!(enabled("中文Ａ"), "中文Ａ");
}
