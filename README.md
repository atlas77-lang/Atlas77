<a id="readme-top"></a>

<!-- PROJECT SHIELDS -->

[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]

<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/atlas77-lang/Atlas77">
    <img src="images/logo.png" alt="Logo" width="80" height="80">
  </a>

  <h3 align="center">Atlas77</h3>

  <p align="center">
    Functional Programming language with a strong interop with Rust,
    designed to be a functional scripting language.
    <br />
    <a href="https://github.com/atlas77-lang/Atlas77"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://github.com/atlas77-lang/Atlas77">Playground (inexistant)</a>
    ·
    <a href="https://github.com/atlas77-lang/Atlas77/issues/new?labels=bug&template=bug-report---.md">Report Bug</a>
    ·
    <a href="https://github.com/atlas77-lang/Atlas77/issues/new?labels=enhancement&template=feature-request---.md">Request Feature</a>
  </p>
</div>



<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#goal-of-the-language">Goal of the language</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
  </ol>
</details>



<!-- ABOUT THE PROJECT -->
## About The Project

Atlas77 is a functional programming language with a strong interop with Rust.
It is designed to be a high-level language with a strong focus on performance and safety. 


<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- GETTING STARTED -->
## Getting Started

### Prerequisites

* Rust Compiler
  ```sh
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

Or directly from their website: [Rust](https://www.rust-lang.org/tools/install)

### Installation

1. Install it from Cargo
    ```sh
    cargo install atlas_77
    ```
2. Use it as a CLI
    ```sh
    atlas_77 --help
    ```
3. Enjoy!

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- USAGE EXAMPLES -->
## Usage

### Fibonacci Example
```ts
let fib: (i64) -> i64 = \ n ->
  match n
  | 0 ~> 0
  | 1 ~> 1
  \ _ ~> fib(n - 1) + fib(n - 2)

let main: () -> i64 = \ _ -> fib(10) //> 55
```

_For more examples, please refer to the [examples folder](https://github.com/atlas77-lang/Atlas77/tree/main/examples)_

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- ROADMAP -->
## Roadmap

### v0.3 "Foundation"
- [v0.3](https://github.com/atlas77-lang/Atlas77/releases/tag/v0.3)
  - [x] Variable assignments
  - [x] Function declarations
  - [x] Conditional expressions (`if-else`)
  - [x] Recursion
  - [x] Basic arithmetic operations
  - [x] `do..end` blocks to allow multiple expressions
  - [x] Basic runtime & memory system
- [v0.3.1](https://github.com/atlas77-lang/Atlas77/releases/tag/v0.3.1)
  - [x] Support for `string` & `List[T]` types (including indexing and concatenation)
  - [x] Basic CLI support
  - [x] Basic stdio functions
  - [x] `match` expressions

### v0.4 "Keystone"
- [v0.4 "Keystone"](https://github.com/atlas77-lang/Atlas77/tag/v0.4)
  - [x] Support for `struct` types
  - [x] Improved runtime & memory system (~80% performance uplift)
  - [x] Support for external functions (Rust interop)
  - [x] Expanded standard library using external functions

### v0.5 Phoenix Release Timeline
The v0.5 is a complete rewrite of Atlas77, it aligns better with the end goal of the language (see _Goal of the language_).
#### v0.5-alpha1: Core Foundations
- Functions: Define and call functions.
- Variables: Immutable and mutable variables.
- Basic Standard Library: Core utilities for:
  - Time operations
  - File handling
  - Input/Output (I/O)
  - Math functions
  - String manipulations
  - List manipulations
- Include Directive: Limited to standard library imports for now.
- Control Flow:
  - match expressions for pattern matching.
  - if/else statements for conditional logic.
  - while loops for iteration.

#### v0.5-alpha2: Data Structures and Lambdas
- Lambdas & Closures: Inline, anonymous functions with captured variables. (May be split into a separate alpha release.)
- Structs: User-defined types with named fields.
- Unions: Low-level data structures allowing overlapping memory layouts.
- Enums: Enumerations with optional associated data for flexible value sets.

#### v0.5-alpha3: Classes and Memory Management
- Classes: Object-oriented programming support.
- Pointers: Basic pointer manipulation for low-level programming.
- Simple Memory Management: Automatic deallocation of resources after their scope ends, except for returned values or objects with ownership ties.

#### v0.5-alpha4: Type Safety
- Basic Type Checking: Initial support for catching type mismatches at compile time.

#### v0.5-beta: Stability and Refinement
- The beta phase will focus on stabilizing the language. All features will be finalized, tested extensively, and optimized for real-world use. This phase will serve as a release candidate.
Future Versions

#### Planned features for v0.6 and beyond:

- Generics: Support for reusable, type-agnostic code.
- Traits: Define shared behavior across different types.
- Macros: Compile-time code generation and metaprogramming.
- Multi-File Projects: Extend the include directive to support user-defined modules across multiple files.


See the [open issues](https://github.com/atlas77-lang/Atlas77/issues) for a full list of proposed features (and known issues).

<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- GOAL OF THE LANGUAGE -->
## Goal of the language

- Boostrapping the compiler
- Making a simple ECS
- Making a simple Game Engine with Vulkan bindings (maybe OpenGL too) 
> At least it should be possible to make one with Atlas77
- Using BlueEngine from the language (even if it's not really most efficient)
- Ahead of time compilation using cranelift
- Making the package manager directly in Atlas77

<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- CONTRIBUTING -->
## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

### Top contributors:

<a href="https://github.com/atlas77-lang/atlas77/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=atlas77-lang/atlas77" />
</a>

Made with [contrib.rocks](https://contrib.rocks).

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- LICENSE -->
## License

Distributed under the MIT License. See `LICENSE.txt` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- CONTACT -->
## Contact

Your Name - [@Gipson62_8015](https://twitter.com/Gipson62_8015) - J.H.Gipson62@gmail.com

Project Link: [https://github.com/atlas77-lang/Atlas77](https://github.com/atlas77-lang/Atlas77)

<p align="right">(<a href="#readme-top">back to top</a>)</p>




<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/atlas77-lang/Atlas77.svg?style=for-the-badge
[contributors-url]: https://github.com/atlas77-lang/Atlas77/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/atlas77-lang/Atlas77.svg?style=for-the-badge
[forks-url]: https://github.com/atlas77-lang/Atlas77/network/members
[stars-shield]: https://img.shields.io/github/stars/atlas77-lang/Atlas77.svg?style=for-the-badge
[stars-url]: https://github.com/atlas77-lang/Atlas77/stargazers
[issues-shield]: https://img.shields.io/github/issues/atlas77-lang/Atlas77.svg?style=for-the-badge
[issues-url]: https://github.com/atlas77-lang/Atlas77/issues
[license-shield]: https://img.shields.io/github/license/atlas77-lang/Atlas77.svg?style=for-the-badge
[license-url]: https://github.com/atlas77-lang/Atlas77/blob/master/LICENSE.txt

