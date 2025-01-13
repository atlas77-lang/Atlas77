NB: This is a work in progress document. The syntax is subject to change.

# Current Syntax of Atlas77
## 1. Introduction

Atlas77 is a simple, easy-to-use, and powerful programming language. It is designed to be easy to learn and use, while still being powerful enough to handle complex tasks. This document describes the syntax of Atlas77, including the rules for writing code in the language (WIP).

## 2. Hello, World!

Here is a simple "Hello, World!" program written in Atlas77:

```cpp
@import "std/io"

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

let x := 5 // This is also a single-line comment
```

# 3.2. Multi-line Comments

Multi-line comments start with `/*` and end with `*/`. For example:

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

