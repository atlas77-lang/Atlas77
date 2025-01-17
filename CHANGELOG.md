# Changelog

All notable changes to this project will be documented in this file.

## [0.5] - 2025-01-17

### Bug Fixes

- Issue with halt shifting the bytecode by 1 ([2932b8c](https://github.com/atlas77-lang/Atlas77/commit/2932b8cfe1f0989e9abebe21acbe09ac2a12df9e))
- Fixed an issue with typechecking in if condition ([229adea](https://github.com/atlas77-lang/Atlas77/commit/229adeac86c3832c7714242808655b3c8187f96e))
- Args in functions were empty resulting in a null error ([ba02e22](https://github.com/atlas77-lang/Atlas77/commit/ba02e22341d6e4140d91a91e7abd3791cdd832a3))
- Return type is optional now ([8d76ee0](https://github.com/atlas77-lang/Atlas77/commit/8d76ee0aabbe1c47fc5cda0e74f742b5f82289a3))

### Documentation

- Redid the Roadmap section of the README.md ([c051231](https://github.com/atlas77-lang/Atlas77/commit/c051231a544e35edf69dc0daefa160a791f92e60))
- Start of SYNTAX.md & redefined the roadmap ([4e606c1](https://github.com/atlas77-lang/Atlas77/commit/4e606c10a2e9139b0303067688e0399ae3e87afe))
- Added git cliff ([2bb7ae5](https://github.com/atlas77-lang/Atlas77/commit/2bb7ae552505170832ccbb24b1415f44eab355e7))

### Features

- Import `std/io` works, other will follow ([c79515c](https://github.com/atlas77-lang/Atlas77/commit/c79515c11949fc3a756ebb7dd1cca9c047bc2df2))
- Should be feature complete for v0.5 ([300d112](https://github.com/atlas77-lang/Atlas77/commit/300d1129fbe96492a8ed3f20f1018ec36b080acc))
- Type Checker seems to be working farily well ([8fac671](https://github.com/atlas77-lang/Atlas77/commit/8fac671ed3831ebcb75ba2f4bd3874982f3d670a))
- Print & println are working ([92d72b5](https://github.com/atlas77-lang/Atlas77/commit/92d72b53567c7e6de84ed5eac137e9551175d423))
- While, if/else, let & assignment working ([5ab6d0a](https://github.com/atlas77-lang/Atlas77/commit/5ab6d0a24c16ce121c0ba65bb9f5df66179e990c))
- Codegen working for square.atlas ([8cf81ca](https://github.com/atlas77-lang/Atlas77/commit/8cf81ca25a004b88a9459edff90d65daca160d9a))
- The lowering pass is working ([dc4db1e](https://github.com/atlas77-lang/Atlas77/commit/dc4db1e4e6e1fb681628535b3fa005041963b913))
- Square.atlas runs ([2113acf](https://github.com/atlas77-lang/Atlas77/commit/2113acffc5318f188ae19d321da2e6dd76e6db11))
- Smol start of codegen + vm ([0198aba](https://github.com/atlas77-lang/Atlas77/commit/0198abaf8b1a709db1a56af4097f7227482751e2))
- Hello.atlas now work ([fc8d2f1](https://github.com/atlas77-lang/Atlas77/commit/fc8d2f18b856080db9d24b18407fc4f9a9ddbe77))
- Parser can now parse hello.atlas ([26e803a](https://github.com/atlas77-lang/Atlas77/commit/26e803a088abea86d9bf3078b8b52395f3006eb2))
- Let & binary op works ([647686b](https://github.com/atlas77-lang/Atlas77/commit/647686b1bf077193abe0125a1b836aaa11f1ed64))
- Upload built binaries to release pages ([a0a6c1a](https://github.com/atlas77-lang/Atlas77/commit/a0a6c1afa0f850b2ece63ced244fee510218e021))
- Implement new AST structure for program representation ([55713de](https://github.com/atlas77-lang/Atlas77/commit/55713de10e0a2a21185739570416c62b456461c2))

### Miscellaneous Tasks

- Cargo clippy ([9af5f25](https://github.com/atlas77-lang/Atlas77/commit/9af5f258eeafe2a38e0c0d5b9f4c9e35266b67dc))
- Doing what clippy wants ([0dbce18](https://github.com/atlas77-lang/Atlas77/commit/0dbce182cb37dec1d818aefb275a49070e9d4813))

### Refactor

- The language as a whole is done for the v0.5 ([ed16dd1](https://github.com/atlas77-lang/Atlas77/commit/ed16dd1fab49a314a21171e88863d5d2ca890643))
- The whole pipeline down to the VM works. ([508414e](https://github.com/atlas77-lang/Atlas77/commit/508414ec66d610c570e14b54483c63a5f94d9c7c))
- Everything is broken rn. I'm refactoring everything ([56adc11](https://github.com/atlas77-lang/Atlas77/commit/56adc11633cd5c75c192bce94262bdb811323f99))

### Misc

- Update ./examples ([fc5053d](https://github.com/atlas77-lang/Atlas77/commit/fc5053d37ff7a097aed4cf9426684711691620d4))
- Clarified the README.md ([390cca4](https://github.com/atlas77-lang/Atlas77/commit/390cca4e2ed8896a6f668497289f2a019defdd62))

# Changelog

All notable changes to this project will be documented in this file.

## [unreleased]

### Features

- Implement new AST structure for program representation ([55713de](https://github.com/atlas77-lang/Atlas77/commit/55713de10e0a2a21185739570416c62b456461c2))

