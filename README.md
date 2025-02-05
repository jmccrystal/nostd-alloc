# nostd-alloc

A personal learning project implementing basic memory allocation and collections in a `no_std` Rust environment. This project serves as an educational exploration of low-level memory management, custom allocators, and fundamental data structures without relying on the standard library.

## Overview

This project implements:
- A basic memory allocator using `mmap`
- Core collections like `Vec` and `String`
- Basic I/O functionality for printing
- All functionality in a `no_std` environment

## Components

### Memory Allocator
The `DummyAllocator` provides basic memory allocation functionality:
- Uses `mmap` for acquiring memory pages
- Implements the `GlobalAlloc` trait
- Simple offset-based allocation strategy
- Fixed page size of 4KB (configurable)

### Collections
- `Vec`: A growable array implementation
  - Dynamic resizing
  - Push operations
  - Slice conversions
  - Basic memory management

- `String`: UTF-8 string implementation
  - Built on top of `Vec<u8>`
  - Basic string operations
  - UTF-8 validation

### I/O
- Basic printing functionality
- Trait-based print interface
- Support for printing integers, strings, and collections

## Educational Purpose

This project is purely for learning purposes and explores:
- Low-level memory management in Rust
- Implementation of fundamental data structures
- Working without the standard library
- Understanding Rust's allocation and collection internals

## Limitations

As this is a learning project, it has several limitations:
- Basic allocation strategy without memory reuse
- Limited error handling
- No deallocation implementation yet
- Minimal safety checks
- Not suitable for production use

## Building

This project requires:
- Rust nightly (for `no_std` features)
- Unix-like environment (uses `mmap`)

## Note

This is a personal project created for educational purposes to better understand Rust's memory management, allocators, and collection implementations. It is not intended for production use and may contain bugs or incomplete implementations.
