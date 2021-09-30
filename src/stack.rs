use crate::list::*;

#[derive(Debug)]
pub struct Stack<T> {
    head: Link<T>,
    len: usize
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { head: None, len: 0 }
    }

    pub fn push(&mut self, val: T) {
        let node = Node { val, next: self.head.take() };

        self.head = Some(Box::from(node));
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.len -= 1;
            self.head = node.next;
            node.val
        })
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

#[cfg(test)]
mod test {
    use super::Stack;

    #[test]
    fn push_pop() {
        let mut stack: Stack<i32> = Stack::new();

        stack.push(3);
        stack.push(5);

        assert_eq!(stack.pop(), Some(5));
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn push_pop_len() {
        let mut stack: Stack<i32> = Stack::new();

        assert_eq!(stack.len(), 0);
        stack.push(3);
        assert_eq!(stack.len(), 1);
        stack.push(5);
        assert_eq!(stack.len(), 2);

        assert_eq!(stack.pop(), Some(5));
        assert_eq!(stack.len(), 1);
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.len(), 0);
        assert_eq!(stack.pop(), None);
        assert_eq!(stack.len(), 0);
    }
}