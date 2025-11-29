//pub mod parser;
pub mod parser;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::parser::{
        JsonType,
        parse_string,
        parse_bool,
        parse_array,
        parse_null,
        parse_number,
        parse_object,
        parse_value,
    };

    use super::*;

    #[test]
    fn parse_string_test() {
        assert_eq!(
            parse_string("\"testing\""),
            Ok((
                JsonType::JsonString("testing".to_string()),
                ""
            ))
        )
    }

    #[test]
    fn parse_bool_test() {
        assert_eq!(
            parse_bool("true"),
            Ok((
                JsonType::JsonBool(true),
                ""
            ))
        );
        assert_eq!(
            parse_bool("false"),
            Ok((
                JsonType::JsonBool(false),
                ""
            ))
        )
    }

    #[test]
    fn parse_number_test() {
        assert_eq!(
            parse_number("123.5 "),
            Ok((
                JsonType::JsonNumber(123.5),
                " "
            ))
        );

        assert_eq!(
            parse_number("1230 "),
            Ok((
                JsonType::JsonNumber(1230.0),
                " "
            ))
        )
    }

    #[test]
    fn parse_null_test() {
        assert_eq!(
            parse_null("null"),
            Ok((
                JsonType::JsonNull,
                ""
            ))
        )
    }

    #[test]
    fn parse_value_test() {
        assert_eq!(
            parse_value("123.4 "),
            Ok((
                JsonType::JsonNumber(123.4),
                " "
            ))
        );

        assert_eq!(
            parse_value("null"),
            Ok((
                JsonType::JsonNull,
                ""
            ))
        );

        assert_eq!(
            parse_value("true"),
            Ok((
                JsonType::JsonBool(true),
                ""
            ))
        );
        assert_eq!(
            parse_value("false"),
            Ok((
                JsonType::JsonBool(false),
                ""
            ))
        );

        assert_eq!(
            parse_value("\"heyho\""),
            Ok((
                JsonType::JsonString("heyho".to_string()),
                ""
            ))
        )
    }

    #[test]
    fn parse_array_test() {
        assert_eq!(
            parse_array("[1, 2, 3, \"123\"]"),
            Ok((
                JsonType::JsonArray(vec![
                    JsonType::JsonNumber(1.0),
                    JsonType::JsonNumber(2.0),
                    JsonType::JsonNumber(3.0),
                    JsonType::JsonString("123".to_string()),
                ]),
                ""
            ))
        );

        assert_eq!(
            parse_array("[1, 2, [3, 4], \"abc\"]"),
            Ok((
                JsonType::JsonArray(vec![
                    JsonType::JsonNumber(1.0),
                    JsonType::JsonNumber(2.0),
                    JsonType::JsonArray(vec![JsonType::JsonNumber(3.0), JsonType::JsonNumber(4.0)]),
                    JsonType::JsonString("abc".to_string()),
                ]),
                ""
            ))
        )
    }

    #[test]
    fn parse_dict_test() {
        assert_eq!(
            parse_object("{\"1\": 123, \"2\": 234, \"3\": 345}"),
            Ok((
                JsonType::JsonObject(HashMap::from([
                    ("1".to_string(), JsonType::JsonNumber(123.0)),
                    ("2".to_string(), JsonType::JsonNumber(234.0)),
                    ("3".to_string(), JsonType::JsonNumber(345.0)),
                ])),
                ""
            ))
        )
    }

    #[test]
    fn parse_deeply_nested_arrays() {
        assert_eq!(
            parse_array("[[[[1, 2], [3, 4]], [[5, 6], [7, 8]]], [[[9, 10]]]]"),
            Ok((
                JsonType::JsonArray(vec![
                    JsonType::JsonArray(vec![
                        JsonType::JsonArray(vec![
                            JsonType::JsonArray(vec![
                                JsonType::JsonNumber(1.0),
                                JsonType::JsonNumber(2.0)
                            ]),
                            JsonType::JsonArray(vec![
                                JsonType::JsonNumber(3.0),
                                JsonType::JsonNumber(4.0)
                            ])
                        ]),
                        JsonType::JsonArray(vec![
                            JsonType::JsonArray(vec![
                                JsonType::JsonNumber(5.0),
                                JsonType::JsonNumber(6.0)
                            ]),
                            JsonType::JsonArray(vec![
                                JsonType::JsonNumber(7.0),
                                JsonType::JsonNumber(8.0)
                            ])
                        ])
                    ]),
                    JsonType::JsonArray(vec![
                        JsonType::JsonArray(vec![
                            JsonType::JsonArray(vec![
                                JsonType::JsonNumber(9.0),
                                JsonType::JsonNumber(10.0)
                            ])
                        ])
                    ])
                ]),
                ""
            ))
        )
    }

    #[test]
    fn parse_object_with_nested_arrays() {
        assert_eq!(
            parse_object("{\"matrix\": [[1, 2, 3], [4, 5, 6]], \"names\": [\"alice\", \"bob\"]}"),
            Ok((
                JsonType::JsonObject(HashMap::from([
                    ("matrix".to_string(), JsonType::JsonArray(vec![
                        JsonType::JsonArray(vec![
                            JsonType::JsonNumber(1.0),
                            JsonType::JsonNumber(2.0),
                            JsonType::JsonNumber(3.0)
                        ]),
                        JsonType::JsonArray(vec![
                            JsonType::JsonNumber(4.0),
                            JsonType::JsonNumber(5.0),
                            JsonType::JsonNumber(6.0)
                        ])
                    ])),
                    ("names".to_string(), JsonType::JsonArray(vec![
                        JsonType::JsonString("alice".to_string()),
                        JsonType::JsonString("bob".to_string())
                    ]))
                ])),
                ""
            ))
        )
    }

    #[test]
    fn parse_array_with_nested_objects() {
        assert_eq!(
            parse_array("[{\"id\": 1, \"name\": \"Alice\"}, {\"id\": 2, \"name\": \"Bob\"}]"),
            Ok((
                JsonType::JsonArray(vec![
                    JsonType::JsonObject(HashMap::from([
                        ("id".to_string(), JsonType::JsonNumber(1.0)),
                        ("name".to_string(), JsonType::JsonString("Alice".to_string()))
                    ])),
                    JsonType::JsonObject(HashMap::from([
                        ("id".to_string(), JsonType::JsonNumber(2.0)),
                        ("name".to_string(), JsonType::JsonString("Bob".to_string()))
                    ]))
                ]),
                ""
            ))
        )
    }

    #[test]
    fn parse_complex_nested_structure() {
        assert_eq!(
            parse_object("{\"users\": [{\"name\": \"Alice\", \"scores\": [10, 20, 30], \"meta\": {\"active\": true, \"level\": 5}}], \"count\": 1}"),
            Ok((
                JsonType::JsonObject(HashMap::from([
                    ("users".to_string(), JsonType::JsonArray(vec![
                        JsonType::JsonObject(HashMap::from([
                            ("name".to_string(), JsonType::JsonString("Alice".to_string())),
                            ("scores".to_string(), JsonType::JsonArray(vec![
                                JsonType::JsonNumber(10.0),
                                JsonType::JsonNumber(20.0),
                                JsonType::JsonNumber(30.0)
                            ])),
                            ("meta".to_string(), JsonType::JsonObject(HashMap::from([
                                ("active".to_string(), JsonType::JsonBool(true)),
                                ("level".to_string(), JsonType::JsonNumber(5.0))
                            ])))
                        ]))
                    ])),
                    ("count".to_string(), JsonType::JsonNumber(1.0))
                ])),
                ""
            ))
        )
    }

    #[test]
    fn parse_empty_structures() {
        assert_eq!(
            parse_array("[]"),
            Ok((JsonType::JsonArray(vec![]), ""))
        );
        
        assert_eq!(
            parse_object("{}"),
            Ok((JsonType::JsonObject(HashMap::new()), ""))
        );
        
        assert_eq!(
            parse_array("[[[]]]"),
            Ok((
                JsonType::JsonArray(vec![
                    JsonType::JsonArray(vec![
                        JsonType::JsonArray(vec![])
                    ])
                ]),
                ""
            ))
        )
    }

    #[test]
    fn parse_mixed_types_in_array() {
        assert_eq!(
            parse_array("[true, false, null, 42, \"hello\", [], {}]"),
            Ok((
                JsonType::JsonArray(vec![
                    JsonType::JsonBool(true),
                    JsonType::JsonBool(false),
                    JsonType::JsonNull,
                    JsonType::JsonNumber(42.0),
                    JsonType::JsonString("hello".to_string()),
                    JsonType::JsonArray(vec![]),
                    JsonType::JsonObject(HashMap::new())
                ]),
                ""
            ))
        )
    }

    #[test]
    fn parse_object_with_various_value_types() {
        assert_eq!(
            parse_object("{\"bool\": true, \"null\": null, \"num\": 3.14, \"str\": \"test\", \"arr\": [1, 2], \"obj\": {\"nested\": true}}"),
            Ok((
                JsonType::JsonObject(HashMap::from([
                    ("bool".to_string(), JsonType::JsonBool(true)),
                    ("null".to_string(), JsonType::JsonNull),
                    ("num".to_string(), JsonType::JsonNumber(3.14)),
                    ("str".to_string(), JsonType::JsonString("test".to_string())),
                    ("arr".to_string(), JsonType::JsonArray(vec![
                        JsonType::JsonNumber(1.0),
                        JsonType::JsonNumber(2.0)
                    ])),
                    ("obj".to_string(), JsonType::JsonObject(HashMap::from([
                        ("nested".to_string(), JsonType::JsonBool(true))
                    ])))
                ])),
                ""
            ))
        )
    }

    #[test]
    fn parse_strings_with_special_content() {
        assert_eq!(
            parse_array("[\"with spaces\", \"with,commas\", \"with:colons\", \"with[brackets]\"]"),
            Ok((
                JsonType::JsonArray(vec![
                    JsonType::JsonString("with spaces".to_string()),
                    JsonType::JsonString("with,commas".to_string()),
                    JsonType::JsonString("with:colons".to_string()),
                    JsonType::JsonString("with[brackets]".to_string())
                ]),
                ""
            ))
        )
    }

    #[test]
    fn parse_negative_and_decimal_numbers() {
        assert_eq!(
            parse_array("[-1, -3.14, 0, 0.5, 999.999]"),
            Ok((
                JsonType::JsonArray(vec![
                    JsonType::JsonNumber(-1.0),
                    JsonType::JsonNumber(-3.14),
                    JsonType::JsonNumber(0.0),
                    JsonType::JsonNumber(0.5),
                    JsonType::JsonNumber(999.999)
                ]),
                ""
            ))
        )
    }

    #[test]
    fn parse_whitespace_heavy_json() {
        assert_eq!(
            parse_array("[  1  ,  2  ,  3  ]"),
            Ok((
                JsonType::JsonArray(vec![
                    JsonType::JsonNumber(1.0),
                    JsonType::JsonNumber(2.0),
                    JsonType::JsonNumber(3.0)
                ]),
                ""
            ))
        );
        
        assert_eq!(
            parse_object("{  \"key\"  :  \"value\"  }"),
            Ok((
                JsonType::JsonObject(HashMap::from([
                    ("key".to_string(), JsonType::JsonString("value".to_string()))
                ])),
                ""
            ))
        )
    }

    #[test]
    fn parse_object_invalid_separator() {
        let result = parse_object(r#"{"a": 1 "b": 2}"#);  // Missing comma!
        println!("{result:?}");
        assert!(result.is_err());
    }

    // ============================================================================
    // MALFORMED JSON TESTS
    // ============================================================================

    // 1. Missing Commas
    #[test]
    fn test_array_missing_comma() {
        assert!(parse_array(r#"[1 2 3]"#).is_err());
        assert!(parse_array(r#"["a" "b"]"#).is_err());
    }

    #[test]
    fn test_object_missing_comma() {
        assert!(parse_object(r#"{"a": 1 "b": 2}"#).is_err());
    }

    // 2. Trailing Commas
    #[test]
    fn test_trailing_comma_array() {
        assert!(parse_array(r#"[1, 2, 3,]"#).is_err());
    }

    #[test]
    fn test_trailing_comma_object() {
        assert!(parse_object(r#"{"a": 1, "b": 2,}"#).is_err());
    }

    // 3. Multiple Commas
    #[test]
    fn test_multiple_commas() {
        assert!(parse_array(r#"[1,, 2]"#).is_err());
        assert!(parse_array(r#"[1,,, 2]"#).is_err());
    }

    // 4. Missing Closing Brackets/Braces
    #[test]
    fn test_unclosed_array() {
        assert!(parse_array(r#"[1, 2, 3"#).is_err());
        assert!(parse_array(r#"[1, [2, 3]"#).is_err());
    }

    #[test]
    fn test_unclosed_object() {
        assert!(parse_object(r#"{"a": 1"#).is_err());
        assert!(parse_object(r#"{"a": {"b": 2}"#).is_err());
    }

    // 5. Missing Opening Brackets/Braces
    #[test]
    fn test_missing_opening_bracket() {
        assert!(parse_array(r#"1, 2, 3]"#).is_err());
    }

    #[test]
    fn test_missing_opening_brace() {
        assert!(parse_object(r#""a": 1}"#).is_err());
    }

    // 6. Unquoted Strings
    #[test]
    fn test_unquoted_string() {
        assert!(parse_string(r#"hello"#).is_err());
    }

    #[test]
    fn test_unquoted_object_key() {
        assert!(parse_object(r#"{name: "Alice"}"#).is_err());
    }

    // 7. Single Quotes (JSON only allows double quotes)
    #[test]
    fn test_single_quotes() {
        assert!(parse_string(r#"'hello'"#).is_err());
        assert!(parse_object(r#"{'name': 'Alice'}"#).is_err());
    }

    // 8. Unclosed Strings
    #[test]
    fn test_unclosed_string() {
        assert!(parse_string(r#""hello"#).is_err());
    }

    // 9. Invalid Numbers
    #[test]
    fn test_malformed_numbers() {
        assert!(parse_number(r#"123.45.67"#).is_err()); // Multiple decimal points
        assert!(parse_number(r#"--123"#).is_err());      // Double negative
        assert!(parse_number(r#"123-"#).is_err());       // Trailing negative
        assert!(parse_number(r#".123"#).is_err());       // Leading decimal
        assert!(parse_number(r#"123."#).is_err());       // Trailing decimal
    }

    // 10. Wrong Case for Keywords
    #[test]
    fn test_wrong_case_keywords() {
        assert!(parse_null(r#"NULL"#).is_err());
        assert!(parse_null(r#"Null"#).is_err());
        assert!(parse_bool(r#"TRUE"#).is_err());
        assert!(parse_bool(r#"FALSE"#).is_err());
    }

    // 11. Missing Colons in Objects
    #[test]
    fn test_object_missing_colon() {
        assert!(parse_object(r#"{"name" "Alice"}"#).is_err());
        assert!(parse_object(r#"{"a" 1, "b" 2}"#).is_err());
    }

    // 12. Non-String Object Keys
    #[test]
    fn test_non_string_keys() {
        assert!(parse_object(r#"{123: "value"}"#).is_err());
        assert!(parse_object(r#"{true: "value"}"#).is_err());
        assert!(parse_object(r#"{null: "value"}"#).is_err());
    }

    // 13. Empty Input
    #[test]
    fn test_empty_input() {
        assert!(parse_value(r#""#).is_err());
        assert!(parse_array(r#""#).is_err());
        assert!(parse_object(r#""#).is_err());
    }

    // 14. Whitespace-Only Input
    #[test]
    fn test_whitespace_only() {
        assert!(parse_value(r#"   "#).is_err());
        assert!(parse_value(r#"
        
        "#).is_err());
    }

    // 15. Mixed Brackets/Braces
    #[test]
    fn test_mismatched_brackets() {
        assert!(parse_array(r#"[1, 2, 3}"#).is_err());
        assert!(parse_object(r#"{"a": 1]"#).is_err());
        assert!(parse_array(r#"[1, [2, 3}"#).is_err());
    }

    // 16. Leading Zeros (not allowed in JSON spec)
    #[test]
    fn test_leading_zeros() {
        assert!(parse_number(r#"0123"#).is_err());
        assert!(parse_number(r#"00"#).is_err());
    }

    // 17. Plus Sign (JSON doesn't allow explicit positive sign)
    #[test]
    fn test_plus_sign() {
        assert!(parse_number(r#"+123"#).is_err());
    }
}
