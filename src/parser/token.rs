use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Or,        // ||
    And,       // &&
    Semicolon, // ;
    Async,     // &
    Pipeline,  // |
    Raw(String),
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Or => write!(f, "||"),
            Token::And => write!(f, "&&"),
            Token::Semicolon => write!(f, ";"),
            Token::Async => write!(f, "&"),
            Token::Pipeline => write!(f, "|"),
            Token::Raw(string) => write!(f, "{}", string),
        }
    }
}

// transform raw lines in to tokens
// >> tokenize_raw_line("a; b | c && d")
// vec!["a", ";", "|", "&&"]
pub fn tokenize_raw_line(line: &String) -> Vec<Token> {
    let mut result = Vec::new();
    let mut token = String::new();
    let mut has_backslash = false;
    let mut escaper: Option<char> = None; // example: " ' `
    let mut skip_next = false;

    for (i, char) in line.chars().enumerate() {
        let mut push_token_to_result = || {
            let result_of_token = token.trim();
            if !result_of_token.is_empty() {
                result.push(Token::Raw(String::from(result_of_token)));
                token = String::new();
            }
        };

        let is_next_char_same = || match line.chars().nth(i + 1) {
            Some(next_char) => char == next_char,
            None => false,
        };

        if skip_next {
            skip_next = false;
            continue;
        }

        if has_backslash {
            token.push(char);
            has_backslash = false;
            continue;
        }

        if escaper == Some(char) {
            escaper = None;
        }

        if escaper.is_none() {
            if char == '#' {
                break;
            }

            if char == '\\' {
                has_backslash = true;
            }

            if char == ';' {
                push_token_to_result();

                result.push(Token::Semicolon);
                continue;
            }

            if char == '&' {
                push_token_to_result();

                if is_next_char_same() {
                    skip_next = true;
                    result.push(Token::And);
                } else {
                    result.push(Token::Async);
                }

                continue;
            }

            if char == '|' {
                push_token_to_result();

                if is_next_char_same() {
                    skip_next = true;
                    result.push(Token::Or);
                } else {
                    result.push(Token::Pipeline);
                }

                continue;
            }

            if is_escaper(char) {
                escaper = Some(char);
            }
        }

        token.push(char);
    }

    let result_of_token = token.trim();
    if !result_of_token.is_empty() {
        result.push(Token::Raw(String::from(result_of_token)))
    }

    return result;
}

pub fn contains_unsupported_async_token(tokens: &Vec<Token>) -> bool {
    tokens.contains(&Token::Async)
}

fn is_escaper(char: char) -> bool {
    return char == '\'' || char == '"' || char == '`';
}

#[cfg(test)]
mod tests {
    use super::*;
    use Token::{And, Async, Or, Pipeline, Raw, Semicolon};

    #[test]
    fn test_tokenize_command() {
        let expected_a_b = vec![
            ("ls", vec![Raw(String::from("ls"))]),
            ("ls &", vec![Raw(String::from("ls")), Async]),
            ("ls -lh", vec![Raw(String::from("ls -lh"))]),
            (
                "awk -F \" \" '{print $1}' README.md",
                vec![Raw(String::from("awk -F \" \" '{print $1}' README.md"))],
            ),
            (
                "ls | wc",
                vec![Raw(String::from("ls")), Pipeline, Raw(String::from("wc"))],
            ),
            ("echo #foo; echo bar", vec![Raw(String::from("echo"))]),
            (
                "echo foo; echo bar",
                vec![
                    Raw(String::from("echo foo")),
                    Semicolon,
                    Raw(String::from("echo bar")),
                ],
            ),
            (
                "echo 'foo; echo bar'",
                vec![Raw(String::from("echo 'foo; echo bar'"))],
            ),
            (
                "echo \"foo; echo bar\"",
                vec![Raw(String::from("echo \"foo; echo bar\""))],
            ),
            (
                "echo `foo; echo bar`",
                vec![Raw(String::from("echo `foo; echo bar`"))],
            ),
            (
                "echo foo && echo bar",
                vec![
                    Raw(String::from("echo foo")),
                    And,
                    Raw(String::from("echo bar")),
                ],
            ),
            (
                "echo foo && echo bar && echo baz",
                vec![
                    Raw(String::from("echo foo")),
                    And,
                    Raw(String::from("echo bar")),
                    And,
                    Raw(String::from("echo baz")),
                ],
            ),
            (
                "echo foo || echo bar",
                vec![
                    Raw(String::from("echo foo")),
                    Or,
                    Raw(String::from("echo bar")),
                ],
            ),
            (
                "echo foo && echo bar; echo end",
                vec![
                    Raw(String::from("echo foo")),
                    And,
                    Raw(String::from("echo bar")),
                    Semicolon,
                    Raw(String::from("echo end")),
                ],
            ),
            ("echo \"\\\"\"", vec![Raw(String::from("echo \"\\\"\""))]),
            (
                "man awk| awk -F \"[ ,.\\\"]+\" 'foo' |sort -k2nr|head",
                vec![
                    Raw(String::from("man awk")),
                    Pipeline,
                    Raw(String::from("awk -F \"[ ,.\\\"]+\" 'foo' |sort -k2nr|head")),
                ],
            ),
            (";", vec![Semicolon]),
            ("||", vec![Or]),
            ("&&", vec![And]),
            ("ls foo\\#bar", vec![Raw(String::from("ls foo\\#bar"))]),
            ("ls \\|\\|foo", vec![Raw(String::from("ls \\|\\|foo"))]),
        ];

        for (line, result) in expected_a_b {
            let expected_cmd = tokenize_raw_line(&String::from(line));

            assert_vec_token_equals(expected_cmd, result)
        }
    }

    fn assert_vec_token_equals(a: Vec<Token>, b: Vec<Token>) {
        assert_eq!(
            a.len(),
            b.len(),
            "Vector {:?} of size {} and {:?} of size {} should have the same size",
            a,
            a.len(),
            b,
            b.len()
        );

        for (i, item) in a.iter().enumerate() {
            assert_eq!(item, &b[i])
        }
    }
}
