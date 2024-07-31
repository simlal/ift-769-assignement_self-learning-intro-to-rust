# Self-learning introduction to Rust programming language

## Table of contents

<details closed>
<summary><a href="#introduction">Introduction</a></summary>

- [Assignement requirements](#assignement-requirements-memo)
- [Personal objectives](#personal-objectives-dart)

</details>

<details closed>
<summary><a href="#development-environment-setup-hammer_and_wrench">Development environment setup</a></summary>

- [Installation](#installation)
- [Package manager (Cargo) and build tools](#package-manager-cargo-and-build-tools)
- [Project structure with Crates and Modules](#project-structure-with-crates-and-modules)

</details>

<details>
<summary><a href="#applied-projects-mag">Applied projects</a></summary>

- [A simple guessing game CLI app](#a-simple-guessing-game-cli-app-gamedie)
- [Write an I/O CLI `grep`-clone program](#write-an-io-cli-grep-clone-program-wrench)

</details>

<details>
<summary><a href="#conclusion">Conclusion</a></summary>

</details>

<details>
<summary><a href="#references-books">References</a></summary>

</details>

## Introduction

<span style="color:orange">**Rust**</span> [[0]](#0) is a modern programming language that focuses on <span style="color:orange">memory safety, speed, and concurrency</span>. It supports *multiple programming paradigms* and can be used for *multiple purposes* (i.e. systems programming, backend/server-side development, CLI tools etc.). Rust is also known for its *ownership system* that allows for memory safety without a garbage collector.

This project is part of a course on self-driven learning of theoritical concepts of computer science. I choose to to follow along the Rust book and implementing the projects as an introduction to the language. A work-inprogress `marp` presentation about the summary of concepts learned and implemented projects can is hosted here: https://simlal.github.io/ift-769-self-learning-intro-to-rust/ [[2]](#2). A PDF version can also be found [here](./docs/project-presentation.pdf).

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
- [x] **Project #1:** Write an I/O CLI program (`grep` clone)


## Development environment setup :hammer_and_wrench:

Easy to install and use, Rust comes with a package manager called Cargo that helps with building, testing and managing Rust projects. The Rust compiler is called `rustc` and is used to compile Rust code.

The Rust toolchain includes the following tools:
- `rustc`: Rust compiler
- `cargo`: Package manager and build tool
- `rustup`: Rust toolchain manager

See ***Project #0: Guessing Game*** goes through the process of setting up the development environment, creating a project and some of the basic features of the language.

## Applied projects :mag:

Along with the theoritical concepts, the book provides practical projects to apply the concepts learned. The projects are designed to be simple and build upon each other to demonstrate the capabilities of the language.


### A simple guessing game CLI app :game_die:

TODO


### Write an I/O CLI `grep`-clone program :wrench:

TODO

## Conclusion

TODO

## References :books:
<!-- As numbered footnotes-->
<a id="0">[0]</a> **Rust Programming Language**. Rust Foundation. 2024. https://www.rust-lang.org/
<a id="1">[1]</a> **The Rust Programming Language**. Klabnik, Steve, and Carol Nichols. 2nd ed., No Starch Press.

<a id="2">[2]</a> **Introduction to Rust Programming Language**. Simon Lalonde. 2024. GitHub Pages. https://simlal.github.io/ift-769-self-learning-intro-to-rust/

<a id="3">[3]</a> **Introduction to Rust Programming Language**. Simon Lalonde. 2024. Markdown source code of presentation.

<a id="4">[4]</a> **Rust - Structs, Functions and Methods**. Gian Lorenzetto. 2021. https://gian-lorenzetto.medium.com/rust-structs-functions-and-methods-d60fd597d956