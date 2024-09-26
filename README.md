# Philo Programming Language

Philo is a low-level programming language designed for C/Rust programmers, offering direct access to registers and efficient performance suitable for systems programming, such as operating systems.

## Features

### 1. Basic Syntax

Functions are defined using the `fn` keyword:

```rust
fn foo() returns void {
    // Function body
}
```

### 2. Variables

Variables can be declared as follows:

```Rust
let x: i32 = 5; // Standard variable
let x: %RAX = 5; // Accessing variables with codenames
```

### 3. Mutable Variables

You can modify variables as needed, promoting flexibility in code.

### 4. Inline Assembly

Philo allows inline assembly for low-level operations:

```Rust
@mov %rax = 5; // Move 5 into the register %rax
```

### 5. Control Flow
#### 5.1 For Loop
The ```for``` loop syntax provides both standard and custom increment functionality:

```Rust
for %rcx in 0..%rbx {
    // Loop body
}

for %rcx in 0..%rbx inc 3 {
    // Loop body with increment of 3
}
```

#### 5.2 While Loop
The while loop enables dynamic condition checking:

```Rust
while %rcx < %rbx {
    // Loop body
}
```

#### 5.3 If Statement

Control flow is further enhanced with the if statement:

```Rust
if %rcx < %rbx {
    // Code to execute if condition is true
} else {
    // Code to execute if condition is false
}
```

### 6. Structs
Philo supports user-defined data types using structs:

```Rust
struct s {
    pub a: u32,
    pub b: i32,
    pub c: f64,
}

fn main() returns void {
    let instance: s = s { a: 10, b: -5, c: 3.14 };
}
```

#### 7. Methods with impl

You can define methods associated with structs using the impl keyword:

```Rust
impl s {
    fn print_values(&self) {
        // Access struct fields using self
    }

    fn create_instance(a: u32, b: i32, c: f64) returns s {
        return s { a, b, c };
    }
}
```

### 8. Enums
Enums allow you to define named constants easily:
```Rust
enum Color {
    Red,
    Green,
    Blue,
}

fn main() returns void {
    let my_color: Color = Color::Green;

    match my_color {
        Color::Red => { /* Handle Red */ },
        Color::Green => { /* Handle Green */ },
        Color::Blue => { /* Handle Blue */ },
    }
}
```

## Conclusion

Philo combines the familiarity of high-level syntax with low-level programming capabilities, making it a powerful tool for developers needing direct access to system resources. The combination of features like structs, enums, and various control flow statements provides flexibility and efficiency for system-level programming.
