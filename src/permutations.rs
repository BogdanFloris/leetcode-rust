#[allow(dead_code)]
pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    fn dfs(
        nums: &[i32],
        permutations: &mut Vec<Vec<i32>>,
        current: &mut Vec<i32>,
        covered_indices: &mut Vec<i32>,
    ) {
        if nums.len() == current.len() {
            permutations.push(current.clone());
        } else {
            for num in nums {
                if !covered_indices.contains(&num) {
                    current.push(*num);
                    covered_indices.push(*num);
                    dfs(nums, permutations, current, covered_indices);
                    covered_indices.pop();
                    current.pop();
                }
            }
        }
    }

    let mut permutations: Vec<Vec<i32>> = vec![];
    let mut current: Vec<i32> = vec![];
    let mut covered_indices: Vec<i32> = vec![];
    dfs(&nums, &mut permutations, &mut current, &mut covered_indices);

    permutations
}

#[cfg(test)]
mod tests {
    use super::permute;

    #[test]
    fn basic() {
        let nums = vec![1, 2, 3];
        assert_eq!(
            permute(nums),
            vec![
                [1, 2, 3],
                [1, 3, 2],
                [2, 1, 3],
                [2, 3, 1],
                [3, 1, 2],
                [3, 2, 1]
            ]
        )
    }
}
