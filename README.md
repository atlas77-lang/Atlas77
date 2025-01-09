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
  <a href="https://github.com/RedGear-Studio/Atlas77">
    <img src="images/logo.png" alt="Logo" width="80" height="80">
  </a>

  <h3 align="center">Best-README-Template</h3>

  <p align="center">
    Functional Programming language with a strong interop with Rust
    <br />
    <a href="https://github.com/RedGear-Studio/Atlas77"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://github.com/RedGear-Studio/Atlas77">View Demo</a>
    ·
    <a href="https://github.com/RedGear-Studio/Atlas77/issues/new?labels=bug&template=bug-report---.md">Report Bug</a>
    ·
    <a href="https://github.com/RedGear-Studio/Atlas77/issues/new?labels=enhancement&template=feature-request---.md">Request Feature</a>
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
let fib: (n: int) -> int = 
  match n
  | 0 ~> 0
  | 1 ~> 1
  \ _ ~> fib(n - 1) + fib(n - 2)

let main: () -> int = fib(10) //> 55
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

### Upcoming Features
- [ ] Support for `enum` types:
  - [ ] Base enums (standard integer enums)
  - [ ] Advanced enums (enums with associated data)
- [ ] Support for structs with named fields
- [ ] Type Checking:
  - [ ] High-Level Intermediate Representation (HLIR)
  - [ ] Defining external functions
  - [ ] Compile-time traits (e.g., `+`, `-`, `*`, `/`, `&`)
  - [ ] Type inference
- [ ] New and improved runtime:
  - [ ] Garbage collection
  - [ ] Memoization
  - [ ] Concurrency/parallelism by default for pure functions
- [ ] Interoperability with other languages (e.g., C):
  - [ ] Support for loading shared libraries
- [ ] Differentiation between "pure" and "impure" functions:
  - [ ] Ability to mark impure functions for side effects
- [ ] Syntax rework for functions:
  - Replace `let fib: (n: int) -> int = ...` with `let fib: (int) -> int = \n -> ...` to remove argument names in type definitions.
  - [ ] New Abstract Syntax Tree (AST)
  - [ ] New parser



See the [open issues](https://github.com/RedGear-Studio/Atlas77/issues) for a full list of proposed features (and known issues).

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

<a href="https://github.com/RedGear-Studio/Atlas77/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=RedGear-Studio/Atlas77" alt="contrib.rocks image" />
</a>

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- LICENSE -->
## License

Distributed under the MIT License. See `LICENSE.txt` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- CONTACT -->
## Contact

Your Name - [@Gipson62_8015](https://twitter.com/Gipson62_8015) - J.H.Gipson62@gmail.com

Project Link: [https://github.com/RedGear-Studio/Atlas77](https://github.com/RedGear-Studio/Atlas77)

<p align="right">(<a href="#readme-top">back to top</a>)</p>




<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/RedGear-Studio/Atlas77.svg?style=for-the-badge
[contributors-url]: https://github.com/RedGear-Studio/Atlas77/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/RedGear-Studio/Atlas77.svg?style=for-the-badge
[forks-url]: https://github.com/RedGear-Studio/Atlas77/network/members
[stars-shield]: https://img.shields.io/github/stars/RedGear-Studio/Atlas77.svg?style=for-the-badge
[stars-url]: https://github.com/RedGear-Studio/Atlas77/stargazers
[issues-shield]: https://img.shields.io/github/issues/RedGear-Studio/Atlas77.svg?style=for-the-badge
[issues-url]: https://github.com/RedGear-Studio/Atlas77/issues
[license-shield]: https://img.shields.io/github/license/RedGear-Studio/Atlas77.svg?style=for-the-badge
[license-url]: https://github.com/RedGear-Studio/Atlas77/blob/master/LICENSE.txt

