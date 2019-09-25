struct MinStack {
    stack: Vec<i32>,
    min: Option<i32>,
}

#[allow(dead_code)]
impl MinStack {
    fn new() -> Self {
        MinStack {
            stack: vec![],
            min: None,
        }
    }

    fn push(&mut self, x: i32) {
        self.stack.push(x);
        if let Some(min) = self.min {
            if x < min {
                self.min = Some(x);
            }
        }
    }

    fn pop(&mut self) {
        let x = self.stack.pop().unwrap();
        if let Some(min) = self.min {
            if x == min {
                self.min = None;
            }
        }
    }

    fn top(&self) -> i32 {
        self.stack[self.stack.len() - 1]
    }

    fn get_min(&mut self) -> i32 {
        self.min.unwrap_or_else(|| {
            let min = *self.stack.iter().min().unwrap();
            self.min = Some(min);
            min
        })
    }
}

#[cfg(test)]
mod tests {
    use super::MinStack;

    #[test]
    fn example() {
        let mut min_stack = MinStack::new();
        min_stack.push(-2);
        min_stack.push(0);
        min_stack.push(-3);
        assert_eq!(min_stack.get_min(), -3);
        min_stack.pop();
        assert_eq!(min_stack.top(), 0);
        assert_eq!(min_stack.get_min(), -2);
    }
}
