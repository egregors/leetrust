/**
 * https://leetcode.com/problems/min-stack/
 *
 * Design a stack that supports push, pop, top, and retrieving the minimum 
 * element in constant time.
 *
 * push(x) -- Push element x onto stack.
 * pop() -- Removes the element on top of the stack.
 * top() -- Get the top element.
 * getMin() -- Retrieve the minimum element in the stack.
 *
 **/

struct MinStack {
    m: Vec<i32>,
}

impl MinStack {
    fn new() -> Self {
        MinStack { m: vec![] }
    }

    fn push(&mut self, val: i32) {
        self.m.push(val)
    }

    fn pop(&mut self) {
        self.m.pop().unwrap();
        return ();
    }

    fn top(&self) -> i32 {
        self.m.last().unwrap().clone()
    }

    fn get_min(&self) -> i32 {
        *self.m.iter().clone().min().unwrap()
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // Explanation
        // MinStack minStack = new MinStack();
        // minStack.push(-2);
        // minStack.push(0);
        // minStack.push(-3);
        // minStack.getMin(); // return -3
        // minStack.pop();
        // minStack.top();    // return 0
        // minStack.getMin(); // return -2

        let mut s = MinStack::new();
        s.push(-2);
        s.push(0);
        s.push(-3);
        assert_eq!(s.get_min(), -3);
        s.pop();
        assert_eq!(s.top(), 0);
        assert_eq!(s.get_min(), -2);
    }
}
