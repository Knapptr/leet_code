/*Implement a first in first out (FIFO) queue using only two stacks.
The implemented queue should support all the functions of a normal queue (push, peek, pop, and empty).

Implement the MyQueue class:

    void push(int x) Pushes element x to the back of the queue.
    int pop() Removes the element from the front of the queue and returns it.
    int peek() Returns the element at the front of the queue.
    boolean empty() Returns true if the queue is empty, false otherwise.

Notes:

    You must use only standard operations of a stack,
    which means only push to top, peek/pop from top, size, and is empty operations are valid.
    Depending on your language, the stack may not be supported natively.
    You may simulate a stack using a list or deque (double-ended queue) as long as you use only a stack's standard operations.
*/
struct MyQueue {
    stack_one: Vec<i32>,
    stack_two: Vec<i32>,
    first_in_one: bool,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {
    fn new() -> Self {
        Self {
            stack_one: Vec::new(),
            stack_two: Vec::new(),
            first_in_one: false,
        }
    }

    /*

    if one has len 0, push to it, otherwise push to 2

        */
    fn push(&mut self, x: i32) {
        if self.first_in_one {
            self.stack_two.push(x)
        }
    }

    fn pop(&mut self) -> i32 {
        let first_in = match self.first_in_one {
            true => self.stack_one.pop(),
            false => self.stack_two.pop(),
        };
        // pop all but one element the non-popped array to the other, toggle first in variable
        let (pop_stack, push_stack) = match self.first_in_one {
            true => (&mut self.stack_two, &mut self.stack_one),
            false => (&mut self.stack_one, &mut self.stack_two),
        };

        while pop_stack.len() > 1 {
            push_stack.push(pop_stack.pop().unwrap());
        }
        self.first_in_one = !self.first_in_one;
        todo!()
    }

    fn peek(&self) -> i32 {
        let target_stack = match self.first_in_one {
            true => &self.stack_one,
            false => &self.stack_two,
        };
        target_stack[0].clone()
    }

    fn empty(&self) -> bool {
        self.stack_two.is_empty() && self.stack_one.is_empty()
    }
}

/*
 * Your MyQueue object will be instantiated and called as such:
 * let obj = MyQueue::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.empty();
 */
