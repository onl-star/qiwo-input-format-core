use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::{jboolean, jstring};

use crate::{CommitContext, FormatOptions, format_commit_text};

/// JNI entry point for Android.
///
/// Java signature:
/// `com.qiwo.inputformat.QiwoInputFormat.nativeFormatCommitText(String, String?, String?, boolean) -> String`
#[unsafe(no_mangle)]
pub extern "system" fn Java_com_qiwo_inputformat_QiwoInputFormat_nativeFormatCommitText(
    mut env: JNIEnv,
    _class: JClass,
    commit_text: JString,
    before_cursor: JString,
    after_cursor: JString,
    enabled: jboolean,
) -> jstring {
    let commit_text = jstring_to_option(&mut env, &commit_text).unwrap_or_default();
    let before_cursor = jstring_to_option(&mut env, &before_cursor);
    let after_cursor = jstring_to_option(&mut env, &after_cursor);

    let formatted = format_commit_text(
        &commit_text,
        CommitContext {
            before_cursor: before_cursor.as_deref(),
            after_cursor: after_cursor.as_deref(),
        },
        FormatOptions {
            auto_spacing_enabled: enabled != 0,
        },
    );

    env.new_string(formatted)
        .map(|value| value.into_raw())
        .unwrap_or(std::ptr::null_mut())
}

fn jstring_to_option(env: &mut JNIEnv, value: &JString) -> Option<String> {
    if value.is_null() {
        return None;
    }

    env.get_string(value).ok().map(Into::into)
}
