#![allow(dead_code)]
use std::borrow::Cow;

use regex_lite::Regex;

struct S(String);

fn main() {
    //repro(&S("test".to_string()));
}

fn to_cow<'a>(text: &'a str) -> Cow<'a, str> {
    Cow::Borrowed(text)
}

// asyncありだと27行目の式のcowで`rust-analyzer: expected Cow<'_, String>, found Cow<'_, str> [E0308]`
async fn repro<'a>(s: &'a S) -> Cow<'a, str> {
    if s.0.is_empty(){
        return Cow::Borrowed(&s.0);
    }

    let regex = Regex::new(".").unwrap();

    let replaced = regex.replace_all(&s.0, "");
    let cow = to_cow(&s.0);
    match replaced {
        Cow::Borrowed(borrowed) if borrowed.len() == s.0.len() => cow,
        Cow::Borrowed(borrowed) => Cow::Owned(borrowed.to_owned()),
        Cow::Owned(owned) => Cow::Owned(owned),
    }
}

// asyncなしだとエラーなし
fn no_repro<'a>(s: &'a S) -> Cow<'a, str> {
    if s.0.is_empty(){
        return Cow::Borrowed(&s.0);
    }

    let regex = Regex::new(".").unwrap();

    let replaced = regex.replace_all(&s.0, "");
    let cow = to_cow(&s.0);
    match replaced {
        Cow::Borrowed(borrowed) if borrowed.len() == s.0.len() => cow,
        Cow::Borrowed(borrowed) => Cow::Owned(borrowed.to_owned()),
        Cow::Owned(owned) => Cow::Owned(owned),
    }
}
