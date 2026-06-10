# qiwo-input-format-core

Shared commit-text formatter for Qiwo frontends.

The formatter is intentionally small: it receives the text that a frontend is
about to commit, optional surrounding text, and an enabled flag, then returns
the commit text that should be inserted.

## Rules

- Enabled by default at each frontend setting layer.
- Insert one ordinary space between Han characters and ASCII letters/digits.
- Keep ASCII letter/digit groups together, such as `ABC123` and `v1`.
- Do not duplicate existing whitespace.
- Insert one ordinary space after half-width punctuation followed by Han text.
- Do not insert a space before half-width punctuation after Han text.
- Do not add spaces only because kana, Hangul, emoji, or full-width symbols are
  adjacent to another character.
- When before/after cursor text is available, apply the same boundary rule at
  the edge of the committed text.

## Rust API

```rust
use qiwo_input_format::{format_commit_text, CommitContext, FormatOptions};

let formatted = format_commit_text(
    "中文ABC123测试",
    CommitContext::default(),
    FormatOptions::default(),
);
assert_eq!(formatted, "中文 ABC123 测试");
```

## C ABI

Include `qiwo-input-format/include/qiwo_input_format.h` and link
`qiwo_input_format`.

```c
QiwoInputFormatOptions options = {
  .auto_spacing_enabled = true,
};
char *formatted = qiwo_input_format_commit_text(
    "中文ABC123测试", NULL, NULL, options);
qiwo_input_format_free_string(formatted);
```

All C strings are UTF-8 and null terminated. `before_cursor` and `after_cursor`
may be `NULL`. The returned string must be released with
`qiwo_input_format_free_string`.

## Android JNI

The crate exports:

```text
Java_com_qiwo_inputformat_QiwoInputFormat_nativeFormatCommitText
```

Android frontends should keep the Kotlin/Java wrapper thin: load
`qiwo_input_format`, pass commit text and optional cursor context, and commit
the returned text through the existing centralized commit path.

## Platform Responsibilities

Frontends are responsible for:

- Reading their local `input/auto_commit_spacing` preference, defaulting to
  true.
- Passing optional cursor context when the platform can provide it.
- Calling the formatter only after candidate selection produces commit text and
  before inserting text into the target application.
- Keeping candidate text, preedit text, input codes, dictionaries, and learning
  data unchanged.

