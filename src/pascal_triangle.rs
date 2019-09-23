#[allow(dead_code)]
pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    if num_rows == 0 {
        return vec![];
    } else if num_rows == 1 {
        return vec![vec![1]];
    }

    let mut generated = vec![vec![1], vec![1, 1]];

    for i in 1..(num_rows - 1) as usize {
        let previous_row = &generated[i];
        let mut new_row = vec![1];

        for j in 0..(previous_row.len() - 1) {
            new_row.push(previous_row[j] + previous_row[j + 1]);
        }

        new_row.push(1);
        generated.push(new_row);
    }

    generated
}

#[cfg(test)]
mod tests {
    use super::generate;

    #[test]
    fn basic() {
        let gen = generate(5);
        let sol = vec![
            vec![1],
            vec![1, 1],
            vec![1, 2, 1],
            vec![1, 3, 3, 1],
            vec![1, 4, 6, 4, 1],
        ];

        assert_eq!(gen, sol);
    }
}
