use std::collections::HashMap;
use crate::hashmap_expansion::hashmap_extension::HashMapExpansion;

pub trait SmartStrings {
    fn lexer(&self) -> Vec<String>;
    fn fq_tokens(&self) -> HashMap<String, usize>;
    fn fq_chars(&self) -> HashMap<char, usize>;
}

impl SmartStrings for str {
    fn lexer(&self) -> Vec<String> {
        let mut tokens: Vec<String> = Vec::new();
        let mut buffer: String = String::new();

        for c in self.chars().collect::<Vec<char>>() {
            if c.is_whitespace() {
                if !buffer.is_empty() {
                    tokens.push(buffer.clone());
                    buffer = String::new();
                }
            } else if c == '=' {
                if !buffer.is_empty() {
                    tokens.push(buffer.clone());
                    buffer = String::new();
                }
                tokens.push(c.to_string());
            } else {
                buffer.push(c);
            }
        }
        if !buffer.is_empty() {
            tokens.push(buffer.clone());
        }
        return tokens;
    }
    fn fq_tokens(&self) -> HashMap<String, usize> {
        let mut fq_map: HashMap<String, usize> = HashMap::new();
        let tokens = self.lexer();

        for token in tokens {
            fq_map.increment(token.clone(), 1);
        }
        return fq_map;
    }
    fn fq_chars(&self) -> HashMap<char, usize> {
        let mut fq_map: HashMap<char, usize> = HashMap::with_capacity(30);

        for c in self.chars().collect::<Vec<char>>() {
            fq_map.increment(c.clone(), 1);
        }
        return fq_map;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_lexer() {
        let input_string: String = "Hello I am Silver and This is a test".to_string();
        let tokens: Vec<String> = input_string.lexer();
        let expected_result = vec!["Hello", "I", "am", "Silver", "and", "This", "is", "a", "test"];
        assert_eq!(tokens, expected_result, "[TEST]: tokens do not match expected result");
    }
    #[test]
    fn test_fq_tokens() {
        let input_string: String = "Hello this is a test string".to_string();
        let fq_result: HashMap<String, usize> = input_string.fq_tokens();
        let mut fq_expected: HashMap<String, usize> = HashMap::new();
        fq_expected.insert("Hello".to_string(), 1);
        fq_expected.insert("this".to_string(), 1);
        fq_expected.insert("is".to_string(), 1);
        fq_expected.insert("a".to_string(), 1);
        fq_expected.insert("test".to_string(), 1);
        fq_expected.insert("string".to_string(), 1);

        assert_eq!(fq_result, fq_expected, "[TEST]: Token frequencies do not match expected values");

    }

    #[test]
    fn test_fq_characters() {
        let input_string: String = "Hello this is a test string".to_string();
        let fq_result: HashMap<char, usize> = input_string.fq_chars();

        let mut fq_expected: HashMap<char, usize> = HashMap::new();

        fq_expected.insert(' ', 5);

        fq_expected.insert('H', 1);
        fq_expected.insert('e', 2);
        fq_expected.insert('l', 2);
        fq_expected.insert('o', 1);
        fq_expected.insert('t', 4);
        fq_expected.insert('h', 1);
        fq_expected.insert('i', 3);
        fq_expected.insert('s', 4);
        fq_expected.insert('a', 1);
        fq_expected.insert('r', 1);
        fq_expected.insert('n', 1);
        fq_expected.insert('g', 1);

        assert_eq!(fq_result, fq_expected, "[TEST]: Character frequencies do not match expected values");
    }
}
