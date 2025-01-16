NB: This is a work in progress document. The syntax is subject to change.

# Current Syntax of Atlas77
## 1. Introduction

Atlas77 is a simple, easy-to-use, and powerful programming language. It is designed to be easy to learn and use, while still being powerful enough to handle complex tasks. This document describes the syntax of Atlas77, including the rules for writing code in the language (WIP).

## 2. Hello, World!

Here is a simple "Hello, World!" program written in Atlas77:

```ts
import "std/io"

func main() -> i64 {
    print("Hello, World!")
}
```

Save this code to a `.atlas` file, then run it directly with `atlas run <FILE_PATH>`

## 3. Comments

Comments in Atlas77 are similar to comments in other programming languages. There are two types of comments: single-line comments and multi-line comments.

# 3.1. Single-line Comments

Single-line comments start with `//` and continue until the end of the line. For example:

```rs
// This is a single-line comment

let x: i64 = 5; // This is also a single-line comment
```

# 3.2. Multi-line Comments

Multi-line comments start with `/*` and end with `*/`. For example:
> NB: Multi-line comments aren't supported yet (you'll see a lot of WIPs in this document)
```rs
/*
This is a multi-line comment.
    /*
        NOTE: Multi-line comments can be nested.
    */

It can span multiple lines.
*/
```

> Comments are parsed as tokens by the compiler, to allow future documentation features.


## 4. Variables

Variables in Atlas77 are either mutable or immutable. The design follows in some sense TypeScript/JavaScript, with the `const` & `let` keywords. Variables can be declared using the `let` keyword, which creates a mutable variable, or the `const` keyword, which creates an immutable variable.

```ts
import "std/io"

func main() -> i64 {
    let x: i64 = 5;
    x = 10;
    print(x); // Output: 10

    const y: i64 = 5;
    y = 10; // Error: Cannot assign to a constant variable
}
```

## 5. Data Types

Atlas77 has several built-in data types, including integers, floating-point numbers, booleans, strings, and arrays. The following table lists the built-in data types in Atlas77:

| Data Type | Description | State |
| --------- | ----------- | ----- |
| `i8`      | 8-bit signed integer | WIP |
| `i16`     | 16-bit signed integer | WIP |
| `i32`     | 32-bit signed integer | WIP |
| `i64`     | 64-bit signed integer | Done |
| `isize`   | Platform-dependent signed integer | WIP |
| `u8`      | 8-bit unsigned integer | WIP |
| `u16`     | 16-bit unsigned integer | WIP |
| `u32`     | 32-bit unsigned integer | WIP |
| `u64`     | 64-bit unsigned integer | Done |
| `usize`   | Platform-dependent unsigned integer | WIP |
| `f32`     | 32-bit floating-point number | WIP |
| `f64`     | 64-bit floating-point number | Done |
| `bool`    | Boolean value (`true` or `false`) | Done |
| `char`    | Unicode character | WIP |
| `str`     | String | WIP |
| `array`   | Array | WIP |

> Note: The `str` type is a sequence of Unicode characters and is immutable, later on a `String` type will be introduced as a more flexible alternative to `str` a bit like Rust's `String` & `&str`. The `array` type is a fixed-size collection of elements of the same type (e.g., `[i64; 5]`).

## 6. Functions

Functions in Atlas77 are defined using the `func` keyword, followed by the function name, parameters, return type, and body. The return type of a function is specified after the `->` symbol. For example:

```ts
import "std/io"

func add(x: i64, y: i64) -> i64 {
    return x + y;
}

func main() -> i64 {
    let result: i64 = add(5, 10);
    print(result); // Output: 15
}
```

## 7. Control Structures

Atlas77 supports several control structures, including `if` statements, `match` expression, `while` loops, and `for` loops. The syntax for these control structures is similar to other programming languages. For example:

| Control Structure | Description | State |
| ----------------- | ----------- | ----- |
| `if` statement    | Conditional statement | Done |
| `match` expression | Pattern matching expression | WIP |
| `while` loop      | Loop with a condition | Done |
| `for` loop        | Loop over a range or collection | WIP |

> Note: Nested if-else (i.e. `if {} else if {} else {}`) isn't supported yet.

```ts
import "std/io"

func main() -> i64 {
    let x: i64 = 5;

    if x > 0 {
        print("x is positive");
    } else {
        if x < 0 {
            print("x is negative");
        } else {
            print("x is zero");
        }
    }

    let i: i64 = 0;
    while i < 5 {
        print(i);
        i += 1;
    }
}
```

## 8. The standard library

Atlas77 comes with a relatively small standard library, which includes functions & types for input/output, file handling, string & list manipulation, time & math functions. The standard library is imported using the `import` keyword, followed by the library name. For example:

```ts
import "std/io"

func main() -> i64 {
    print("Hello, World!");
}
```