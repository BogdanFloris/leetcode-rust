use std::collections::HashMap;

#[allow(dead_code)]
pub fn first_uniq_char(s: String) -> i32 {
    let mut map: HashMap<char, bool> = HashMap::new();
    let mut index_store: Vec<usize> = Vec::new();

    s.chars().enumerate().for_each(|(i, c)| {
        index_store.push(i);
        map.entry(c).and_modify(|e| *e = false).or_insert(true);
    });

    let s_bytes = s.as_bytes();

    for index in index_store {
        if map.get(&(s_bytes[index] as char)).unwrap().clone() {
            return index as i32;
        }
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::first_uniq_char;

    #[test]
    fn basic_1() {
        let s = String::from("leetcode");
        assert_eq!(first_uniq_char(s), 0)
    }

    #[test]
    fn basic_2() {
        let s = String::from("loveleetcode");
        assert_eq!(first_uniq_char(s), 2)
    }

    #[test]
    fn basic_3() {
        let s = String::from("ee");
        assert_eq!(first_uniq_char(s), -1)
    }
}
