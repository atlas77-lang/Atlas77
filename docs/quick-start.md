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

> Do know that strings are not implemented yet, so this is just an example of how it will look like

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
    print_int(x); // Output: 10

    const y: i64 = 5;
    y = 10; // Error: Cannot assign to a constant variable
}
```

## 5. Data Types

Atlas77 has several built-in data types, including integers, floating-point numbers, booleans, strings, and arrays. The following table lists the built-in data types in Atlas77:

| Data Type | Description | State |
| --------- | ----------- | ----- |
| `i8`      | 8-bit signed integer | 💤 |
| `i16`     | 16-bit signed integer | 💤 |
| `i32`     | 32-bit signed integer | 💤 |
| `i64`     | 64-bit signed integer | ✅ |
| `isize`   | Platform-dependent signed integer | 💤 |
| `u8`      | 8-bit unsigned integer | 💤 |
| `u16`     | 16-bit unsigned integer | 💤 |
| `u32`     | 32-bit unsigned integer | 💤 |
| `u64`     | 64-bit unsigned integer | ✅ |
| `usize`   | Platform-dependent unsigned integer | 💤 |
| `f32`     | 32-bit floating-point number | 💤 |
| `f64`     | 64-bit floating-point number | ✅ |
| `bool`    | Boolean value (`true` or `false`) | ✅ |
| `char`    | Unicode character | 💭 |
| `str`     | String | 💭 |
| `array`   | Array (syntax: `[YourType]`) | 💭 |

> Note: The `str` and `array` types will be mutable and resizable. More powerful types for both Strings and Arrays will be implemented in the future (e.g., `Vec<YourType>` for arrays & `String`).
>
> NB: The `char` type is not implemented yet, but it will be a 32-bit Unicode character.
>
> NB 2: Since this is a VM-based language, all numeric types smaller than 64 bits (e.g., u8, u16, u32) are internally represented as 64-bit values for simplicity and consistency. However, they will behave as their original types, respecting their size and overflow semantics. In the future, packed types may be introduced to optimize memory usage for arrays (e.g., ``[u8]`` could be represented as ``[u8x8]``, ``[u16]`` as ``[u16x4]``, etc.). At present, the minimum memory size for numeric types is 8 bytes.


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
| `if` statement    | Conditional statement | ✅ |
| `match` expression | Pattern matching expression | 💤 |
| `while` loop      | Loop with a condition | ✅ |
| `for` loop        | Loop over a range or collection | 💤 |

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

As of writing this document, the following standard libraries are available:

- `std/io`: Input/output functions
    - `print_int(i: i64)`: Print an integer to the console
    - `print_float(f: f64)`: Print a floating-point number to the console
    - `print_uint(u: u64)`: Print an unsigned integer to the console
    - `print_bool(b: bool)`: Print a boolean value to the console

> Yes it is very limited, but tbh `str`, `array` & structs are still not implemented, so it's a bit hard to implement more complex functions.