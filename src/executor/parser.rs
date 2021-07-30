pub fn line_to_cmds(line: &String) -> Vec<String> {
    let mut result = Vec::new();
    let mut token = String::new();
    let mut has_backslash = false;
    let mut separator: Option<char> = None; // example: " ' `
    let mut skip_next = false;

    for (i, char) in line.chars().enumerate() {
        if skip_next {
            skip_next = false;
            continue;
        }

        if has_backslash {
            token.push(char);
            has_backslash = false;
            continue;
        }

        if separator.is_some() && separator.unwrap() == char  {
            separator = None;
        }

        if separator.is_none() {
            if char == '#' {
                break;
            }

            if char == '\\' {
                has_backslash = true;
            }

            if char == ';' {
                // push token
                let result_of_token = token.trim();
                if !result_of_token.is_empty() {
                    result.push(String::from(result_of_token))
                }
                token = String::new();
                // end push token

                result.push(String::from(";"));
                continue;
            }

            if char == '&' || char == '|' {
                let next_is_same_char = match line.chars().nth(i + 1) {
                    Some(next_char) => char == next_char,
                    None => false
                };

                if next_is_same_char {
                    // push token
                    let result_of_token = token.trim();
                    if !result_of_token.is_empty() {
                        result.push(String::from(result_of_token))
                    }
                    token = String::new();
                    // end push token

                    skip_next = true;

                    let mut sign: String  = String::from(char);
                    sign.push(char);

                    result.push(sign);
                    continue;
                }
            }

            if is_separator(char) {
                separator = Some(char);
            }
        }

        token.push(char);
    }


    let result_of_token = token.trim();
    if !result_of_token.is_empty() {
        result.push(String::from(result_of_token))
    }

    return result;
}

fn is_separator(char: char) -> bool {
    return char == '\'' || char == '"' || char == '`'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_to_cmds() {
        let expected_a_b = vec![
            ("ls", vec!["ls"]),
            ("ls &", vec!["ls &"]),
            ("ls -lh", vec!["ls -lh"]),
            (
                "awk -F \" \" '{print $1}' README.md",
                vec!["awk -F \" \" '{print $1}' README.md"],
            ),
            ("ls | wc", vec!["ls | wc"]),
            ("echo #foo; echo bar", vec!["echo"]),
            ("echo foo; echo bar", vec!["echo foo", ";", "echo bar"]),
            ("echo 'foo; echo bar'", vec!["echo 'foo; echo bar'"]),
            ("echo \"foo; echo bar\"", vec!["echo \"foo; echo bar\""]),
            ("echo `foo; echo bar`", vec!["echo `foo; echo bar`"]),
            ("echo foo && echo bar", vec!["echo foo", "&&", "echo bar"]),
            (
                "echo foo && echo bar && echo baz",
                vec!["echo foo", "&&", "echo bar", "&&", "echo baz"],
            ),
            ("echo foo || echo bar", vec!["echo foo", "||", "echo bar"]),
            (
                "echo foo && echo bar; echo end",
                vec!["echo foo", "&&", "echo bar", ";", "echo end"],
            ),
            ("echo \"\\\"\"", vec!["echo \"\\\"\""]),
            (
                "man awk| awk -F \"[ ,.\\\"]+\" 'foo' |sort -k2nr|head",
                vec!["man awk| awk -F \"[ ,.\\\"]+\" 'foo' |sort -k2nr|head"],
            ),
            (";", vec![";"]),
            ("||", vec!["||"]),
            ("&&", vec!["&&"]),
            ("ls foo\\#bar", vec!["ls foo\\#bar"]),
            ("ls \\|\\|foo", vec!["ls \\|\\|foo"]),
        ];

        for (line, result) in expected_a_b {
            let expected_cmd = line_to_cmds(&String::from(line));

            assert_vec_str_equals(expected_cmd, result)
        }
    }

    fn assert_vec_str_equals(a: Vec<String>, b: Vec<&str>) {
        assert_eq!(a.len(), b.len(), "Vector {:?} of size {} and {:?} of size {} should have the same size", a, a.len(), b, b.len());

        for (i, item) in a.iter().enumerate() {
            assert_eq!(item, b[i])
        }
    }



}
