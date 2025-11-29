use std::{collections::HashMap};

#[derive(PartialEq, Debug)]
pub enum ParseError {
    UnexpectedChar { expected: String, found: char },
    UnexpectedEof,
    MalformedNumber,
    MalformedNull,
    MalformedBool,
    MalformedString,
    MalformedArray
}

#[derive(PartialEq, Debug)]
pub enum JsonType {
    JsonObject(HashMap<String, JsonType>),
    JsonArray(Vec<JsonType>),
    JsonString(String),
    JsonNumber(f64),
    JsonBool(bool),
    JsonNull
}

pub fn parse_array(data: &str) -> Result<(JsonType, &str), ParseError> {

    let mut rest = data.trim_start();
    let mut chars = rest.chars();
    match chars.next() {
        Some('[') => {},
        None => {return Err(ParseError::UnexpectedEof)},
        _ => {return Err(ParseError::MalformedArray)}
    }
    rest = rest.get(1..).unwrap();
    // Handle empty array
    match chars.next() {
        Some(']') => return Ok((JsonType::JsonArray(Vec::new()), rest.get(1..).unwrap())),
        None => {return Err(ParseError::UnexpectedEof)},
        _ => {}
    }

    let mut values = Vec::<JsonType>::new();
    loop {
        let value;
        match parse_value(rest) {
            Ok(r) => {(value, rest) = r},
            Err(err) => return Err(err)
        }
        values.push(value);

        // Check if there is a new entry or if array has ended
        rest = rest.trim_start();
        match rest.chars().next() {
            Some(']') => {return Ok((JsonType::JsonArray(values), rest.get(1..).unwrap()))},
            Some(',') => {},
            None => {return Err(ParseError::UnexpectedEof)},
            _ => {return Err(ParseError::MalformedArray)}
        }
        rest = rest.get(1..).unwrap();
    }
}

pub fn parse_string(data: &str) -> Result<(JsonType, &str), ParseError> {
    match parse_string_raw(data) {
        Ok(r) => {
            let j_str = JsonType::JsonString(r.0);
            return Ok((j_str, r.1))
        },
        Err(e) => { return Err(e) }
    }
}

pub fn parse_string_raw(data: &str) -> Result<(String, &str), ParseError> {
    let start ;
    match data.find('"') {
        Some(i) => {start = i+1;},
        None => {return Err(ParseError::MalformedString)}
    }
    let rest = data.get((start)..).unwrap();
    match rest.find('"') {
        Some(i) => {
            return Ok((
                data.get(start..(start+i)).unwrap().to_string(),
                data.get((start+i+1)..).unwrap()
            ))
        },
        None => {return Err(ParseError::MalformedString)}
    }
}

pub fn parse_bool(data: &str) -> Result<(JsonType, &str), ParseError> {
    if let Some(v) = data.get(..4) && v == "true" {
        return Ok((JsonType::JsonBool(true), data.get(4..).unwrap()))
    } else if let Some(v) = data.get(..5) && v == "false" {
        return Ok((JsonType::JsonBool(false), data.get(5..).unwrap()))
    } else {
        return Err(ParseError::MalformedBool)
    }
}

#[derive(PartialEq, Debug)]
enum NumberState {
    Parsing,
    ParsingAfterDecimal
}

pub fn parse_number(data: &str) -> Result<(JsonType, &str), ParseError> {
    let rest = data.trim_start();
    let mut state = NumberState::Parsing;
    let mut chars = rest.chars();
    let mut i = 0;

    loop {
        if let Some(c) = chars.next() {
            match c {
                '.' => {
                    match state {
                        NumberState::Parsing => {state = NumberState::ParsingAfterDecimal; i+=1}
                        NumberState::ParsingAfterDecimal => {return Err(ParseError::MalformedNumber)}
                    }
                }
                '-' => {
                    if i != 0 {
                        return Err(ParseError::MalformedNumber)
                    }
                    i += 1;
                }
                'e' | 'E' => {panic!("Haven't implemented scientific notation")}
                c if c.is_numeric() => {i += 1;},
                ',' | ']' | '}' => {break},
                c if c.is_whitespace() => {break},
                _ => return Err(ParseError::MalformedNumber)
            }
        } else {
            return Err(ParseError::UnexpectedEof)
        }
    }
    let ex = rest.get(..i);
    let value = rest.get(..i).unwrap().parse::<f64>().unwrap();
    return Ok((JsonType::JsonNumber(value), rest.get(i..).unwrap()))
}

pub fn parse_null(data: &str) -> Result<(JsonType, &str), ParseError> {
    if let Some(v) = data.get(..4) && v == "null" {
        return Ok((JsonType::JsonNull, data.get(4..).unwrap()))
    } else {
        return Err(ParseError::MalformedNull)
    }
}

pub fn parse_value(mut data: &str) -> Result<(JsonType, &str), ParseError> {
    data = data.trim_start();
    if let Some(c) = data.chars().next() {
        match c {
            '"' => {
                return parse_string(data)
            },
            '{' => {
                return parse_object(data);
            },
            '[' => {
                return parse_array(data);
            },
            'T' | 't' | 'F' | 'f' => {
                return parse_bool(data);
            },
            'n' => {
                return parse_null(data);
            },
            c if c.is_numeric() || c == '-' => {
                return parse_number(data);
            },
            _ => {
                return Err(ParseError::UnexpectedChar { expected: "\",{,[,T,t,F,f,n,is_numeric".to_string(), found: c });
            }
        }
    } else {
        return Err(ParseError::UnexpectedEof)
    }
}

//const JSON_TOKENS : str = "[{}[],:\"]";

#[derive(PartialEq, Debug)]
enum ObjectState {
    WaitingObject, // Whitespace until {
    StartedObject, // Whitespace until "
    CollectingKey, // Hold chars until "
    EndKey, // Whitespace until :
    WaitingValue, // Whitespace until ""
    EndValue, // Immediately need a , or a }
    Finished, // Done parsing
}

pub fn parse_object(data: &str) -> Result<(JsonType, &str), ParseError> {
    let mut output: HashMap<String, JsonType> = HashMap::new();
    let mut rest = data.trim_start();
    let mut chars = rest.chars();
    match chars.next() {
        Some('{') => {},
        None => {return Err(ParseError::UnexpectedEof)},
        c => {
            return Err(ParseError::UnexpectedChar { expected: "{".to_string(), found: c.unwrap() });
        }
    }
    match chars.next() {
        Some('}') => {return Ok((JsonType::JsonObject(HashMap::new()), rest.get(2..).unwrap()))},
        None => {return Err(ParseError::UnexpectedEof)},
        _ => {}
    }
    rest = rest.get(1..).unwrap();
    rest = rest.trim_start();

    loop {
        // Get key
        let key;
        match parse_string_raw(rest) {
            Ok(r) => {(key, rest) = r;},
            Err(e) => {return Err(e)}
        }

        // Get :
        rest = rest.trim_start();
        match rest.chars().next() {
            Some(':') => {},
            Some(c) => {
                return Err(ParseError::UnexpectedChar { expected: ":".to_string(), found: c });
            },
            None => {return Err(ParseError::UnexpectedEof)}
        }
        rest = rest.get(1..).unwrap();

        // Get value
        let value;
        match parse_value(rest) {
            Ok(r) => {(value, rest) = r;},
            Err(e) => {return Err(e)}
        }

        output.insert(key, value);

        rest = rest.trim_start();
        // Check for end of array
        match rest.chars().next() {
            Some('}') => {break},
            Some(',') => {},
            Some(c) => return Err(ParseError::UnexpectedChar { expected: "},,".to_string(), found: c}),
            None => return Err(ParseError::UnexpectedEof)
        }
    }

    return Ok((JsonType::JsonObject(output), rest.get(1..).unwrap()));
}