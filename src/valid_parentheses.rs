#[allow(dead_code)]
pub fn is_valid(s: String) -> bool {
    let mut stack = vec![];

    for paren in s.chars() {
        match paren {
            '(' | '[' | '{' => stack.push(paren),
            ')' => {
                if stack.pop().unwrap_or('!') != '(' {
                    return false;
                }
            }
            ']' => {
                if stack.pop().unwrap_or('!') != '[' {
                    return false;
                }
            }
            '}' => {
                if stack.pop().unwrap_or('!') != '{' {
                    return false;
                }
            }
            _ => {}
        }
    }

    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::is_valid;

    #[test]
    fn basic_1() {
        let s = String::from("()");
        assert_eq!(is_valid(s), true);
    }

    #[test]
    fn basic_2() {
        let s = String::from("()[]{}");
        assert_eq!(is_valid(s), true);
    }

    #[test]
    fn basic_3() {
        let s = String::from("(]");
        assert_eq!(is_valid(s), false);
    }

    #[test]
    fn basic_4() {
        let s = String::from("([)]");
        assert_eq!(is_valid(s), false);
    }

    #[test]
    fn basic_5() {
        let s = String::from("{[]}");
        assert_eq!(is_valid(s), true);
    }
}
