pub struct Stack<T: Clone> {
    elements: Vec<T>,
}

impl<T: Clone> Stack<T> {
    pub fn new() -> Self {
        Stack {
            elements: Vec::new(),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn push(&mut self, item: T) {
        self.elements.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    pub fn peek(&self) -> Option<T> {
        if self.elements.len() == 0 {
            return None;
        }
        Some(self.elements[self.elements.len() - 1].clone())
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}
