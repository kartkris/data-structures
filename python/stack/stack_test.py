from stack import Stack


int_stack = Stack[int]()
int_stack.push(10)
int_stack.push(20)
int_stack.push(30)

# 1. Test peek()
print(f"Top item (peek): {int_stack.peek()}")  # Output: 30
print(f"Stack size after peek: {len(int_stack._items)}")  # Still 3

# 2. Test loop iteration (LIFO order)
print("Iterating through stack:")
for item in int_stack:
    print(item)
# Output:
# 30
# 20
# 10
