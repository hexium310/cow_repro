#![allow(dead_code)]
use std::borrow::Cow;

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
        return Cow::Borrowed(&s.0); // => &s.0は&StringだからCow<'_, String>。ここをas_str()に変えるとエラーなくなる（下に例あり）。
    }

    to_cow(&s.0)
}

// asyncなしだとエラーなし
fn no_repro_sync<'a>(s: &'a S) -> Cow<'a, str> {
    if s.0.is_empty(){
        return Cow::Borrowed(&s.0);
    }
    
    to_cow(&s.0)
}

async fn no_repro_as_str<'a>(s: &'a S) -> Cow<'a, str> {
    if s.0.is_empty(){
        return Cow::Borrowed(s.0.as_str());
    }

    to_cow(&s.0)
}
