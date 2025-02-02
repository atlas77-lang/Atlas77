# Changelog

All notable changes to this project will be documented in this file.

## [0.5.2] - 2025-02-02

### Bug Fixes

- Issue with returning pointer ([13731a1](https://github.com/atlas77-lang/Atlas77/commit/13731a1c9b93fbd314ac3d06345a644905a71408))
- Issue with unary op ([4a2f30b](https://github.com/atlas77-lang/Atlas77/commit/4a2f30b83ce254244fe85944bbf6c46a4479ee51))
- Issue #104 ([f27248e](https://github.com/atlas77-lang/Atlas77/commit/f27248e4ca877997c2f29145ecd524e4e594e5fd))

### Documentation

- Tried to already prepare string & vector library with classes ([ae6c4ef](https://github.com/atlas77-lang/Atlas77/commit/ae6c4ef1701578a13bfccd0ff84f95fddcf4a426))
- Update CHANGELOG.md ([d3b407e](https://github.com/atlas77-lang/Atlas77/commit/d3b407e40c41f7051b66491027e89e0bd68d553f))
- Removed the doc and put it in atlas77-docs ([2e79547](https://github.com/atlas77-lang/Atlas77/commit/2e7954737800eb6c714343670d93e595b47e048e))
- Added some doc and updated it ([b65dcf5](https://github.com/atlas77-lang/Atlas77/commit/b65dcf53bef943eff4cd212521641c6a963bbfb3))
- Mdbook build ([adbd5a6](https://github.com/atlas77-lang/Atlas77/commit/adbd5a67ade26f4796c5ee46a4788bea1672c979))
- More test ([e66c72a](https://github.com/atlas77-lang/Atlas77/commit/e66c72ad60accb03a4b31c6a36c805005d3c8fe4))
- Update to the docs ([3ac1248](https://github.com/atlas77-lang/Atlas77/commit/3ac12482e283eb77894b2c1d5989d163207ed8a3))
- Added some docs for the standard library ([8a2be67](https://github.com/atlas77-lang/Atlas77/commit/8a2be67d73e5edf63ccce99aecea55081df7cbc1))
- Basic setup for documentation of this project ([929f6a9](https://github.com/atlas77-lang/Atlas77/commit/929f6a94e15a424cb6f21f5bdb2b7b4a3661b90d))

### Features

- Added working classes ([1bd098b](https://github.com/atlas77-lang/Atlas77/commit/1bd098ba8c29337bb60da19e1587d775e74960c3))
- Classes are fully parsed, lowered and type checked! ([dcbefc4](https://github.com/atlas77-lang/Atlas77/commit/dcbefc4f764bb71a5910bf9ceae1b35c9a41dc69))
- We can parse classes now ([fa6a68b](https://github.com/atlas77-lang/Atlas77/commit/fa6a68b25ff2db779114c3699091999d19500dbe))
- Warnings have been added for wrong cases ([9bffcc3](https://github.com/atlas77-lang/Atlas77/commit/9bffcc3b87b025b4c5f817f39e574553637bf73c))
- Basic generics for external function ([1a9e510](https://github.com/atlas77-lang/Atlas77/commit/1a9e5107ecd40728468f677a6be3cc5792b5214d))
- Improved Runtime by optimizing the VarMap ([1223c83](https://github.com/atlas77-lang/Atlas77/commit/1223c838b0caa13dbafdea8d6d9fca74f67dbdfb))
- Made a small matmul in test.atlas ([4867b57](https://github.com/atlas77-lang/Atlas77/commit/4867b57507e8ba5012405fefaa7649c3da68b8c4))
- VMData.tag is now u8 from u16 ([efd12ae](https://github.com/atlas77-lang/Atlas77/commit/efd12ae966e61e1822571e2bd99ee6134fae892d))
- Added PushBool instruction ([ef65471](https://github.com/atlas77-lang/Atlas77/commit/ef65471e1c43155fb3e284a60a3b11c3a2be2d6c))
- Added a working Reference Counting memory management ([1b8ae06](https://github.com/atlas77-lang/Atlas77/commit/1b8ae06a67b9ffea1e6cd46c4464093948b998ee))
- Lists work. [int64] or [float64] should work ([82ef451](https://github.com/atlas77-lang/Atlas77/commit/82ef451ec87ae1cc332d5d5490eb9e43bc87327b))
- Casting is here with the `as` keyword! ([aca37c9](https://github.com/atlas77-lang/Atlas77/commit/aca37c90ea8c0a18e37be756a288cd7983e25968))
- Added strings ([c39ff5a](https://github.com/atlas77-lang/Atlas77/commit/c39ff5a3e95f5a3dbb7a8a3312f55f1d0df64749))
- Added unary operation in the codegen 💀☠️ ([64b14af](https://github.com/atlas77-lang/Atlas77/commit/64b14af56800e10e39c84cbefcd318bfa45042ec))
- Parser for classes and Static access (i.e. ::) ([f137438](https://github.com/atlas77-lang/Atlas77/commit/f137438c65f0e9fdf501d9a0b1fbfddb6e1579f7))
- Type Inference is working ([dfbb536](https://github.com/atlas77-lang/Atlas77/commit/dfbb536f004635e10b13bb99bf14ae9207aa28fd))

### Miscellaneous Tasks

- Cargo clippy ([bd51868](https://github.com/atlas77-lang/Atlas77/commit/bd5186852c8769c55d26e42f8f5d8dddd874239b))
- Update rand requirement from 0.8.5 to 0.9.0 ([4327b05](https://github.com/atlas77-lang/Atlas77/commit/4327b050002fc20cc86ef167ff80f8597eb66c65))
- Prepare for v0.5.1 (again-again-again) ([08989e9](https://github.com/atlas77-lang/Atlas77/commit/08989e96fe39aeb436ec8f367a070cf7234790e4))
- Prepare for v0.5.1 (again-again) ([8655028](https://github.com/atlas77-lang/Atlas77/commit/8655028cfd64957eb535c1dbfea67aff7818ae1a))
- Prepare for v0.5.1 (again) ([39c3879](https://github.com/atlas77-lang/Atlas77/commit/39c3879b162e2a85a89c7399b7954a01c86beeff))
- Prepare for v0.5.1 ([32221d4](https://github.com/atlas77-lang/Atlas77/commit/32221d4d556787915e0096d3e6e1cbb78d7d558b))
- Rand 0.8.5 -> 0.9.0 ([26e0603](https://github.com/atlas77-lang/Atlas77/commit/26e06038b77abb3478eb60d453c3aba7fb84be05))
- Cleaning a bit ([3ffa049](https://github.com/atlas77-lang/Atlas77/commit/3ffa0496c22c24ac2844c19c712354e6cb137d97))
- Updated Cargo.toml files version ([bd743fb](https://github.com/atlas77-lang/Atlas77/commit/bd743fb287ba7890b93e7fd1a20a0be039569855))
- Added a bit of syntax highlighting for VSCode ([29a46f6](https://github.com/atlas77-lang/Atlas77/commit/29a46f6c4bf84e0fa09d0b502803a1f614b28ca6))
- Redid the file structure so it's more easier to navigate ([356b785](https://github.com/atlas77-lang/Atlas77/commit/356b7857ea564ea02d0504e75d4dc317d8ab185e))

### Refactor

- Redid the file structure once again for `cargo publish` ([56de771](https://github.com/atlas77-lang/Atlas77/commit/56de77191a7172714eaac554158a08b3d73810cc))
- Swapped the lexer from atlas-core to logos ([825fdbe](https://github.com/atlas77-lang/Atlas77/commit/825fdbe06f7d4a557ffd6b65a1ca1ee5f0f58d6b))
- Atlas-core -> logos for a more efficient lexer ([e4bc5d7](https://github.com/atlas77-lang/Atlas77/commit/e4bc5d7f543b7dc502b7a25ab6059a58932ea20d))
- Change type names `i64` -> `int64` ([090fa4f](https://github.com/atlas77-lang/Atlas77/commit/090fa4fe1119b3473f1132f4a9d6dcf1e2fc69fd))
- Changed file structure for the better ([4c40770](https://github.com/atlas77-lang/Atlas77/commit/4c407708930a9a8ce53994d64a2cb92215095aa4))

### Misc

- Git asked me to commit before pushing again ([bf7cbf8](https://github.com/atlas77-lang/Atlas77/commit/bf7cbf8a7fcdc871dfdf20f8851df32af09c10cc))
- Removed debug types in error messages ([b46aa14](https://github.com/atlas77-lang/Atlas77/commit/b46aa143efbb29ef365b9aad7c41abcd7685f657))
- Added some stuff, nothing fancy, mostly comments ([04f0324](https://github.com/atlas77-lang/Atlas77/commit/04f03247a5987469b3f9eee0c5fead172fa8b136))
- Stuff done, no idea what ([58c6aa2](https://github.com/atlas77-lang/Atlas77/commit/58c6aa20b405bb4c3421198c265687e4ec1aee06))

## [0.5.1] - 2025-01-29

### Bug Fixes

- Issue with returning pointer ([13731a1](https://github.com/atlas77-lang/Atlas77/commit/13731a1c9b93fbd314ac3d06345a644905a71408))
- Issue with unary op ([4a2f30b](https://github.com/atlas77-lang/Atlas77/commit/4a2f30b83ce254244fe85944bbf6c46a4479ee51))
- Issue #104 ([f27248e](https://github.com/atlas77-lang/Atlas77/commit/f27248e4ca877997c2f29145ecd524e4e594e5fd))

### Documentation

- Removed the doc and put it in atlas77-docs ([2e79547](https://github.com/atlas77-lang/Atlas77/commit/2e7954737800eb6c714343670d93e595b47e048e))
- Added some doc and updated it ([b65dcf5](https://github.com/atlas77-lang/Atlas77/commit/b65dcf53bef943eff4cd212521641c6a963bbfb3))
- Mdbook build ([adbd5a6](https://github.com/atlas77-lang/Atlas77/commit/adbd5a67ade26f4796c5ee46a4788bea1672c979))
- More test ([e66c72a](https://github.com/atlas77-lang/Atlas77/commit/e66c72ad60accb03a4b31c6a36c805005d3c8fe4))
- Update to the docs ([3ac1248](https://github.com/atlas77-lang/Atlas77/commit/3ac12482e283eb77894b2c1d5989d163207ed8a3))
- Added some docs for the standard library ([8a2be67](https://github.com/atlas77-lang/Atlas77/commit/8a2be67d73e5edf63ccce99aecea55081df7cbc1))
- Basic setup for documentation of this project ([929f6a9](https://github.com/atlas77-lang/Atlas77/commit/929f6a94e15a424cb6f21f5bdb2b7b4a3661b90d))

### Features

- Improved Runtime by optimizing the VarMap ([1223c83](https://github.com/atlas77-lang/Atlas77/commit/1223c838b0caa13dbafdea8d6d9fca74f67dbdfb))
- Made a small matmul in test.atlas ([4867b57](https://github.com/atlas77-lang/Atlas77/commit/4867b57507e8ba5012405fefaa7649c3da68b8c4))
- VMData.tag is now u8 from u16 ([efd12ae](https://github.com/atlas77-lang/Atlas77/commit/efd12ae966e61e1822571e2bd99ee6134fae892d))
- Added PushBool instruction ([ef65471](https://github.com/atlas77-lang/Atlas77/commit/ef65471e1c43155fb3e284a60a3b11c3a2be2d6c))
- Added a working Reference Counting memory management ([1b8ae06](https://github.com/atlas77-lang/Atlas77/commit/1b8ae06a67b9ffea1e6cd46c4464093948b998ee))
- Lists work. [int64] or [float64] should work ([82ef451](https://github.com/atlas77-lang/Atlas77/commit/82ef451ec87ae1cc332d5d5490eb9e43bc87327b))
- Casting is here with the `as` keyword! ([aca37c9](https://github.com/atlas77-lang/Atlas77/commit/aca37c90ea8c0a18e37be756a288cd7983e25968))
- Added strings ([c39ff5a](https://github.com/atlas77-lang/Atlas77/commit/c39ff5a3e95f5a3dbb7a8a3312f55f1d0df64749))
- Added unary operation in the codegen 💀☠️ ([64b14af](https://github.com/atlas77-lang/Atlas77/commit/64b14af56800e10e39c84cbefcd318bfa45042ec))
- Parser for classes and Static access (i.e. ::) ([f137438](https://github.com/atlas77-lang/Atlas77/commit/f137438c65f0e9fdf501d9a0b1fbfddb6e1579f7))
- Type Inference is working ([dfbb536](https://github.com/atlas77-lang/Atlas77/commit/dfbb536f004635e10b13bb99bf14ae9207aa28fd))

### Miscellaneous Tasks

- Prepare for v0.5.1 (again-again-again) ([08989e9](https://github.com/atlas77-lang/Atlas77/commit/08989e96fe39aeb436ec8f367a070cf7234790e4))
- Prepare for v0.5.1 (again-again) ([8655028](https://github.com/atlas77-lang/Atlas77/commit/8655028cfd64957eb535c1dbfea67aff7818ae1a))
- Prepare for v0.5.1 (again) ([39c3879](https://github.com/atlas77-lang/Atlas77/commit/39c3879b162e2a85a89c7399b7954a01c86beeff))
- Prepare for v0.5.1 ([32221d4](https://github.com/atlas77-lang/Atlas77/commit/32221d4d556787915e0096d3e6e1cbb78d7d558b))
- Rand 0.8.5 -> 0.9.0 ([26e0603](https://github.com/atlas77-lang/Atlas77/commit/26e06038b77abb3478eb60d453c3aba7fb84be05))
- Cleaning a bit ([3ffa049](https://github.com/atlas77-lang/Atlas77/commit/3ffa0496c22c24ac2844c19c712354e6cb137d97))
- Updated Cargo.toml files version ([bd743fb](https://github.com/atlas77-lang/Atlas77/commit/bd743fb287ba7890b93e7fd1a20a0be039569855))
- Added a bit of syntax highlighting for VSCode ([29a46f6](https://github.com/atlas77-lang/Atlas77/commit/29a46f6c4bf84e0fa09d0b502803a1f614b28ca6))
- Redid the file structure so it's more easier to navigate ([356b785](https://github.com/atlas77-lang/Atlas77/commit/356b7857ea564ea02d0504e75d4dc317d8ab185e))

### Refactor

- Redid the file structure once again for `cargo publish` ([56de771](https://github.com/atlas77-lang/Atlas77/commit/56de77191a7172714eaac554158a08b3d73810cc))
- Swapped the lexer from atlas-core to logos ([825fdbe](https://github.com/atlas77-lang/Atlas77/commit/825fdbe06f7d4a557ffd6b65a1ca1ee5f0f58d6b))
- Atlas-core -> logos for a more efficient lexer ([e4bc5d7](https://github.com/atlas77-lang/Atlas77/commit/e4bc5d7f543b7dc502b7a25ab6059a58932ea20d))
- Change type names `i64` -> `int64` ([090fa4f](https://github.com/atlas77-lang/Atlas77/commit/090fa4fe1119b3473f1132f4a9d6dcf1e2fc69fd))
- Changed file structure for the better ([4c40770](https://github.com/atlas77-lang/Atlas77/commit/4c407708930a9a8ce53994d64a2cb92215095aa4))

### Misc

- Removed debug types in error messages ([b46aa14](https://github.com/atlas77-lang/Atlas77/commit/b46aa143efbb29ef365b9aad7c41abcd7685f657))
- Added some stuff, nothing fancy, mostly comments ([04f0324](https://github.com/atlas77-lang/Atlas77/commit/04f03247a5987469b3f9eee0c5fead172fa8b136))
- Stuff done, no idea what ([58c6aa2](https://github.com/atlas77-lang/Atlas77/commit/58c6aa20b405bb4c3421198c265687e4ec1aee06))

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

