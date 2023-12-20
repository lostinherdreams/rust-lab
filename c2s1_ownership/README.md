# Rust Ownership and Borrowing

This Rust project demonstrates fundamental concepts of ownership and borrowing in Rust programming.

## Overview

Rust's memory management revolves around the concept of ownership. The code in this project showcases:

### Ownership and Heap Memory

Illustrates how variables live in the stack while dynamically allocated data, such as arrays using `Box`, resides in the heap. It demonstrates ownership transfer when moving values between variables.

### Transfer of Ownership

Shows how Rust enforces a strict ownership model, preventing the use of variables after their ownership has been transferred.

### Cloning for Avoiding Moves

Demonstrates the `clone()` method for types like `String`, allowing the creation of deep copies to avoid ownership transfers and retain access to the original data.

## Usage

To run this Rust project, clone the repository:

```bash
git clone https://github.com/yourusername/rust-ownership-borrowing.git
```
