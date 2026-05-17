package stack

import "testing"

func TestStackOperations(t *testing.T) {
	stack := NewStack()

	if stack.Len() != 0 {
		t.Errorf("Expected length 0, got %d", stack.Len())
	}

	_, err := stack.Pop()
	if err == nil {
		t.Error("Expected error when popping from empty stack, got nil")
	}

	// Test 2: Push operations
	stack.Push(10)
	stack.Push(20)
	stack.Push(30)

	if stack.Len() != 3 {
		t.Errorf("Expected length 3, got %d", stack.Len())
	}

	// Test 3: Peek operation
	top, err := stack.Peek()
	if err != nil {
		t.Fatalf("Unexpected error on Peek: %v", err)
	}
	if top != 30 {
		t.Errorf("Expected top element to be 30, got %v", top)
	}

	// Test 4: Pop operations (LIFO order)
	val1, _ := stack.Pop()
	if val1 != 30 {
		t.Errorf("Expected 30, got %v", val1)
	}

	val2, _ := stack.Pop()
	if val2 != 20 {
		t.Errorf("Expected 20, got %v", val2)
	}

	// Test 5: Length decreases
	if stack.Len() != 1 {
		t.Errorf("Expected length 1, got %d", stack.Len())
	}
}
