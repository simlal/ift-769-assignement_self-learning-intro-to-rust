---
marp: true
theme: default
class: invert
paginate: true
---
<!-- _paginate: false -->

<h1 style="text-align: center;">Introduction to the <span style="color:orange">Rust programming Language</span></h1> 

<div style="text-align: center;">
    <img src="https://www.rust-lang.org/static/images/rust-logo-blk.svg" width="150" height=150 style="margin: 0 30; background-color: white; border-radius: 10px;"> 
    <img src="https://rustacean.net/assets/rustacean-orig-noshadow.svg" width="150" height=150 style="margin: 0 auto; background-color: white; border-radius: 10px;">
</div>
<br>
<p style="text-align: center;">Following along <a href="https://doc.rust-lang.org/book/"> The Rust Book</a> from the official source</p>

<br>
<p style="text-align: center;">by: <strong>Simon Lalonde</strong></p>
<p style="text-align: center;">For: <strong>IFT-769 </strong> (Theoritical concepts CS)</p>


---

<h2>📖 Project overview - <span style="font-weight: normal;">Going through "The Rust Programming Language"</span></h2>

**The Rust Programming Language** by Steve Klabnik and Carol Nichols <img src="https://nostarch.com/sites/default/files/styles/uc_product_full/public/RustProgramming2ndEd_comp.png?itok=sj30HGNE" width="100" style="float: right;">

<u>Book overview</u>:
- Official guide to the Rust programming language
- Covers the basics (syntax, types, functions) + toolchain
- Advanced and Rust-specific features:
    - Ownership, borrowing, lifetimes
    - Unique error handling
    - Concurrency

<font size="4">Klabnik, Steve, and Carol Nichols. The Rust Programming Language. 2nd ed., No Starch Press.</font>

---

<h2>📚 Theoretical concepts <span style="font-weight: normal;"> - Key topics covered</span></h2>

1. Common Programming Concepts (_variables, types, control flow_)
2. Understanding <span style="color:orange">Ownership</span> (_memory management_)
3. Structs, Enums and Pattern Matching
4. Containers/Collections
5. <span style="color:orange">Error Handling</span>
6. <span style="color:orange">Generics, Traits and Lifetimes</span>
7. Functional and OO features
8. <span style="color:orange">Smart pointers and Concurrency</span>
9. Patterns and matching + Advanced features

<font size="4">Klabnik, Steve, and Carol Nichols. The Rust Programming Language. 2nd ed., No Starch Press.</font>

---

<h2><img src="https://rustacean.net/assets/rustacean-orig-noshadow.svg" width=60px> <span style="color:orange;">Rust</span> Overview</h2>

- **Systems programming language** focused on **safety** and **performance**
- TODO

<strong><u>Currently known projects</u></strong>
TODO
<strong><u>Predicted use cases</u></strong>
TODO

---

<h2><img src="https://em-content.zobj.net/source/google/387/memo_1f4dd.png" width=60px> Pros and Cons of <span style="color:orange;">Rust</span></h2>

<span style="color:green;">***PROS***:</span>
- **Memory safety**: No null pointers, dangling pointers, or buffer overflows
- **Error handling**: With the `Result` and `Option` types
- **Concurrency**: Safe and efficient with the ownership system
- **Performance**: Comparable to C/C++ with zero-cost abstractions
- **Ecosystem**: Growing with a strong community and package manager (**Cargo**)
- **Helpful compiler**: Provides detailed error messages and warnings

<span style="color:red;">***CONS***:</span>
- **Learning curve**: Ownership, borrowing, and lifetimes can be challenging
- **Tooling and prevalence**: Not as mature as other languages (C/C++, Python, etc.)
- **Syntax**: Can be verbose and complex compared to other languages

---

<h2><img src="https://em-content.zobj.net/source/google/387/gear_2699-fe0f.png" width=60px> Installation and setup</h2>

<u>**Installation**</u>:
1. Install <span style="color:orange;">Rust</span> using `rustup` (Rust toolchain installer)

<u>**Included toolchain**</u>:
- `rustc`: Rust compiler
- `rustup`: Rust toolchain manager
- `rustfmt`: Rust code formatter
- `cargo`: Rust package manager and build tool

**Package and library management**
- **Crates** are Rust packages that can be shared and reused
- Managed with **Cargo**, the Rust package manager


---

<h2><img src="https://em-content.zobj.net/source/google/387/national-park_1f3de-fe0f.png" width=60px> Development environment<span style="font-weight: normal;"> - Toolchain overview </span></h2>

**Env setup and features:**
- Easy install: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Rustup for managing toolchains: `rustup update`
- Included formatter: `rustfmt --check src/main.rs` (dry-run mode)
- Cargo for building and managing projects: `cargo new project_name`
- Quality of life with `rust-analyzer`: LSP, build/debug IDE support etc.

---
<h2><img src="https://em-content.zobj.net/source/google/387/national-park_1f3de-fe0f.png" width=60px> Development environment<span style="font-weight: normal;"> - Cargo features </span></h2>

**Useful Cargo commands when building a project:**
- `cargo build` or `cargo run` to compile and run the project. Use `--release` flag for compilation with optimizations inside `target/release/`
- `cargo check`: Check the project for errors without building
- `cargo doc`: Generate documentation for the project
- `cargo clean`: Remove build artifacts
- `cargo update`: Update dependencies
- `cargo fmt`: Format the code according to the Rust style guidelines
- `cargo test`: Run tests in the project

---

<h2><img src="https://em-content.zobj.net/source/google/387/hammer-and-wrench_1f6e0-fe0f.png" width=60px> Practical project #0 - <span style="font-weight: normal;">Guessing game</span></h2>

Great way to introduce to the development environment and basic concepts of Rust:
- Common programming concepts (types, funcs, control flow)
- Use of another crate (rand) inside the project
- I/O, String manipulation, error handling
- Compiler warnings and error messages
- `rust-analyzer` compiler FE for IDE support


<br>

<font size="4">Klabnik, Steve, and Carol Nichols. The Rust Programming Language. 2nd ed., No Starch Press.</font>


---

<p style="text-align: center; font-size: 2em;">🚀 Demo Time!
<br>
<p style="text-align: center; font-size: 1.5em;">Simple guessing game CLI app 🎲 (Basics and dev environment features)</p>

---

<h2> <img src="https://em-content.zobj.net/source/google/387/magnifying-glass-tilted-right_1f50e.png" width=60px> Demo reminders <span style="font-weight: normal;"> - P#0 (Guessing game)</span></h2>

- `Result` type with `.expect()` for error handling
- `cargo doc --open` to generate and view documentation
- `cargo fmt` to format the code
- Type annotations and `let` for variable declaration

---

<h2><img src="https://em-content.zobj.net/source/google/387/memo_1f4dd.png" width=60px> Variables and mutability <span style="font-weight: normal;"></span></h2>

Variables are immutable by default
```rust
let x = 5;          // immutable variable
x = 10;             //!ERROR!

let mut y = 10;     // mutable variable
y = 15;             // Will compile (overwrite with same type)
let y = "hello";    // Will compile (shadowing)
```
<br>

Constants are always immutable within the scope
```rust
const MAX_POINTS: u32 = 100_000;
```

---

<h2><img src="https://em-content.zobj.net/source/google/387/abacus_1f9ee.png" width=60px> Statically typed + type inference <span style="font-weight: normal;"></span></h2>

`rust-analyzer` provides type hints and suggestions
```rust
let secret_num = rand::thread_rng().gen_range(1..101); // Will infer i32 type
```

Explicit type annotations can or must be used
```rust
let mut num: String = String::new(); // Can be annotated or inferred
num = "42".to_string();              

let guess = guess.trim().parse().expect("Please enter a number");  // Wont Compile
let guess: u32 = guess.trim().parse().expect("Please enter a number"); // Will compile
```

---

<h2><img src="https://em-content.zobj.net/source/google/387/abacus_1f9ee.png" width=60px> Data types <span style="font-weight: normal;"> - Scalars</span></h2>


| Data type | Size | Specifity |
|-----------|-------------|-----------|
| int       | 8-128 bits       | signed/unsigned |
| float     | 32/64 bits      | simple/double precision |
| char      | 4 bytes          | unicode |
| bool      | 1 byte           | true/false |


---

<h2><img src="https://em-content.zobj.net/source/google/387/abacus_1f9ee.png" width=60px> Data types <span style="font-weight: normal;"> - Compound</span></h2>


| Data type | Size | Elements | Example | Access |
|-----------|-------------|-----------|--|---|
| tuple     | fixed         | mixed types | `(1, "hello", 3.14)` | `tuple.0` |
| array     | fixed         | same type | `[1, 2, 3, 4, 5]` | `array[0]` |
| vec       | dynamic       | same type | `vec![1, 2, 3, 4, 5]` | `vec[0]` |

<span style="color:orange">Access safety with runtime bounds checking.</span> If using `array[10]` will panic at runtime instead of *undefined behavior like in C/C++*

---

<h2><img src="https://em-content.zobj.net/source/google/387/gear_2699-fe0f.png" width=60px> Functions <span style="font-weight: normal;"> - <code>main</code></span></h2>

Functions are defined with the `fn` keyword. All programs start with a `main` function
```rust
fn main() {
    println!("Hello, world!");
    
    say_hello_back();
}

fn say_hello_back() {
    println!("Hello back!");
}
```

---

<h2><img src="https://em-content.zobj.net/source/google/387/gear_2699-fe0f.png" width=60px> Functions <span style="font-weight: normal;"> - Parameters and return</span></h2>

**Function signatures and use**:

- Parameters must have type annotations
- Return type must be specified with `->`
- Functions can return multiple values with tuples

```rust
fn main() {
    let num_sum = add(5, 10);
    println!("The sum is: {}", num_sum);
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}
```

---

<h2><img src="https://em-content.zobj.net/source/google/387/scroll_1f4dc.png" width=60px> Statements <span style="font-weight: normal;"></span></h2>

- `let` is a statement, and `x + y` is an expression.

- Compared to C/C++, var assignment is an expression in Rust and **does not** return a value
    
- Statements must end with a semicolon `;`

```rust
let x = 5;          // statement
let y = z = 10;     // ERROR! z = 10 does not return a value
```

---

<h2><img src="https://em-content.zobj.net/source/google/387/light-bulb_1f4a1.png" width=60px> Expressions <span style="font-weight: normal;"></span></h2>

- Expressions **evaluate** to a value (_func calls, operations, blocks_)
- No `;` at the end of expressions
- **Blocks** `{}` are expressions and can be used to create new scopes + return values

```rust
fn main() {
    let x = 5; // whole line is statement, 5 is expression
    let y = {
        let x = 3;
        x + 1
    }; // an expression
    println!("The value of y is: {}", y); // Prints 4!
}
```
---

<h2><img src="https://em-content.zobj.net/source/google/387/toolbox_1f9f0.png" width=60px> Control Flow<span style="font-weight: normal;"> - Conditionals</span></h2>

**if/else**: (Only takes boolean expressions)
```rust
// Classic if/else if/else
let mut condition = false;
if number < 5 {
    println!("Too small!");
} else if number > 5 {
    println!("Too big!");
} else {
    println!("Just right!");
    condition = true;
}

// Assignement with if/else
let result = if condition { 5 } else { 6 };
```

---

<h2><img src="https://em-content.zobj.net/source/google/387/toolbox_1f9f0.png" width=60px> Control Flow<span style="font-weight: normal;"> - Loops overview</span></h2>

3 types of loops in <span style="color:orange;">Rust</span>: **`loop`**, **`while`** and **`for`**

- **`loop`**: Infinite loop until `break` or return
- **`while`**: Loop while condition is true
- **`for`**: Loop over an iterator

<br>

```rust
// Conditional loop
let mut counter = 0;
while counter < 10 {
    println!("counter = {counter}");
    counter += 1;
}
```

---

<h2><img src="https://em-content.zobj.net/source/google/387/toolbox_1f9f0.png" width=60px> Control Flow<span style="font-weight: normal;"> - Loop labels</span></h2>

**Loop labels** can be used to distinguish nested loops (_break_ and _continue_)
```rust
fn main() {
    let mut count = 0;
    'counting_up: loop { // Label the outer loop
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;   // Break the outer loop
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}
```
---

<h2><img src="https://em-content.zobj.net/source/google/387/toolbox_1f9f0.png" width=60px> Control Flow<span style="font-weight: normal;"> - Collection with <code>for</code></span></h2>

No need for manual indexing, **`for`** loops iterate over collections
```rust
let collection = [10, 20, 30, 40, 50];
for element in collection {
    println!("The value is: {element}");
};
```

---

<h2><img src="https://em-content.zobj.net/source/google/387/key_1f511.png" width=60px> Ownership <span style="font-weight: normal;"> - Overview</span></h2>

**Ownership** is a key feature of <span style="color:orange;">Rust</span> regarding the management of stack (_static, compile-time known, LIFO_) and heap memory (_allocated at runtime, dynamic, FIFO_). 

It ensures memory safety without garbage collection.

<br>

<span style="color:orange;">**The 3 rules of ownership**</span>:
1. Each value in Rust has a variable that's its owner
2. There can only be one owner at a time
3. When the owner goes out of scope, the value will be dropped

---

<h2><img src="https://em-content.zobj.net/source/google/387/key_1f511.png" width=60px> Ownership <span style="font-weight: normal;"> - String Type vs. literals</span></h2>

```rust
let s1: &str = "hello"; // string literal, immutable

{
    // s1 is still valid
    let mut s2 = String::from("hello"); // allocated on the heap
    s2.push_str(", world!"); // Mutable
    
} // calls drop(), s2 goes out of scope its memory is freed
```

- String literals hardcoded into binary. Immutable and fast.
- **`String`** type is allocated on the heap and is mutable. Memory freed when out of scope. Similar to smart pointers in C++.


---

<h2><img src="https://em-content.zobj.net/source/google/387/key_1f511.png" width=60px> Ownership <span style="font-weight: normal;"> - Move</span></h2>

```rust
// MOVE
let s1 = String::from("hello");
let s2 = s1; // s1 is moved to s2
println!("{s1}"); // ERROR! s1 is no longer valid

// DEEP COPY
let s3 = s2.clone(); // deep copy
println!("{s2}"); // s2 is still valid
```


<div style="display: flex; align-items: center; justify-content: space-between; width: 85%;">
    <p>No <em>double free</em> or <em>dangling pointers</em> with the <strong>move</strong> operation (first 3 lines of code).</p>
    <img src="./img/fig4-4_move.svg" height="300">
</div>

---

<h2><img src="https://em-content.zobj.net/source/google/387/key_1f511.png" width=60px> Ownership <span style="font-weight: normal;"> - Copy</span></h2>

Types that implement the `Copy` trait are copied instead of moved. Stack-only data types (i.e. integers, booleans, char etc.) for speed and efficiency. 

```rust
let x = 5;
let y = x; // x is copied to y

println!("{x}"); // x is still valid. Same as x.clone() but no needed
```

---

<h2><img src="https://em-content.zobj.net/source/google/387/key_1f511.png" width=60px> Taking ownership <span style="font-weight: normal;"> - Functions </span></h2>

```rust
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...

    let x = 5;                      // x comes into scope
                                    // but i32 is Copy, so x available afterward
    
    println!("{s} world!");         // ERROR! s is no longer valid
} 

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // `some_string` goes out of scope, `drop` is called and memory is freed

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // `some_integer` goes out of scope, nothing happens.
```

---

<h2><img src="https://em-content.zobj.net/source/google/387/key_1f511.png" width=60px> Transfer Ownership <span style="font-weight: normal;"> - Function return and scope</span></h2>

A bit tedious, but ownership can be transferred back to the calling function with the return value. 

```rust
fn main() {
    let s1 = gives_ownership();         // `gives_ownership` moves its return val into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into `takes_and_gives_back` becomes invalid
                                        // `takes_and_gives_back` returns a new String that into s3
} 

fn gives_ownership() -> String {             // `gives_ownership` move return val into the 
                                             // function that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned moves out of calling func
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope

    a_string  // a_string is returned and moves out to the calling function
}
```

---

<h2><img src="https://em-content.zobj.net/source/google/387/handshake_1f91d.png" width=60px> References and Borrowing <span style="font-weight: normal;"> - Overview</span></h2>

Kind of like passing by reference in C/C++ but with some key differences: 
- **References** are immutable by default
- **Borrowing** allows multiple references to the same data
- **Mutable references** are exclusive and have strict rules

<br>

References are created with the `&` symbol, and borrowing is done with `&mut` for mutable references (see next slide).

---

<h2><img src="https://em-content.zobj.net/source/google/387/handshake_1f91d.png" width=60px> References and Borrowing<span style="font-weight: normal;"> - Simple borrowing example</span></h2>

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1); 

    println!("The length of '{s1}' is {len}."); // s1 still valid because it is borrowed
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // s goes out of scope, but does not have ownership of what it refers to
```
<img src="./img/fig4-5_borrowing-simple.svg" height="250">


---

<h2><img src="https://em-content.zobj.net/source/google/387/gear_2699-fe0f.png" width=60px> TODO <span style="font-weight: normal;"></span></h2>

---

<h2><img src="https://em-content.zobj.net/source/google/387/hammer-and-wrench_1f6e0-fe0f.png" width=60px> Practical project #1 - <span style="font-weight: normal;">Write an I/O CLI program</span></h2>

**Halfway project for a `grep` clone CLI app covers:**
1. Code organization (crates, modules)
2. Use of containers and strings
3. Error handling
4. Using traits and lifetimes
5. Testing and documentation

<font size="4">Klabnik, Steve, and Carol Nichols. The Rust Programming Language. 2nd ed., No Starch Press.</font>

---

<h2><img src="https://em-content.zobj.net/source/google/387/hammer-and-wrench_1f6e0-fe0f.png" width=60px> Practical project #2 - <span style="font-weight: normal;">Building a Multithreaded Web Server</span></h2>

**Final Project from the book includes :**
1. Learn TCP/IP networking and HTTP
2. Listen to TCP connections on a socket
3. Parse HTTP requests
4. Generate HTTP responses
5. Handle multiple requests concurrently with a thread pool

<font size="4">Klabnik, Steve, and Carol Nichols. The Rust Programming Language. 2nd ed., No Starch Press.</font>

---

<h2><img src="https://em-content.zobj.net/source/google/387/gear_2699-fe0f.png" width=60px> THEORY STUFF <span style="font-weight: normal;"></span></h2>

TODO


