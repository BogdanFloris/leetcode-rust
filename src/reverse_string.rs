#[allow(dead_code)]
pub fn reverse_string(s: &mut Vec<char>) {
    let length = s.len();
    for index in 0..s.len() / 2 {
        let temp = s[index];
        s[index] = s[length - index - 1];
        s[length - index - 1] = temp;
    }
}

#[cfg(test)]
mod tests {
    use super::reverse_string;

    #[test]
    fn basic() {
        let mut s = vec!['h', 'e', 'l', 'l', 'o'];
        reverse_string(&mut s);
        assert_eq!(s, vec!['o', 'l', 'l', 'e', 'h']);
    }
}
