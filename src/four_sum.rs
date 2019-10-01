use std::collections::HashMap;

#[allow(dead_code)]
pub fn four_sum_count(a: Vec<i32>, b: Vec<i32>, c: Vec<i32>, d: Vec<i32>) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();

    for a in &a {
        for b in &b {
            map.entry(a + b).and_modify(|e| *e += 1).or_insert(1);
        }
    }

    let mut counter = 0;
    for c in &c {
        for d in &d {
            let key = 0 - (c + d);
            if let Some(val) = map.get(&key) {
                counter += *val;
            }
        }
    }

    counter
}

#[cfg(test)]
mod tests {
    use super::four_sum_count;

    #[test]
    fn basic() {
        let a = vec![1, 2];
        let b = vec![-2, -1];
        let c = vec![-1, 2];
        let d = vec![0, 2];
        assert_eq!(four_sum_count(a, b, c, d), 2);
    }
}
