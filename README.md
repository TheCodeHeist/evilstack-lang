# EvilStack Language

---

## About

EvilStack is a stack-based programming language that has familiar syntax and has similar commands to assembly. It is still in development and is not yet feature-complete. It is made for fun and educational purposes.

## Commands

> The features for heap and memory access are not yet implemented.

### Stack Manipulation

- [x] `push` - Pushes the value to the stack
- [x] `pop` - Pops the value from the stack
- [x] `dup` - Duplicates the top value from the stack
- [x] `swap` - Swaps the top two values from the stack

### Binary Operations

- [x] `add` - Adds the top two values from the stack
- [x] `sub` - Subtracts the top value from the second top value
- [x] `mul` - Multiplies the top two values from the stack
- [x] `div` - Divides the second top value by the top value
- [x] `mod` - Divides the second top value by the top value and pushes the remainder

### Data Manipulation

- [x] `atoi` - Converts the top string value from the stack to an integer
- [x] `ftoi` - Converts the top float value from the stack to an integer

### Flow Control

- [x] `<label>:` - Labels are used to mark a position in the program
- [x] `cmp` - Compares the top two values from the stack
- [x] `jmp @<label>` - Jumps to the label
- [x] `jeq @<label>` - Jumps to the label if the top two values are equal
- [x] `jne @<label>` - Jumps to the label if the top two values are not equal
- [x] `jgt @<label>` - Jumps to the label if the second top value is greater than the top value
- [x] `jlt @<label>` - Jumps to the label if the second top value is less than the top value
- [x] `jge @<label>` - Jumps to the label if the second top value is greater than or equal to the top value
- [x] `jle @<label>` - Jumps to the label if the second top value is less than or equal to the top value
- [x] `ret` - Returns from a subroutine
- [x] `exit` - Exits the program

### I/O

- [x] `print` - Prints the top value from the stack
- [x] `read` - Reads a value from the input and pushes it to the stack

### Built-in Functions

- [x] `rand` - Pushes a random number to the stack
