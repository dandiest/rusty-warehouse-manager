
<p align="center">
  <img src="https://img.shields.io/badge/License-MIT-yellow.svg" />
  <img src="https://img.shields.io/badge/Language-Rust-orange.svg" />
  <img src="https://img.shields.io/badge/Status-Project_Complete-brightgreen.svg" /> 
</p>

<h1 align="center">📦 Rust Warehouse Manager - Inventory Management System</h1>

<p align="center">
  A robust Command Line Interface (CLI) tool for dynamic inventory management, demonstrating Rust core concepts: Structs, Enums, and Ownership.
</p>

---

## 🎓 Educational Disclaimer
This repository is a core part of my **Personal Apprenticeship** in Rust. 
* **Purpose**: This project serves as a practical sandbox to master Rust's strict type system and input handling.
* **Evolution**: This code represents my transition from basic syntax to managing complex collections like `Vectors` of `Structs`.
* **Feedback**: I am constantly refining my logic as I work towards mastering memory safety and efficient data management!

## 🌟 Features
* **Dynamic Input**: Users specify exactly how many items to process per session.
* **Type-Safe Inventory**: Leverages a `Category` Enum and `Object` Struct to ensure data integrity.
* **Real-Time Valuation**: Automatically calculates the total warehouse value ($) after every successful entry.
* **Clean UI**: Formatted console output for a professional CLI experience.

## 🛠️ Technical Deep Dive
* **Struct Composition**: The `Object` struct manages `String` ownership and numeric types (`f32`, `u32`) seamlessly.
* **Vector Management**: Efficiently pushes custom objects into a heap-allocated `Vec<Object>`.
* **Input Parsing**: Implements `trim()` and `parse()` chains with robust error handling via `.expect()`.
* **Iterator Logic**: Uses `.iter().map().sum()` patterns to calculate global state values (Total Price).



---

## 🚀 How to Run
1. Clone the repository.
2. Ensure you have [Rust](https://www.rust-lang.org/) installed.
3. Run the following command in your terminal:
   ```bash
   cargo run

    To create a standalone executable:
    Bash

    cargo build --release

## ⚖️ License & Copyright

Copyright (c) 2026 **[dandiest]**

This project is licensed under the MIT License.

*You are free to use, study, and modify this code for educational purposes. Feel free to fork it if you are on your own Rust learning journey!*
