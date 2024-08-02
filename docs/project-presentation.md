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
7. OO features (⚠️ _little covered in this presentation_)
8. <span style="color:orange">Smart pointers and Concurrency</span> (🛑 _Not covered in this presentation_)

<font size="4">Klabnik, Steve, and Carol Nichols. The Rust Programming Language. 2nd ed., No Starch Press.</font>

---

<h2><img src="https://rustacean.net/assets/rustacean-orig-noshadow.svg" width=60px> <span style="color:orange;">Rust</span> Overview</h2>

Written by **Graydon Hoare in 2006**, Rust is a systems programming language focused on safety, speed, and concurrency. Backed by **Mozilla** and now the Rust Foundation.

<strong><u>Currently known projects</u></strong>
- Now inside some of the Linux kernel
- Used in components of Firefox browser

<strong><u>Predicted use cases</u></strong>
- Replacement for C/C++ in systems programming and embedded systems
- WebAssembly, ML/AI, and other performance-critical applications 

<font size="4"> Rust Programming Language. 2024. [Official Website](https://www.rust-lang.org/)</font>
<font size="4"> Lin Clark. The whole web at maximum FPS: How WebRender gets rid of jank. 2017. [Mozilla Blog](https://hacks.mozilla.org/2017/10/the-whole-web-at-maximum-fps-how-webrender-gets-rid-of-jank/)</font>
<font size="4">Clive Thompson. How Rust went from a side project to the world’s most-loved programming language. 2023. [TechReview](https://www.technologyreview.com/2023/02/14/1067869/rust-worlds-fastest-growing-programming-language/)</font>

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
<br>

🔎 **Ranges**, use the `..` operator
```rust
for number in 1..4 {
    println!("The value is: {number}");
}
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

<h2><img src="https://em-content.zobj.net/source/google/387/handshake_1f91d.png" width=60px> References and Borrowing <span style="font-weight: normal;"> - Simple borrowing example</span></h2>

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

<h2><img src="https://em-content.zobj.net/source/google/387/handshake_1f91d.png" width=60px> Mutable references <span style="font-weight: normal;"> - General case</span></h2>

**Borrowed references are not mutable by default**. To allow mutation, use `&mut`

```rust
// let s = String::from("hello"); // WOULD NOT COMPILE!
let mut s = String::from("hello");
change(&mut s);

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

---

<h2><img src="https://em-content.zobj.net/source/google/387/handshake_1f91d.png" width=60px> Mutable references <span style="font-weight: normal;"> - Data races safety</span></h2>

<u>Compile time checks for mutable refs</u>
⚠️ **NO** multiple mutable references to the same data

```rust
    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s;   // ERROR! r1 is still active
    println!("{}, {}", r1, r2);
```

<br>

⚠️ **NO** mutable references while immutable references are active

```rust
    let mut s = String::from("hello");
    let r1 = &s;        // OK
    let r2 = &s;        // OK
    let r3 = &mut s;    // ERROR! r1 and r2 are still active
    println!("{}, {}, {}", r1, r2, r3);
```

---

<h2><img src="https://em-content.zobj.net/source/google/387/handshake_1f91d.png" width=60px> Mutable references <span style="font-weight: normal;"> - Data races safety (2/2)</span></h2>

🔎 Use of scopes to limit mutable references

```rust
let mut s = String::from("hello");
{
    let r1 = &mut s;
} // r1 goes out of scope, allowing a new mutable reference
let r2 = &mut s; // OK!
```

🔎 Reference's scope ends after the last usage of the reference.

```rust
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem because r1/r2 are no longer valid
    println!("{r3}");
```

---

<h2><img src="https://em-content.zobj.net/source/google/387/warning_26a0-fe0f.png" width=60px> Reference caution <span style="font-weight: normal;"> - Fixing a state management problem </span></h2>

❗Tedious or even problematic when working on a reference

```rust
let mut s = String::from("hello world");
let word_index = first_word(&s); // word_index will get the value 5

s.clear(); // empties the String, making it equal to ""
// `word_index` still has the value 5 here, but no more string tied because s is invalid
println!("the first word is: {s[..word_index]}"); // ERROR! s is empty
```
```rust
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
// Imagine implementing second_word() and managing state...
```

---

<h2><img src="https://em-content.zobj.net/source/google/387/kitchen-knife_1f52a.png" width=60px> String Slice Type <span style="font-weight: normal;"> - A kind of reference </span></h2>

<div style="display: flex; align-items: center; justify-content: space-between;">
    
<p><span style="font-weight: bold;">Slices</span> are references to a contiguous sequence of elements in a collection. They are a reference to a part of a string or array.</p>
    
<img src="./img/fig4-6_string-slice.svg" height="350" style="margin: 0 30;">

</div>

```rust
    let s = String::from("hello world");
    let hello = &s[0..5];   // same as &s[..5]. Excludes the last index 
    let world = &s[6..11];  // same as &s[6..]. Includes the first index
```

---

<h2><img src="https://em-content.zobj.net/source/google/387/kitchen-knife_1f52a.png" width=60px> String Slice <span style="font-weight: normal;"> - Refactoring <code>first_word()</code></span></h2>

```rust
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]; // return a slice(ref) of the string
        }
    }
    &s[..]  // or return the slice of whole string
}
```

```rust
// Compiler assures that the slice is valid as long as the string is valid
fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s); // immutable borrow (return type is &str)
    
    s.clear(); // error! mutable borrow while immutable borrow is active
    println!("the first word is: {word}");
}
```
---

<h2><img src="https://em-content.zobj.net/source/google/387/kitchen-knife_1f52a.png" width=60px> Other Slice types <span style="font-weight: normal;"> - Array example <code>first_word()</code></span></h2>

Similar to strings, slices can be used with arrays

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3]; // slice is of type &[i32]
    assert_eq!([2, 3], slice);
}
```

Useful for passing parts of arrays to functions without copying the data.

---

<h2><img src="https://em-content.zobj.net/source/google/387/building-construction_1f3d7-fe0f.png" width=60px> Structs <span style="font-weight: normal;"> - Overview</span></h2>

**Structs** data structure encapsulate fields of specific types and methods (just like in C++/OO language). 
- If declared mutable, the whole struct is mutable.
- dot notation for named field access
- Methods are defined within the `impl` block

```rust
let mut user1 = User {
    username: String::from("user1"),
    phone: 1234567890
    active: true,
};

user1.active = false;
```

---

<h2><img src="https://em-content.zobj.net/source/google/387/building-construction_1f3d7-fe0f.png" width=60px> Structs <span style="font-weight: normal;"> - Shorthands</span></h2>

```rust
fn build_user(username: String, phone: u32) -> User {  // Returns a User struct
    User {
        username, // shorthand for username: username
        phone,    // shorthand for phone: phone
        active: true
    }
}

// Struct update syntax
let user2 = User {
    phone: 9876543210
    ..user1 // copy the rest of the fields from user1
}
```

---

<h2><img src="https://em-content.zobj.net/source/google/387/building-construction_1f3d7-fe0f.png" width=60px> Tuple Structs <span style="font-weight: normal;"></span></h2>

**Tuple structs** are similar, but don't have named fields. Useful for naming tuples and creating new types.

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    // black and origin are different types
    let red = Color(255, 0, 0);
    let origin = Point(0, 0, 0);
}
// Cannot pass a Point even though they have the same fields' types
fn make_paler(color: Color) -> Color {  
    Color(color.0 / 2, color.1 / 2, color.2 / 2)
}
```

---

<h2><img src="https://em-content.zobj.net/source/google/387/gear_2699-fe0f.png" width=60px> Methods <span style="font-weight: normal;"> - Basic implementation</span></h2>

Methods are defined within the `impl` block.

```rust
struct SumArgs {
  n1: i32,
  n2: i32,
}
impl SumArgs {
  fn add_numbers(&self) -> i32 { // self is alias for Self (instead of args: &SumArgs)
    self.n1 + self.n2
  }
}
fn main() {
  let args = SumArgs { n1: 2, n2: 3 };
  let sum = args.add_numbers(); // Or SumArgs::add_numbers(&args)
  println!("{} + {} = {}", args.n1, args.n2, sum);
}
```
<font size="4"> Gian Lorenzetto. Rust - Structs, Functions and Methods. 2021. [Medium Post](https://gian-lorenzetto.medium.com/rust-structs-functions-and-methods-d60fd597d956)</font>

---

<h2><img src="https://em-content.zobj.net/source/google/387/gear_2699-fe0f.png" width=60px> Methods <span style="font-weight: normal;"> - Mutability</span></h2>

Use `&self` for read-only and `&mut self` for methods that modify the struct.

```rust
struct Rectangle {
    width: u32,
    height: u32
}
impl Rectangle {
    fn area(&self) -> u32 {  // takes ownership of self (read-only)
        self.width * self.height
    }
    fn half_rect(&mut self) {  // borrows mutably
        self.width /= 2;
        self.height /= 2;
    }
    fn width(&self) -> bool {  // Getters in Rust
        self.width > 0
    }
}
let mut rect = Rectangle { width: 10, height: 20 }; 
println!("rect's width is valid: {} because width={}", rect.width(), rect.width);
```

---

<h2><img src="https://em-content.zobj.net/source/google/387/gear_2699-fe0f.png" width=60px> Methods <span style="font-weight: normal;"> - Automatic referencing/dereferencing</span></h2>

Unlike in C/C++, <span style="color: orange;">Rust automatically references and dereferences </span>when calling methods (No `->` operator or `(*object).something()`)

```rust
p1.distance(&p2); // Both are the same, version1 is more readable
(&p1).distance(&p2);
```
With `object.something()`, Rust automatically adds in `&`, `&mut`, or `*` to match signature of the method.

 🔎 It depends wether method is **reading (`&self`), writing (`&mut self`), or consuming (`self`)**

---

<h2><img src="https://em-content.zobj.net/source/google/387/gear_2699-fe0f.png" width=60px> Methods <span style="font-weight: normal;"> - Associated function</span></h2>

When a function is associated with a struct, it doesn't take `self` as a parameter.
- Often used for constructor
- Called with the `::` syntax

```rust
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
let square = Rectangle::square(10);
```
---

<h2><img src="https://em-content.zobj.net/source/google/387/clipboard_1f4cb.png" width=60px> Enums <span style="font-weight: normal;"> - Overview</span></h2>

- `Enums` are a way to define a type by enumerating its possible variants
- Each variant can have different data associated with it (_i.e. `struct`, `String` ..._)
- Namespaced under identifier, accessed with `Enum::variant` syntax 
- Default constructor is `Enum::variant(data)`
```rust
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    // Construct instances of each variant
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
```

---

<h2><img src="https://em-content.zobj.net/source/google/387/clipboard_1f4cb.png" width=60px> Enums <span style="font-weight: normal;"> - Advantages over <code>struct</code></span></h2>

Use of `impl` blocks for **common methods** that applies to **all variants**

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
} // Could be same as 4 different structs `struct Quit`, `struct Move{...}`

impl Message {
    fn send(&self) { 
        // self ref to the variant instance
        println!("Sending message {:?}...", self);  
    }
}
let m = Message::Write(String::from("Hello, world!"));
m.send();
```

---

<h2><img src="https://em-content.zobj.net/source/google/387/question-mark_2753.png" width=60px> Option Enum <span style="font-weight: normal;"></span> - <span style="color: red"> NULL free!</span> </h2>

Rust has no `null` value, but uses the `Option` enum to represent the presence or absence of a value from standard library.

```rust
enum Option<T> { // Generic type T
    Some(T),     // Some value of type T
    None,
}

let x: i8 = 5; 
let y: Option<i8> = Some(5); // Some value
let z: Option<i8> = None;  // No value

let sum = x + y;  // Won't compile because i8 + Option<i8> are different types 
                  // and sum not implemented
```
With `Option`, the compiler forces you to handle the case where the value is `None`.

---

<h2><img src="https://em-content.zobj.net/source/google/387/crossed-swords_2694-fe0f.png" width=60px> Match Expression<span style="font-weight: normal;"> - Overview</span></h2>

`match` is a control flow operator that compares a value against a series of patterns and then executes code based on which pattern matches.

```rust
#[derive(Debug)] // to inspect the state inside match expr
enum UsState {Alabama, Alaska, //...}
enum Coin {Penny, Nickel, Dime, Quarter(UsState)}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        } // Passing a Coin::Quarter(UsState::Alaska) will print "State
    }     // quarter from Alaska!" and return 25
}     
```

---

<h2><img src="https://em-content.zobj.net/source/google/387/crossed-swords_2694-fe0f.png" width=60px> Match Expression<span style="font-weight: normal;"> - Matching with <code>Option&ltT&gt</code></span></h2>

More powerful than `switch` in C/C++ because it can match on any type.

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);             
let six = plus_one(five);       // returns Some(6)
let none = plus_one(None);      // returns None
```

---

<h2><img src="https://em-content.zobj.net/source/google/387/crossed-swords_2694-fe0f.png" width=60px> Match Expression<span style="font-weight: normal;"> - Exhaustive matching and catch-all</span></h2>

Evaluation is in order. We can use `other` or `_` to catch all other cases.

```rust
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    other => move_player(other),    // if no param needed, 
}                                   // use _ => paramless_func()

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
```
<span style="color: orange;">Powerful (type checking, Option, enums) and concise (no `if-else` chains).</span>

---

<h2><img src="https://em-content.zobj.net/source/google/387/counterclockwise-arrows-button_1f504.png" width=60px> Concise Control Flow <span style="font-weight: normal;"></span></h2>

Syntax sugar for single match arms with `if` guards or single catch-all arm.

```rust
let mut count = 0;
// match version
match coin {
    Coin::Quarter(state) => println!("State quarter from {state:?}!"),
    _ => count += 1,
}
// if let version
if let Coin::Quarter(state) = coin {
    println!("State quarter from {state:?}!");
} else {
    count += 1;
}
```

---

<h2><img src="https://em-content.zobj.net/source/google/387/open-file-folder_1f4c2.png" width=60px> Common Collections <span style="font-weight: normal;"> - Overview</span></h2>

- Collections are data structures that can store multiple values
- Heap allocated
- Unknown size at compile time, but can grow or shrink at runtime

**Discussed here:**
1. **Vectors** - Dynamic array
2. **Strings** - UTF-8 encoded
3. **Hash Maps** - Key/Value pairs

---

<h2><img src="https://em-content.zobj.net/source/google/387/open-file-folder_1f4c2.png" width=60px> Collections <span style="font-weight: normal;"> - Vectors init and access</span></h2>

**Vectors** are dynamic arrays. Generic type of `Vec<T>`
```rust
// Initialization
let v: Vec<u8> = Vec::new();   // type required
let mut my_vec = vec![1, 2, 3];    // type inferred with `vec!` macro
my_vec.push(4);                    // Add an element
```

Accessing elements and bounds checking. Both yield a reference.
```rust
let third: &i32 = &my_vec[2];    // Panics! if out of bounds
println!("The third element is {third}");

let third: Option<&i32> = my_vec.get(2);    // Returns None if out of bounds
match third {
    Some(third) => println!("The third element is {third}"),
    None => println!("There is no third element."),
}
```

---

<h2><img src="https://em-content.zobj.net/source/google/387/open-file-folder_1f4c2.png" width=60px> Collections <span style="font-weight: normal;"> - Vectors' iteration and types</span></h2>

Iterate in read-only or mutable mode with `for` loop
```rust
for i in &my_vec {    // Readonly
    println!("{i}");
}
```

Store only similar types within same vec, but can use `enum` for different types
```rust
{
    enum CliArg {
        Int(i32),
        Text(String),
    }
    let mut arguments = vec![
        CliArg::Int(5),
        CliArg::Text(String::from("my_database_name")),
    ];
    arguments.push(CliArg::Text(String::from("my_table_name")));   // mutable with mixed types
} // <---- Out of scope, `arguments`'s memory is freed
```

---

<h2><img src="https://em-content.zobj.net/source/google/387/open-file-folder_1f4c2.png" width=60px> Collections <span style="font-weight: normal;"> - Strings Overview</span></h2>

String `str` std vs `String` type:
- `str` is immutable, usually used as a slice/reference that can be borrowed
- `String` is mutable, heap-allocated, growable, and owned
- `String` is a wrapper around a `Vec<u8>`

```rust
let mut s = String::new();      // Empty string
let data = "initial contents";  // str slice

// `to_string` is available on any type that implements the `Display` trait
let s = data.to_string();                   // Convert &str to String
let s = String::from("initial contents");   // Same as above
s.push_str(" and more");                        // Append a string slice, does not take ownership
println!("{s}");                            // Prints "initial contents and more"
```

---

<h2><img src="https://em-content.zobj.net/source/google/387/open-file-folder_1f4c2.png" width=60px> Collections <span style="font-weight: normal;"> - String basic ops: Concat</span></h2>

**Concat with `+` or `format!` macro**

`+` Operator is actually a call to add which takes a reference to a `String` and returns a new `String`. So one of the strings will be moved and the other will be borrowed.
```rust
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // s1 has been moved here and can no longer be used

let s4 = String::from("tic");
let s5 = String::from("toc");
let s6 = format!("{s4}-{s5}"); // format! does not take ownership
```

---

<h2><img src="https://em-content.zobj.net/source/google/387/open-file-folder_1f4c2.png" width=60px> Collections <span style="font-weight: normal;"> - String basic ops: Indexing</span></h2>

- Cannot index directly with `[]` because of UTF-8 encoding**
- Use `&str` slices to index per byte (careful 💀!)
- Use `chars()` method to iterate over Unicode characters

```rust
let hello = "Здравствуйте";
let s = &hello[0..4];   // Corresponds to first 4 bytes so `Зд` in UTF-8

// To iterate per character regardless of size
for c in hello.chars() {
    println!("{c}");
    // prints `З`, `д`, `р`, `а`, `в`, `с`, `т`, `в`, `у`, `й`, `т`, `е`
}
```

---

<h2><img src="https://em-content.zobj.net/source/google/387/open-file-folder_1f4c2.png" width=60px> Collections <span style="font-weight: normal;"> - Hash Maps</span></h2>

- `HashMap<K, V>` is a collection of key-value pairs/dictionary/hash table with unique keys
- Homogenous typing for keys and values

```rust
use std::collections::HashMap;
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

let team_name = String::from("Blue");  // later borrowed by get as `&str`
let score = scores.get(&team_name).copied().unwrap_or(0);
// get() returns an Option<&V> so we use copied() to get the value
// unwrap_or() returns the value or a default if None

// Iterate over key-value pairs
for (key, value) in &scores {
    println!("{key}: {value}");
}
```

---

<h2><img src="https://em-content.zobj.net/source/google/387/open-file-folder_1f4c2.png" width=60px> Collections <span style="font-weight: normal;"> - Hash Maps Ownership and updates</span></h2>

- Inserts take ownership of the key and value (see [compiler error](https://github.com/simlal/ift-769-self-learning-intro-to-rust/tree/main/projects/hashmaps_test)). Can use references but must be valid for the lifetime of the map.
- Overwrites the value if the key already exists. Use `entry` to insert instead.

```rust
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.entry(String::from("Yellow")).or_insert(50);  // `entry` returns Entry enum
scores.entry(String::from("Blue")).or_insert(50);  // `or_insert` returns a mutable reference
println!("{scores:?}");    // {"Blue": 10, "Yellow": 50}
```
Mutable references by `entry.or_insert` to update value
```rust
let text = "hello world wonderful world";
let mut map = HashMap::new();                                                            // {}

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;   // Dereference count to update the value
}
println!("{map:?}");    // {}
```

---

<h2><img src="https://em-content.zobj.net/source/microsoft-teams/337/warning_26a0-fe0f.png" width=60px> Error Handling <span style="font-weight: normal;"> - (Un)recoverable Errors overview </span><img src="https://em-content.zobj.net/source/google/387/police-car-light_1f6a8.png" width=60px></h2>

<p>Two types of <span style="color:red;">errors</span> in Rust:</p>

1. **Recoverable errors** are handled by `Result<T, E>` enum
2. **Unrecoverable errors** are handled by `panic!` macro
<br>

Let's us handle errors in a way that can be made explicit in the function signature.

---

<h2><img src="https://em-content.zobj.net/source/google/387/police-car-light_1f6a8.png" width=60px> Error Handling <span style="font-weight: normal;"> - Unrecoverable Errors and <code>panic!</code></span></h2>

Full backtrace on demand when a program encounters an error that it cannot handle.

```rust
fn main() {
    let my_vec: Vec<i32> = vec![1, 2, 3];
    println!("{my_vec[99]}"); // Panics! Index out of bounds
}
```

When running with `RUST_BACKTRACE=1 cargo run`, the error message will include a backtrace. See next slide for the output:

--- 

<pre style="background-color: #2d2d2d; color: #f8f8f2; padding: 10px; border-radius: 5px; font-size: 0.53em; white-space: pre-wrap;">
<code>
~/out_of_bounds_panic main !1 ?1 ❯ <span style="color: #66d9ef;">RUST_BACKTRACE=1 cargo run</span>
    <span style="color: #a6e22e;">'Finished'</span> `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     <span style="color: #a6e22e;">'Running'</span> `target/debug/out_of_bounds_panic`
my_deque[0]: a
<span style="color: #f92672;">thread 'main' panicked at src/main.rs:8:42:</span>
Out of bounds access
stack backtrace:
   0: rust_begin_unwind
             at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/panicking.rs:652:5
   1: core::panicking::panic_fmt
             at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/core/src/panicking.rs:72:14
   2: core::panicking::panic_display
             at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/core/src/panicking.rs:263:5
   3: core::option::expect_failed
             at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/core/src/option.rs:1994:5
   4: core::option::Option<T>::expect
             at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/core/src/option.rs:895:21
   5: &lt;alloc::collections::vec_deque::VecDeque<T,A> as core::ops::index::Index&lt;usize&gt;&gt;::index
             at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/alloc/src/collections/vec_deque/mod.rs:2784:25
   6: out_of_bounds_panic::main
             at ./src/main.rs:8:42
   7: core::ops::function::FnOnce::call_once
             at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with <span style="color: #66d9ef;">`RUST_BACKTRACE=full`</span> for a verbose backtrace.
</code>
</pre>

---

<h2><img src="https://em-content.zobj.net/source/microsoft-teams/337/warning_26a0-fe0f.png" width=60px> Error Handling <span style="font-weight: normal;"> - Recoverable Errors and <code>Result</code></span></h2>

- `Result<T, E>` is an enum with two variants: `Ok(T)` and `Err(E)` avail from prelude
- Use match to handle the `Result` enum (both `Ok` and `Err` cases)

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt");                 // Returns Result<File, Error>

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {                              // ErrorKind enum
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,                                           // Returns the file handle when created
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => {                                            // Choose to panic for other possible ErrorKind
                panic!("Problem opening the file: {other_error:?}");
            }
        },
    };
}
```

---

<h2><img src="https://em-content.zobj.net/source/microsoft-teams/337/warning_26a0-fe0f.png" width=60px> Error Handling <span style="font-weight: normal;"> - Unwrap and expect shortcuts</span></h2>

Almost similar behaviors as `match` expression but more concise:

- `unwrap()` returns the value if `Ok` or panics with the error message
- `expect()` is similar but allows you to specify the error message

```rust
// Panics with the error message if missing file
let f = File::open("hello.txt").unwrap();   

// Same as above but with custom message
let f = File::open("hello.txt").expect("Failed to open hello.txt");  
```

---

<h2><img src="https://em-content.zobj.net/source/microsoft-teams/337/warning_26a0-fe0f.png" width=60px> Error Handling <span style="font-weight: normal;"> - Propagating errors</span></h2>

Return the error to the calling function using type `Result<T, E>` in the function signature.

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {     // Take note of Result enum
    let username_file_result = File::open("hello.txt");     

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();
    // Possible to call `.read_to_string()` directly on the Result enum 
    // even when Err. This is Propagating the error
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    } // Returns the username or the error
}
```

---

<h2><img src="https://em-content.zobj.net/source/microsoft-teams/337/warning_26a0-fe0f.png" width=60px> Error Handling <span style="font-weight: normal;"> - Propagating shortcut with <code>?</code> operator</span></h2>

Use of `?` operator to propagate errors and reduce boilerplate code. It is a shortcut for `match` and `return` the error.

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}
```

At each `?`, the error is returned to the calling function if it is of `Err` type. Otherwise, the value is unwrapped and returned.

---

<h2><img src="https://em-content.zobj.net/source/microsoft-teams/337/warning_26a0-fe0f.png" width=60px> To panic! or not to panic!? <span style="font-weight: normal;"></span><img src="https://em-content.zobj.net/source/google/387/police-car-light_1f6a8.png" width=60px></h2>

When writing library/API code ->better to return `Result` (let caller handle error)...

...But we can use `panic!` when testing, when the state is invalid, calling external code etc.
<br> 

Take advantage of Rust's <span style="color: orange;">strong type system</span> to catch errors at compile time (more concise code) and use <span style="color: orange;">`Result` enum</span> to handle errors at runtime.

---
<h2><img src="https://em-content.zobj.net/source/microsoft-teams/337/warning_26a0-fe0f.png" width=60px> OOP-styled safe design example <span style="font-weight: normal;"></span><img src="https://em-content.zobj.net/source/google/387/police-car-light_1f6a8.png" width=60px></h2>


```rust
pub struct Guess {
    value: i32,
}

impl Guess {   // Interface for the struct with constructor and getter
    // Constructor panic if value is out of bounds
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");        
        }
        Guess { value }
    }
    // borrows self to get the value
    pub fn value(&self) -> i32 {  
        self.value   // value is a private, cannot be modified
    }
}
```

Calling `Guess` constructor will panic if out-of-bounds (should be in API docs). `value` can never return an invalid value of an invalid type.


---

<h2><img src="https://em-content.zobj.net/source/google/387/puzzle-piece_1f9e9.png" width=60px> Generics <span style="font-weight: normal;"> - Data type overview</span></h2>

- **Generics** allow you to define _functions_, _structs_, _enums_, and _methods_ that work with any data type.
- Like C++ templates, but more type constraints (shared trait) and safety (borrow checker)

**Example of a function that would :**
- Take a reference to a slice of any type
- Return a reference to the an object of the same type
```rust
fn smallest<T>(list: &[T]) -> &T {...} // 
```

---

<h2><img src="https://em-content.zobj.net/source/google/387/puzzle-piece_1f9e9.png" width=60px> Generics<span style="font-weight: normal;">... Must implement traits</span></h2>

**Traits** are similar to interfaces in other languages. They define a set of methods that a type must implement.

A function cannot use a generic type `T` unless it knows that `T` implements a specific trait.

```rust
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {   // Error! T does not implement the `PartialOrd` trait
            largest = item;
        }
    }
    largest
}

fn main() {    // Won't compile!!!
    let number_list = vec![34, 50, 25, 100, 65];                                                            
    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {result}");
}
```
---

<h2><img src="https://em-content.zobj.net/source/google/387/puzzle-piece_1f9e9.png" width=60px> Generics<span style="font-weight: normal;"> - Struct example</span></h2>

Use a single generic type `T` for both fields of the struct. Each field cannot mix types.

```rust
struct BadPoint<T> {
    x: T,
    y: T,
}
let wont_work = BadPoint { x: 5, y: 4.0 };

struct GoodPoint<T, U> {   // `U` is just another generic type
    x: T,
    y: U,
}
let will_work = GoodPoint { x: 5, y: 4.0 };
```
---

<h2><img src="https://em-content.zobj.net/source/google/387/puzzle-piece_1f9e9.png" width=60px> Generics<span style="font-weight: normal;"> - Enum example</span></h2>

stdlib provides `Option` and `Result` enums that use generics. They can expression the presence or absence of a value or the success or failure of an operation.

`Result` even uses multiple generics for its `Ok` and `Err` variants.

```rust
enum Option<T> {
    Some(T),    // Holds a value of type T
    None,       // Does not hold a value
}
enum Result<T, E> {
    Ok(T),      // Holds a value of type T
    Err(E),     // Holds a value of Type E (error)
}
```

---

<h2><img src="https://em-content.zobj.net/source/google/387/puzzle-piece_1f9e9.png" width=60px> Generics<span style="font-weight: normal;"> - Method definitions</span></h2>
Methods written within an `impl` that declares the generic type will be defined on any instance of the type regardless of concrete type substituted.

```rust
struct Point<T> {
    x: T,
    y: T,
}
impl<T> Point<T> {    // Can be used by any Point struct
    fn x(&self) -> &T {
        &self.x
    }
}
impl Point<f32> {   // Specific to Point<f32> structs
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };    // Cannot use `.distance_from_origin()` because `Point<i32>`
    println!("p.x = {}", p.x());
    let p2 = Point { x: 5.0, y: 10.0 };
    println!("Distance from origin: {p2.distance_from_origin()}");
}
```

---
<h2><img src="https://em-content.zobj.net/source/google/387/hammer-and-wrench_1f6e0-fe0f.png" width=60px> Traits <span style="font-weight: normal;"> - Shared behavior</span></h2>

TODO


---

<!-- Lifetoimes -->

<h2><img src="https://em-content.zobj.net/source/google/387/ring-buoy_1f6df.png" width=60px> Lifetimes <span style="font-weight: normal;"> - Overview</span></h2>

TODO

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

<p style="text-align: center; font-size: 2em;">🚀 Demo Time!
<br>
<p style="text-align: center; font-size: 1.5em;"> I/O CLI program `grep` clone 🧑🏻‍💻</p>

---


<h2><img src="https://em-content.zobj.net/source/google/387/gear_2699-fe0f.png" width=60px> THEORY STUFF <span style="font-weight: normal;"></span></h2>

TODO


