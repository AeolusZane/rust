# 1. Hello World
```rust
fn main() {
    // Statements here are executed when the compiled binary is called.

    // Print text to the console.
    println!("Hello World!");
}
```

`rustc hello.rs`

# 1.1 Comments
...
# 1.2 Formatted print
`format!`

`print!`

`eprint!`

## Activities
```
Add a println! macro call that prints: Pi is roughly 3.142 by controlling the number of decimal places shown. For the purposes of this exercise, use let pi = 3.141592 as an estimate for pi. (Hint: you may need to check the std::fmt documentation for setting the number of decimals to display)
```
```rust
let pi = 3.141592;
println!("Pi is roughly {pi:.3}"); // 3.142
```

## 1.2.1 Debug
`#[derive(Debug)]` can make variable to be print in debug env.

`{:#?}` pretty printing a structure.

## 1.2.2 Display
manually implementing fmt::Display
...
### Activities

```
use std::fmt; // Import `fmt`

// A structure holding two numbers. `Debug` will be derived so the results can
// be contrasted with `Display`.
#[derive(Debug)]
struct MinMax(i64, i64);

// Implement `Display` for `MinMax`.
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "({}, {})", self.0, self.1)
    }
}

// Define a structure where the fields are nameable for comparison.
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

// Similarly, implement `Display` for `Point2D`.
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

fn main() {
    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range =   MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and the small is {small}",
             small = small_range,
             big = big_range);

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    // Error. Both `Debug` and `Display` were implemented, but `{:b}`
    // requires `fmt::Binary` to be implemented. This will not work.
    // println!("What does Point2D look like in binary: {:b}?", point);
}
```

```
After checking the output of the above example, use the Point2D struct as a guide to add a Complex struct to the example. When printed in the same way, the output should be:

Display: 3.3 + 7.2i
Debug: Complex { real: 3.3, imag: 7.2 }
```
```rust
#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}
fn main(){
    let complex = Complex { real:3.3, imag:7.2};

    println!("Compare points:");
    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);
}
```

tips: `{:b} is unimplemented，if wanna use 'print!({:b},xxx)', requires to implement fmt::Binary`

### 1.2.2.1 List
Vec print

## 1.2.3 Formatting
### Activities
```
Add an implementation of the fmt::Display trait for the Color struct above so that the output displays as:


RGB (128, 255, 90) 0x80FF5A
RGB (0, 3, 254) 0x0003FE
RGB (0, 0, 0) 0x000000
```

```rust
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Display for Color {
    // `f` is a buffer, and this method must write the formatted string into it.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {

        // `write!` is like `format!`, but it will write the formatted string
        // into a buffer (the first argument).
        let g = format!("{:X}",self.green);
        let r = format!("{:X}",self.red);
        let b = format!("{:X}",self.blue);
        write!(f, "RGB ({}, {}, {}) 0x{:0>2}{:0>2}{:0>2}",
               self.red, self.green, self.blue,r,g,b)
    }
}
```