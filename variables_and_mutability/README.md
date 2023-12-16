# Rust Concepts Demo

This Rust code demonstrates the concepts of `constant variables`, `Mutability`, and `shadowing` in Rust programming.

## Table of Contents

- [Introduction](#introduction)
- [Usage](#usage)
- [Concepts Illustrated](#concepts-illustrated)
- [Code Explanation](#code-explanation)

## Introduction

This repository contains Rust code showcasing the usage of constants, mutability, and shadowing. These concepts are fundamental in Rust and are crucial for understanding the language's behavior regarding variable assignments, immutability, and constants.

## Usage

To run the code, follow these steps:

1. Ensure you have Rust installed on your system.
2. Clone this repository to your local machine.
3. Navigate to the directory containing the code.
4. Run the code using `cargo run`.

## Concepts Illustrated

### Mutable Variables (`mut`)

The code demonstrates how to create mutable variables in Rust using the `mut` keyword. It showcases the ability to change the value of a mutable variable and the necessity of explicitly declaring mutability.

### Constants (`const`)

It explains the usage of constants in Rust, declared using the `const` keyword. Constants are immutable by default and must have their type specified. The code showcases the advantage of constants in maintaining fixed values throughout the program's execution.

### Shadowing

The code illustrates the concept of shadowing in Rust, allowing redeclaration of a variable within the same scope. It shows how shadowing differs from mutability, allowing variable redefinition with different types and values.

## Code Explanation

- `get_user_input()` function retrieves user input as a `u32` type, demonstrating the parsing of input.
- Various examples are provided within the `main()` function, illustrating mutability, constants, and shadowing.
