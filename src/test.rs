#![cfg(test)]

use super::*;

#[test]
fn basic_parsing() {
    let json = r#"
    {
        "name": "John Doe",
        "age": 43,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ],
        "field1": null,
        "field2": true,
        "field3": false,
        "field4": {
            "foo": "bar",
        }
    }"#;

    assert_eq!(
        parse_json(json),
        Ok(JsonType::Object(vec![
            ("name".into(), JsonType::String("John Doe".into())),
            ("age".into(), JsonType::Number(43.0)),
            (
                "phones".into(),
                JsonType::Array(vec![
                    JsonType::String("+44 1234567".into()),
                    JsonType::String("+44 2345678".into()),
                ])
            ),
            ("field1".into(), JsonType::Null),
            ("field2".into(), JsonType::Bool(true)),
            ("field3".into(), JsonType::Bool(false)),
            (
                "field4".into(),
                JsonType::Object(vec![("foo".into(), JsonType::String("bar".into())),])
            )
        ]))
    );
}

#[test]
fn parse_complex_strings() {
    let json = r#"
    "foo\nbar\tbaz\"qux\\quux"
    "#;

    assert_eq!(
        parse_json(json),
        Ok(JsonType::String("foo\nbar\tbaz\"qux\\quux".into()))
    );
}

#[test]
fn unexpected_char_error() {
    let json = r#"
    {
        "name"; "John Doe",
    }"#;

    assert_eq!(
        parse_json(json),
        Err(JsonParseError {
            kind: JsonParseErrorKind::UnexpectedChar(';'),
            pos: 21,
        })
    );

    let json = r#"
    {
        a"name": "John Doe",
    }"#;

    assert_eq!(
        parse_json(json),
        Err(JsonParseError {
            kind: JsonParseErrorKind::UnexpectedChar('a'),
            pos: 15,
        })
    );
}

#[test]
fn unexpected_end_error() {
    let json = r#"
    {
        "name": "John Doe",
    "#;

    assert_eq!(
        parse_json(json),
        Err(JsonParseError {
            kind: JsonParseErrorKind::UnexpectedEnd,
            pos: 39,
        })
    );

    let json = r#"
    {
        "name": 
    "#;

    assert_eq!(
        parse_json(json),
        Err(JsonParseError {
            kind: JsonParseErrorKind::UnexpectedEnd,
            pos: 28,
        })
    );
}

#[test]
fn unvalid_unicode_encoding_error() {
    let json = r#"
    "\u{110000}"
    "#;

    assert_eq!(
        parse_json(json),
        Err(JsonParseError {
            kind: JsonParseErrorKind::InvalidUnicode,
            pos: 12,
        })
    );
}

#[test]
fn invalid_number_error() {
    let json = r#"
    123.456.789
    "#;

    assert_eq!(
        parse_json(json),
        Err(JsonParseError {
            kind: JsonParseErrorKind::InvalidNumber,
            pos: 16,
        })
    );
}

#[test]
fn invalid_escape_error() {
    let json = r#"
    "\q"
    "#;

    assert_eq!(
        parse_json(json),
        Err(JsonParseError {
            kind: JsonParseErrorKind::InvalidEscape,
            pos: 7,
        })
    );
}
