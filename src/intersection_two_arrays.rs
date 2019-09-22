use std::collections::HashMap;

#[allow(dead_code)]
pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut nums: Vec<i32> = vec![];
    let mut map: HashMap<i32, i32> = HashMap::new();

    nums1.iter().for_each(|num| {
        map.entry(*num).and_modify(|e| *e += 1).or_insert(1);
    });

    nums2.iter().for_each(|num| {
        if map.get(num).unwrap_or(&0).clone() != 0 {
            nums.push(*num);
            map.entry(*num).and_modify(|e| *e -= 1);
        }
    });

    nums
}

#[cfg(test)]
mod tests {
    use super::intersect;

    #[test]
    fn basic_1() {
        let nums1 = vec![1, 2, 2, 1];
        let nums2 = vec![2, 2];
        assert_eq!(intersect(nums1, nums2), vec![2, 2]);
    }

    #[test]
    fn basic_2() {
        let nums1 = vec![4, 9, 5];
        let nums2 = vec![9, 4, 9, 8, 4];
        assert_eq!(intersect(nums1, nums2), vec![9, 4]);
    }
}
