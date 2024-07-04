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

<h2><img src="https://em-content.zobj.net/source/google/387/gear_2699-fe0f.png" width=60px> Functions <span style="font-weight: normal;"></span></h2>

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


