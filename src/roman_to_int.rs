use std::collections::HashMap;

#[allow(dead_code)]
pub fn roman_to_int(s: String) -> i32 {
    let mut roman_map: HashMap<char, i32> = HashMap::with_capacity(7);
    roman_map.insert('I', 1);
    roman_map.insert('V', 5);
    roman_map.insert('X', 10);
    roman_map.insert('L', 50);
    roman_map.insert('C', 100);
    roman_map.insert('D', 500);
    roman_map.insert('M', 1000);

    let mut sol = 0;
    let chars = s.as_bytes();

    for i in 0..(s.len() - 1) {
        if roman_map.get(&(chars[i] as char)).unwrap()
            < roman_map.get(&(chars[i + 1] as char)).unwrap()
        {
            sol -= *roman_map.get(&(chars[i] as char)).unwrap();
        } else {
            sol += *roman_map.get(&(chars[i] as char)).unwrap();
        }
    }

    sol + *roman_map.get(&(chars[s.len() - 1] as char)).unwrap()
}

#[cfg(test)]
mod tests {
    use super::roman_to_int;

    #[test]
    fn basic_1() {
        let s = String::from("III");
        assert_eq!(roman_to_int(s), 3);
    }

    #[test]
    fn basic_2() {
        let s = String::from("IV");
        assert_eq!(roman_to_int(s), 4);
    }

    #[test]
    fn basic_3() {
        let s = String::from("IX");
        assert_eq!(roman_to_int(s), 9);
    }

    #[test]
    fn basic_4() {
        let s = String::from("LVIII");
        assert_eq!(roman_to_int(s), 58);
    }

    #[test]
    fn basic_5() {
        let s = String::from("MCMXCIV");
        assert_eq!(roman_to_int(s), 1994);
    }
}
