use rand::prelude::*;

struct Shuffler {
    nums: Vec<i32>,
    rng: ThreadRng,
}

impl Shuffler {
    #[allow(dead_code)]
    fn new(nums: Vec<i32>) -> Self {
        Shuffler {
            nums,
            rng: rand::thread_rng(),
        }
    }

    /// Resets the array to its original configuration and return it.
    #[allow(dead_code)]
    fn reset(&self) -> Vec<i32> {
        self.nums.clone()
    }

    /// Returns a random shuffling of the array.
    #[allow(dead_code)]
    fn shuffle(&mut self) -> Vec<i32> {
        let mut shuffled = self.nums.clone();
        shuffled.shuffle(&mut self.rng);
        shuffled
    }
}

#[cfg(test)]
mod tests {
    use super::Shuffler;

    #[test]
    fn basic() {
        let nums = vec![1, 2, 3];
        let permutations = vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1],
        ];
        let mut shuffler = Shuffler::new(nums);
        assert_eq!(shuffler.reset(), vec![1, 2, 3]);
        assert_eq!(permutations.contains(&shuffler.shuffle()), true);
    }
}
