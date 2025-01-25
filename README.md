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

Atlas77 is an experimental statically typed programming language with a strong interop with Rust.
It will run on a custom VM. I'll implement a JIT compiler using cranelift later on.
(There will be an AOT compiler too)


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

> [!Note]
> I recommend you to build it yourself, as the version on Cargo is not always up to date.
> There are also some bugs in the current version on Cargo.

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
import "std/io"

func
fib(n
:
i64
) ->
i64
{
    if n <= 1 {
        return n;
    }
    return fib(n - 1) + fib(n - 2);
}

func
main()
->
i64
{
    let n: i64 = 10;
    print_int(fib(n));
}
```

_For more examples, please refer to the [examples folder](./examples/README.MD)_

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- ROADMAP -->

## Roadmap

### v0.3 "Foundation"

> Deprecated, if you wanna know more about the v0.3.x, check the releases page.

- [v0.3](https://github.com/atlas77-lang/Atlas77/releases/tag/v0.3)
- [v0.3.1](https://github.com/atlas77-lang/Atlas77/releases/tag/v0.3.1)

### v0.4 "Keystone"

> Deprecated, if you wanna know more about the v0.4, check the releases page.

- [v0.4 "Keystone"](https://github.com/atlas77-lang/Atlas77/tag/v0.4)

### v0.5 Phoenix Release Timeline

The v0.5 is a complete rewrite of Atlas77, it aligns better with the end goal of the language (see [_Goal of the
language_](#goal-of-the-language)).
> NB: up until the v1.0 the language will always be in alpha, tho I'll try to make every release as stable as possible.
> I hope I can release the v0.5 this month

| Feature                  | Expected Version | Description                                                             | Status |
|--------------------------|------------------|-------------------------------------------------------------------------|--------|
| Functions                | **v0.5**         | Define and call functions                                               | ✅      |
| Variables                | **v0.5**         | Immutable (`const`) and mutable (`let`) variables                       | ✅      |
| Basic `std` Library      | **v0.5**         | Core utilities for `time`, `file`, `io`, `math`, `string`, `list`       | ✅      |
| Import                   | **v0.5**         | Limited to standard library imports for now                             | ✅      |
| Control Flow             | **v0.5**         | `if/else` statements for conditional logic, `while` loops for iteration | ✅      |
| Match Expressions        | **v0.5.1**       | Pattern matching                                                        | 💭     |
| Structs                  | **v0.5.1**       | User-defined types with named fields                                    | 🔧     |
| Unions                   | **v0.5.1**       | Low-level data structures allowing overlapping memory layouts           | 💤     |
| Enums                    | **v0.5.1**       | Enumerations with optional associated data for flexible value sets      | 🔧     |
| Lambdas & Closures       | **v0.5.2**       | Inline, anonymous functions with captured variables                     | 🔧     |
| Classes                  | **v0.5.2**       | Object-oriented programming support                                     | 🔧     |
| Traits                   | **v0.5.2**       | Interfaces for defining shared behavior                                 | 🔧     |
| Pointers                 | **v0.5.2**       | Basic pointer manipulation for low-level programming                    | 🔧     |
| Memory Management        | **v0.5.2**       | Simple memory management                                                | 🔧     |
| Imports                  | **v0.5.2**       | Importing code from other files                                         | 🔧     |
| Generics                 | **v0.5.x**       | Type parameters for writing reusable code                               | 🔧     |
| Standard Library         | **v0.5.x**       | A comprehensive standard library                                        | 💭     |
| Package Manager          | **unknown**      | A package manager for sharing code                                      | 💤     |
| Language Server Protocol | **unknown**      | Editor support for code completion, diagnostics, and more               | 💤     |
| Cranelift JIT            | **unknown**      | Just-in-time compilation for faster execution                           | 💤     |

#### Legend

- 💤: Not implemented
- 💭: Being thought of
- 🔧: Being implemented
- ✅: Working

#### Stability and Refinement

> As the language is still in alpha (not 1.0 yet), I won't make "alpha"/"beta" build, it doesn't really make sense.

The beta phase (aka after 0.5.2 and beyond) will focus on stabilizing the language. All features will be finalized,
tested extensively, and optimized for real-world use. This phase will serve as a release candidate.

See the [open issues](https://github.com/atlas77-lang/Atlas77/issues) for a full list of proposed features (and known
issues).

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

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any
contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also
simply open an issue with the tag "enhancement".
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
