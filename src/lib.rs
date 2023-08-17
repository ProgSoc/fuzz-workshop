//! This is a basic JSON parser to demonstrate how fuzz testing works.

#[derive(Debug, Clone, Copy)]
struct Cursor<'a> {
    data: &'a str,
    pos: usize,
}

impl<'a> Cursor<'a> {
    fn new(data: &'a str) -> Self {
        Self { data, pos: 0 }
    }

    fn peek(&self) -> Option<char> {
        self.data[self.pos..].chars().nth(0)
    }

    fn consume(&mut self) -> Option<char> {
        let c = self.peek()?;
        self.pos += c.len_utf8();
        Some(c)
    }

    fn consume_n_chars(&mut self, n: usize) -> Option<&'a str> {
        let old_pos = self.pos;

        for _ in 0..n {
            self.consume()?;
        }

        Some(&self.data[old_pos..self.pos])
    }

    fn skip_whitespaces(&mut self) {
        while let Some(c) = self.peek() {
            if !c.is_whitespace() {
                break;
            }
            self.consume();
        }
    }

    fn consume_str(&mut self, s: &str) -> Option<&'a str> {
        if self.data[self.pos..].starts_with(s) {
            let old_pos = self.pos;
            self.pos += s.len();
            Some(&self.data[old_pos..self.pos])
        } else {
            None
        }
    }

    fn create_error(&self, kind: JsonParseErrorKind) -> JsonParseError {
        JsonParseError {
            kind,
            pos: self.pos,
        }
    }
}

#[derive(Debug)]
pub enum JsonType {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonType>),
    Object(Vec<(String, JsonType)>),
}

#[derive(Debug)]
pub struct JsonParseError {
    pub kind: JsonParseErrorKind,
    pub pos: usize,
}

#[derive(Debug)]
pub enum JsonParseErrorKind {
    UnexpectedChar(char),
    UnexpectedEnd,
    InvalidNumber,
    InvalidEscape,
    InvalidUnicode,
}

pub fn parse_json(str: &str) -> Result<JsonType, JsonParseError> {
    let mut cursor = Cursor::new(str);
    parse_value(&mut cursor)
}

fn parse_value(cursor: &mut Cursor) -> Result<JsonType, JsonParseError> {
    cursor.skip_whitespaces();

    if cursor.consume_str("null").is_some() {
        return Ok(JsonType::Null);
    } else if cursor.consume_str("true").is_some() {
        return Ok(JsonType::Bool(true));
    } else if cursor.consume_str("false").is_some() {
        return Ok(JsonType::Bool(false));
    }

    let next_char = cursor
        .peek()
        .ok_or_else(|| cursor.create_error(JsonParseErrorKind::UnexpectedEnd))?;

    let _large_stack_obj = [0; 100000];
    std::hint::black_box(_large_stack_obj);

    if next_char == '"' {
        parse_string(cursor)
    } else if next_char == '[' {
        parse_array(cursor)
    } else if next_char == '{' {
        parse_object(cursor)
    } else if next_char == '-' || next_char.is_digit(10) {
        parse_number(cursor)
    } else {
        Err(cursor.create_error(JsonParseErrorKind::UnexpectedChar(next_char)))
    }
}

fn parse_string(cursor: &mut Cursor) -> Result<JsonType, JsonParseError> {
    cursor.consume(); // Consume the opening quote
    let mut string = String::new();

    loop {
        match cursor
            .peek()
            .ok_or_else(|| cursor.create_error(JsonParseErrorKind::UnexpectedEnd))?
        {
            '"' => {
                cursor.consume(); // Consume the closing quote
                return Ok(JsonType::String(string));
            }
            '\\' => {
                cursor.consume(); // Consume the escape character
                match cursor
                    .peek()
                    .ok_or_else(|| cursor.create_error(JsonParseErrorKind::UnexpectedEnd))?
                {
                    '"' => string.push('"'),
                    '\\' => string.push('\\'),
                    '/' => string.push('/'),
                    'b' => string.push('\x08'),
                    'f' => string.push('\x0C'),
                    'n' => string.push('\n'),
                    'r' => string.push('\r'),
                    't' => string.push('\t'),
                    'u' => {
                        cursor.consume(); // Consume the 'u'
                        let unicode = cursor.consume_n_chars(4).ok_or_else(|| {
                            cursor.create_error(JsonParseErrorKind::InvalidUnicode)
                        })?;
                        let unicode = u32::from_str_radix(unicode, 16)
                            .map_err(|_| cursor.create_error(JsonParseErrorKind::InvalidUnicode))?;
                        string.push(char::from_u32(unicode).ok_or_else(|| {
                            cursor.create_error(JsonParseErrorKind::InvalidUnicode)
                        })?);
                    }
                    _ => return Err(cursor.create_error(JsonParseErrorKind::InvalidEscape)),
                }
                cursor.consume(); // Consume the escaped character
            }
            c => {
                string.push(c);
                cursor.consume();
            }
        }
    }
}
fn parse_number(cursor: &mut Cursor) -> Result<JsonType, JsonParseError> {
    let mut number = String::new();

    while let Some(c) = cursor.peek() {
        if c.is_digit(10) || c == '.' {
            number.push(c);
            cursor.consume();
        } else {
            break;
        }
    }

    let number = number
        .parse::<f64>()
        .map_err(|_| cursor.create_error(JsonParseErrorKind::InvalidNumber))?;
    Ok(JsonType::Number(number))
}

fn parse_array(cursor: &mut Cursor) -> Result<JsonType, JsonParseError> {
    cursor.consume(); // Consume the opening bracket
    let mut array = Vec::new();

    loop {
        cursor.skip_whitespaces();

        if let Some(c) = cursor.peek() {
            if c == ']' {
                cursor.consume(); // Consume the closing bracket
                return Ok(JsonType::Array(array));
            }
        }

        let value = parse_value(cursor)?;
        array.push(value);

        cursor.skip_whitespaces();

        if let Some(c) = cursor.peek() {
            if c == ',' {
                cursor.consume(); // Consume the comma
            } else if c == ']' {
                cursor.consume(); // Consume the closing bracket
                return Ok(JsonType::Array(array));
            } else {
                return Err(cursor.create_error(JsonParseErrorKind::UnexpectedChar(c)));
            }
        } else {
            return Err(cursor.create_error(JsonParseErrorKind::UnexpectedEnd));
        }
    }
}

fn parse_object(cursor: &mut Cursor) -> Result<JsonType, JsonParseError> {
    cursor.consume(); // Consume the opening brace
    let mut object = Vec::new();

    loop {
        cursor.skip_whitespaces();

        if let Some(c) = cursor.peek() {
            if c == '}' {
                cursor.consume(); // Consume the closing brace
                return Ok(JsonType::Object(object));
            }
        }

        let key = match parse_string(cursor)? {
            JsonType::String(s) => s,
            _ => return Err(cursor.create_error(JsonParseErrorKind::UnexpectedChar('"'))),
        };

        cursor.skip_whitespaces();

        if cursor.consume_str(":").is_none() {
            return Err(cursor.create_error(JsonParseErrorKind::UnexpectedChar(':')));
        }

        cursor.skip_whitespaces();

        let value = parse_value(cursor)?;
        object.push((key, value));

        cursor.skip_whitespaces();

        if let Some(c) = cursor.peek() {
            if c == ',' {
                cursor.consume(); // Consume the comma
            } else if c == '}' {
                cursor.consume(); // Consume the closing brace
                return Ok(JsonType::Object(object));
            } else {
                return Err(cursor.create_error(JsonParseErrorKind::UnexpectedChar(c)));
            }
        } else {
            return Err(cursor.create_error(JsonParseErrorKind::UnexpectedEnd));
        }
    }
}
