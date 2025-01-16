#[allow(unused)]
pub mod atlas_codegen;
pub mod atlas_frontend;
pub mod atlas_hir;
pub mod atlas_macro;
pub mod atlas_memory;
pub mod atlas_runtime;
pub mod atlas_stdlib;
pub mod atlas_vm;
use crate::{
    atlas_codegen::{arena::CodeGenArena, CodeGenUnit},
    atlas_frontend::parse,
    atlas_hir::{arena::HirArena, syntax_lowering_pass::AstSyntaxLoweringPass},
};
use bumpalo::Bump;
use clap::{command, Parser};
use std::{io::Write, path::PathBuf, time::Instant};

#[derive(Parser)] // requires `derive` feature
#[command(name = "Atlas77")]
#[command(
    bin_name = "atlas_77",
    author = "Gipson62",
    version("v0.5-beta Phoenix"),
    about = "Programming language made in Rust",
    long_about = "Atlas77 is a programming language made in Rust. It is a statically typed language with a focus on [To be defined]."
)]
enum AtlasRuntimeCLI {
    #[command(
        arg_required_else_help = true,
        about = "Compile then run a local package",
        long_about = "Compile then run a local package. The output will be written to the current directory."
    )]
    Run { file_path: String },
    #[command(
        arg_required_else_help = true,
        about = "Compile a local package and all of its dependencies",
        long_about = "Compile a local package and all of its dependencies. The output will be written to the current directory."
    )]
    Build { file_path: String },
}

fn main() -> miette::Result<()> {
    //std::env::set_var("RUST_BACKTRACE", "1");
    match AtlasRuntimeCLI::parse() {
        AtlasRuntimeCLI::Run { file_path } => run(file_path),
        AtlasRuntimeCLI::Build { file_path } => build(file_path),
    }
}

pub(crate) fn build(path: String) -> miette::Result<()> {
    let mut path_buf = PathBuf::from(path.clone());

    if let Ok(current_dir) = std::env::current_dir() {
        if !path_buf.is_absolute() {
            path_buf = current_dir.join(path_buf);
        }
    } else {
        eprintln!("Failed to get current directory");
    }

    let bump = Bump::new();

    let program = parse(path_buf.to_str().unwrap(), &bump)?;

    let hir_arena = HirArena::new();

    let lower = AstSyntaxLoweringPass::new(&hir_arena, &program);
    let hir = lower.lower()?;
    let bump = Bump::new();

    let arena = CodeGenArena::new(&bump);

    let mut codegen = CodeGenUnit::new(hir, arena);

    let program = codegen.compile()?;

    let output = ron::ser::to_string_pretty(&program, Default::default()).unwrap();
    let mut file = std::fs::File::create("output.atlasc").unwrap();
    file.write_all(output.as_bytes()).unwrap();

    let start = Instant::now();
    let mut vm = atlas_vm::Atlas77VM::new(program);
    let res = vm.run();
    let end = Instant::now();
    match res {
        Ok(_) => {
            println!(
                "Program ran successfully: {} (time: {}ms)",
                vm.stack.pop().unwrap(),
                (end - start).as_millis()
            );
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    }
    Ok(())
}

pub(crate) fn run(path: String) -> miette::Result<()> {
    let mut path_buf = PathBuf::from(path.clone());

    if let Ok(current_dir) = std::env::current_dir() {
        if !path_buf.is_absolute() {
            path_buf = current_dir.join(path_buf);
        }
    } else {
        eprintln!("Failed to get current directory");
    }

    let bump = Bump::new();

    let program = parse(path_buf.to_str().unwrap(), &bump)?;

    #[cfg(debug_assertions)]
    println!("{:?}", &program);

    let start = Instant::now();

    let end = Instant::now();
    println!("Elapsed time: {:?}", (end - start));
    Ok(())
}
