pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new(&self) -> Self {
        Stack { items: vec![] }
    }

    pub fn push(&mut self, stack_item: T) {
        self.items.push(stack_item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
}
