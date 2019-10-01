use std::collections::HashSet;

#[allow(dead_code)]
pub fn is_happy(n: i32) -> bool {
    let number_set = HashSet::new();
    is_happy_helper(n, number_set)
}

fn is_happy_helper(mut n: i32, mut number_set: HashSet<i32>) -> bool {
    number_set.insert(n);
    let mut sum = 0;

    while n > 0 {
        sum += (n % 10).pow(2);
        n /= 10;
    }

    if sum == 1 {
        return true;
    }
    if number_set.contains(&sum) {
        return false;
    }

    is_happy_helper(sum, number_set)
}

#[cfg(test)]
mod tests {
    use super::is_happy;

    #[test]
    fn basic() {
        let n = 19;
        assert_eq!(is_happy(n), true);
    }
}
