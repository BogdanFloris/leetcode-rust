use std::collections::HashMap;

#[allow(dead_code)]
pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut map: HashMap<char, i32> = HashMap::new();

    s.chars().zip(t.chars()).for_each(|(s, t)| {
        if s == t {
            return;
        }
        map.entry(s).and_modify(|e| *e += 1).or_insert(1);
        map.entry(t).and_modify(|e| *e -= 1).or_insert(-1);
    });

    map.values().find(|&&e| e != 0).is_none()
}

#[cfg(test)]
mod tests {
    use super::is_anagram;

    #[test]
    fn basic_1() {
        let s = String::from("anagram");
        let t = String::from("nagaram");
        assert_eq!(is_anagram(s, t), true);
    }

    #[test]
    fn basic_2() {
        let s = String::from("rat");
        let t = String::from("car");
        assert_eq!(is_anagram(s, t), false);
    }
}
