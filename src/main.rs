use json::parse_json;

fn main() {
    let json = r#"
    {
        "name": "John Doe",
        "age": 43,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    }"#;

    println!("{:#?}", parse_json(json));
}
