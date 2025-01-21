# Configuration

## Prerequisites

* Rust Compiler
  ```sh
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

> NB: This is only for Unix-based systems *(or WSL for Windows)*. For Windows, you can use [Scoop](https://scoop.sh/)
> ```shell
> scoop install main/rust
> ```

Or directly from their website: [Rust](https://www.rust-lang.org/tools/install)

## Installation

Once Rust is installed, you can install Atlas77 using Cargo.

1. Install it from Cargo
    ```sh
    cargo install atlas_77
    ```
2. Use it as a CLI
    ```sh
    atlas_77 --help
    ```
3. Enjoy!

Or you can clone the repository and build it yourself.

1. Clone the repository
    ```sh
    git clone https://github.com/atlas77-lang/atlas77.git && cd atlas77
    ```
2. Build it
    ```sh
    cargo build --release
    ```
3. Use it as a CLI
    ```sh
    ./target/release/atlas_77 --help
    ```