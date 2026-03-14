
struct MinStack {
    // Elements
    elems: Vec<i32>,
    // Positions of elements which, when added, were the minimum
    min_elems: Vec<usize>,
}

impl MinStack {

    fn new() -> Self {
        MinStack { min_elems: Vec::new(), elems: Vec::new() }
    }
    
    fn push(&mut self, val: i32) {
        // Add to the stack
        self.elems.push(val);
        // If smaller than current minimum, push its position to list of minimums
        if val < self.get_min() {
            self.min_elems.push(self.elems.len() - 1);
        }
    }
    
    fn pop(&mut self) {
        self.elems.pop();

        if let Some(min_elem_pos) = self.min_elems.last() {
            if *min_elem_pos == self.elems.len() {
                // Minimum was the element we popped --> update the minimum
                self.min_elems.pop();
            }
        }
    }
    
    fn top(&self) -> i32 {
        self.elems[self.elems.len() - 1]
    }
    
    fn get_min(&self) -> i32 {
        let min_pos = self.min_elems.last().unwrap_or(&usize::MIN);
        self.elems[*min_pos]
    }
}

fn main() {
    let mut ms = MinStack::new();
    ms.push(-2);
    ms.push(0);
    ms.push(-3);
    assert_eq!(ms.get_min(), -3);
    ms.pop();
    assert_eq!(ms.top(), 0);
    assert_eq!(ms.get_min(), -2);
    println!("All tests passed");
}
