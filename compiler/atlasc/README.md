# Atlas77 Compiler

This is the compiler for the atlas77 language. Here's a list of all the modules in the compiler:

- `atlas_asm`: The assembler for the atlas77 language it takes in a list of Instructions and outputs a list of bytes.
- `atlas_codegen`: The code generator for the atlas77 language. It takes in the Hir and outputs a list of instructions.
- `atlas_frontend`: The frontend for the atlas77 language. It takes in a string and outputs the AST.
- `atlas_hir`: The high-level intermediate representation for the atlas77 language. It takes in the AST and outputs the
  HIR.
  It also performs type checking, dead code elimination (WIP), and constant folding (WIP).
- `atlas_macro`: Utilities macros for the atlas77 language.
- `atlas_runtime`: Compiler runtime for the atlas77 language. Will run `comptime` expressions in the future.
