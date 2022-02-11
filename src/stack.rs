pub struct Stack<T>(Vec<T>);

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack(vec![])
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn push(&mut self, value: T) {
        self.0.push(value);
    }

    pub fn pop(&mut self) -> T {
        self.0.pop().expect("unable to pop from empty stack")
    }
}

#[cfg(test)]
mod test {
    use super::Stack;

    #[test]
    fn new() {
        let stack: Stack<usize> = Stack::new();
        assert!(stack.is_empty())
    }

    #[test]
    fn push() {
        let mut stack: Stack<usize> = Stack::new();
        stack.push(7);
        assert!(!stack.is_empty())
    }

    #[test]
    fn pop() {
        const N: usize = 7;
        let mut stack: Stack<usize> = Stack::new();

        stack.push(N);
        let v = stack.pop();
        assert_eq!(v, N);
    }

    #[test]
    #[should_panic(expected = "empty stack")]
    fn is_empty() {
        let mut stack: Stack<usize> = Stack::new();
        stack.pop();
    }
}
