# Self-learning introduction to Rust programming language

## Table of contents

<details closed>
<summary><a href="#introduction">Introduction</a></summary>

- [Overview](#overview)
- [Assignement requirements](#assignement-requirements-memo)
- [Personal objectives](#personal-objectives-dart)

</details>

<details closed>
<summary><a href="#development-environment-setup-hammer_and_wrench">Development environment setup</a></summary>

</details>

<details>
<summary><a href="#applied-projects-mag">Applied projects</a></summary>

- [guessing game CLI app](#a-simple-guessing-game-cli-app-game_die)
- [I/O CLI `grep`-clone program](#io-cli-grep-clone-program-wrench)

</details>

<details>
<summary><a href="#conclusion">Conclusion</a></summary>

</details>

<details>
<summary><a href="#references-books">References</a></summary>

</details>

## Introduction

### Overview

<span style="color:orange">**Rust**</span> [[0]](#0) is a modern programming language that focuses on <span style="color:orange">memory safety, speed, and concurrency</span>. It supports *multiple programming paradigms* and can be used for *multiple purposes* (i.e. systems programming, backend/server-side development, CLI tools etc.). Rust is also known for its ***ownership and lifetime systems***, ***strong type checking*** that allows for memory safety without a garbage collector safeguarded by the compiler's borrow checker.

This project is half of a course on self-driven learning of theoritical concepts of computer science. I choose to to follow along the Rust book and implementing the projects as an introduction to the language. A `marp` presentation about the summary of concepts learned and implemented projects can is hosted here: https://simlal.github.io/ift-769-self-learning-intro-to-rust/ [[2]](#2). A PDF version can also be found [here](./docs/project-presentation.pdf).

### Assignement requirements :memo:
- [x] Choose a reference manual for a subject of interest in computer science (_The Rust Programming Language_ by Steve Klabnik and Carol Nichols) [[1]](#1)
- [x] Follow the book to learn the concepts and apply to the proposed projects from the book.
- [x] Make a [presentation](https://simlal.github.io/ift-769-self-learning-intro-to-rust/) [[2]](#2) (Markdown source code [[3]](#3)) of the theoritical concepts of Rust.
    See custom scripts within for docs generation automation:
        - `nix-shell` environment: [build-docs.nix](./docs/build-docs.nix)
        - Helpers: [compress-jpgs.sh](./docs/compress-jpgs.sh) + [build-docs.sh](./docs/build-docs.sh).
- [x] Demonstrate with practical projects examples from the book

### Personal objectives :dart:

Learn the basic and specific features of <span style="color:orange">Rust</span> and present them in a clear and concise manner to the which have no prior knowledge of the language.

Gain practical experience by implementing the projects from the book and demonstrate the capabilities of the language in real-world apps.

**Practical projects:**
- [x] **Project #0:** [Guessing game CLI app](./projects/guessing_game/)
- [x] **Project #1:** I/O CLI program `grep` clone: [minigrep](./projects/minigrep/)


## Development environment setup :hammer_and_wrench:

Easy to install and use, Rust comes with a package manager called Cargo that helps with building, testing and managing Rust projects. The Rust compiler is called `rustc` and is used to compile Rust code.

The Rust toolchain includes the following tools:
- `rustc`: Rust compiler
- `cargo`: Package manager and build tool
- `rustup`: Rust toolchain manager

See ***Project #0: Guessing Game*** goes through the process of setting up the development environment, creating a project and some of the basic features of the language.

## Summarizing the key concepts learned

As mentionned earlier, the presentation can be found [here](https://simlal.github.io/ift-769-self-learning-intro-to-rust/). The following are the key concepts learned from the book:
- Basic programming concepts (variables, functions, control flow)
- <span style="color:orange; font-weight: bold;">Ownership, borrowing, and references </span> 
- Specific data types and collections (struct, enums, vectors, strings)
- <span style="color:orange; font-weight: bold;">Error handling and null</span>
- <span style="color:orange; font-weight: bold;">Generics, traits, and lifetimes</span>

## Applied projects :mag:

Along with the theoritical concepts, the book provides practical projects to apply the concepts learned. The projects are designed to be simple and build upon each other to demonstrate the capabilities of the language.


### A simple guessing game CLI app :game_die:

Simple CLI app to showcase the basic programming concepts of Rust (i.e. variables, functions, control flow, error handling, etc.). Game generates a random number, takes input via stdin and compares it to the generated number. The game ends when the user guesses the correct number.

Project can be found [here](./projects/guessing_game/).


### I/O CLI `grep`-clone program :wrench:

Simplified implementation of grep, a CLI tool that searches for a pattern in a file and prints the lines that contain the pattern. The program reads the file and searches for the pattern in the file's contents. The program takes the pattern and file name as command-line arguments.

Project can be found [here](./projects/minigrep/).

## Conclusion

Only scratched the surface of Rust's features as this material covers half of the book! <u>Remaining topics to be covered:</u>
- <span style="color: orange">Closures, iterators, smart pointers, advanced pattern matching and concurrency </span> (language features)
- <span style="color: DarkCyan">Testing, documentation, and cargo </span> (tooling)

Still, we have seen how Rust's <span style="color: orange">strong type system, ownership, and borrowing</span> and why it is considered a strong candidate for replacing C/C++ in systems programming, backends, and high-performance applications.

Regardless of the learning curve, because of its performance and memory safety, **Rust is being more adopted in both academia and industry.**[[5]](#5)


## References :books:
<!-- As numbered footnotes-->
<a id="0">[0]</a> **Rust Programming Language**. Rust Foundation. 2024. https://www.rust-lang.org/
<a id="1">[1]</a> **The Rust Programming Language**. Klabnik, Steve, and Carol Nichols. 2nd ed., No Starch Press.

<a id="2">[2]</a> **Introduction to Rust Programming Language**. Simon Lalonde. 2024. GitHub Pages. https://simlal.github.io/ift-769-self-learning-intro-to-rust/

<a id="3">[3]</a> **Introduction to Rust Programming Language**. Simon Lalonde. 2024. Markdown source code of presentation.

<a id="4">[4]</a> **Rust - Structs, Functions and Methods**. Gian Lorenzetto. 2021. https://gian-lorenzetto.medium.com/rust-structs-functions-and-methods-d60fd597d956

<a id="5">[5]</a> **Industry and academia support**. Rust For Linux. 2024. https://rust-for-linux.com/industry-and-academia-support