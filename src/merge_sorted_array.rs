#[allow(dead_code)]
pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut m = m - 1;
    let mut n = n - 1;
    for i in (0..nums1.len()).rev() {
        if m >= 0 && n >= 0 {
            if nums1[m as usize] > nums2[n as usize] {
                nums1[i] = nums1[m as usize];
                m -= 1;
            } else {
                nums1[i] = nums2[n as usize];
                n -= 1;
            }
        } else if m > n {
            nums1[i] = nums1[m as usize];
            m -= 1;
        } else {
            nums1[i] = nums2[n as usize];
            n -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::merge;

    #[test]
    fn basic() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let mut nums2 = vec![2, 5, 6];
        merge(&mut nums1, 3, &mut nums2, 3);
        assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
    }
}
