#[allow(dead_code)]
pub fn title_to_number(s: String) -> i32 {
    s.chars()
        .rev()
        .enumerate()
        .map(|(index, c)| ((c as u8 - 64) % 27) as i32 * 26_i32.pow(index as u32))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::title_to_number;

    #[test]
    fn basic_1() {
        let s = String::from("A");
        assert_eq!(title_to_number(s), 1);
    }

    #[test]
    fn basic_2() {
        let s = String::from("AB");
        assert_eq!(title_to_number(s), 28);
    }

    #[test]
    fn basic_3() {
        let s = String::from("ZY");
        assert_eq!(title_to_number(s), 701);
    }
}
