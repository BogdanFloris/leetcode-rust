#[allow(dead_code)]
pub fn fizz_buzz(n: i32) -> Vec<String> {
    let mut sol: Vec<String> = vec![];

    for i in 1..=n {
        if i % 3 == 0 && i % 5 == 0 {
            sol.push(String::from("FizzBuzz"));
        } else if i % 3 == 0 {
            sol.push(String::from("Fizz"));
        } else if i % 5 == 0 {
            sol.push(String::from("Buzz"));
        } else {
            sol.push(i.to_string())
        }
    }

    sol
}

#[cfg(test)]
mod tests {
    use super::fizz_buzz;

    #[test]
    fn basic() {
        let n = 15;
        let sol = fizz_buzz(n);
        assert_eq!(
            sol,
            vec![
                "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz",
                "13", "14", "FizzBuzz"
            ]
        );
    }
}
