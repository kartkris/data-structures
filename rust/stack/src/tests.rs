use crate::stack::Stack;

#[test]
fn test_new_stack_is_empty() {
    let stack: Stack<i32> = Stack::new();
    assert!(stack.is_empty());
    assert_eq!(stack.size(), 0);
}

#[test]
fn test_push_increments_size() {
    let mut stack = Stack::new();
    stack.push(42);

    assert!(!stack.is_empty());
    assert_eq!(stack.size(), 1);
}

#[test]
fn test_pop_returns_elements_in_lifo_order() {
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    // Verifies Last-In, First-Out behavior
    assert_eq!(stack.pop(), Some(3));
    assert_eq!(stack.pop(), Some(2));
    assert_eq!(stack.pop(), Some(1));
    assert_eq!(stack.pop(), None); // Stack is now empty
}

#[test]
fn test_peek_does_not_remove_element() {
    let mut stack = Stack::new();
    stack.push("Rust");

    // Peek should let us look at the top item
    assert_eq!(stack.peek(), Some("Rust"));
    // Size should remain unchanged
    assert_eq!(stack.size(), 1);
    // We can still pop it afterwards
    assert_eq!(stack.pop(), Some("Rust"));
}

#[test]
fn test_peek_and_pop_on_empty_stack() {
    let mut stack: Stack<f64> = Stack::new();

    // Operations on an empty stack must safely return None
    assert_eq!(stack.peek(), None);
    assert_eq!(stack.pop(), None);
}

#[test]
fn test_works_with_strings() {
    let mut stack = Stack::new();
    // Using String to test ownership transfer
    stack.push(String::from("Hello"));
    stack.push(String::from("World"));

    assert_eq!(stack.pop(), Some(String::from("World")));
}
