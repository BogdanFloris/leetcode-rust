#[allow(dead_code)]
pub fn climb_stairs(n: i32) -> i32 {
    match n {
        1 => 1,
        2 => 2,
        k => (2..k).fold((1, 2), |(e0, e1), _| (e1, e0 + e1)).1,
    }
}

#[cfg(test)]
mod tests {
    use super::climb_stairs;

    #[test]
    fn basic_1() {
        let n = 2;
        assert_eq!(climb_stairs(n), 2);
    }

    #[test]
    fn basic_2() {
        let n = 3;
        assert_eq!(climb_stairs(n), 3);
    }
}
