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
- Covers the basics (syntax, types, functions)
- Build tool and package manager (Cargo)
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

<h2><img src="https://em-content.zobj.net/source/google/387/woman-technologist_1f469-200d-1f4bb.png" width=60px> Installation and setup</h2>

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
<h2><img src="https://em-content.zobj.net/source/google/387/man-technologist_1f468-200d-1f4bb.png" width=60px> Setup example<span style="font-weight: normal;"> Hello World! (1/2)</span></h2>

**Env setup and features:**
- Easy install: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Rustup for managing toolchains: `rustup update`
- Included formatter: `rustfmt --check src/main.rs` (dry-run mode)
- Cargo for building and managing projects: `cargo new project_name`
- Quality of life with `rust-analyzer`: LSP, build/debug IDE support etc.

---
<h2><img src="https://em-content.zobj.net/source/google/387/man-technologist_1f468-200d-1f4bb.png" width=60px> Building with cargo <span style="font-weight: normal;"> Hello World! (2/2)</span></h2>

To **initialize** a new project use `cargo new hello_world`. The **structure** will include a `src/` dir for code, `Cargo.toml` config file, `Cargo.lock` for dependencies and version and `target/` for build artifacts:

**Useful Cargo commands when building a project:**
- `cargo build` or `cargo run` to compile and run the project. Use `--release` flag for compilation with optimizations inside `target/release/`
- `cargo check`: Check the project for errors without building
- `cargo doc`: Generate documentation for the project
- `cargo clean`: Remove build artifacts
- `cargo update`: Update dependencies
- `cargo fmt`: Format the code according to the Rust style guidelines
- `cargo test`: Run tests in the project
---

<h2><img src="https://em-content.zobj.net/source/google/387/gear_2699-fe0f.png" width=60px> EXAMPLE TITLE <span style="font-weight: normal;"></span></h2>

TODO

---

<!-- input and output for helloworld via usb or uart -->
<h2><img src="https://em-content.zobj.net/source/google/387/keyboard_2328-fe0f.png" width=60px> GREP PROJ? <span style="font-weight: normal;">TODO</span></h2>

TODO

---
