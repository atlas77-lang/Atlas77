use atlas_77::{atlas_codegen::Codegen, atlas_frontend::parser::arena::AstArena};
#[allow(unused)]
use atlas_77::{
    atlas_codegen, atlas_frontend, atlas_hir, atlas_macro, atlas_memory, atlas_runtime,
    atlas_stdlib, atlas_vm,
};
use std::{path::PathBuf, time::Instant};

use atlas_frontend::parse;
use atlas_runtime::{visitor::Visitor, Runtime};

use bumpalo::Bump;
use clap::{command, Parser};

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

    println!("{:?}", &program);

    let bump = Bump::new();

    let arena = AstArena::new(&bump);

    let mut gen = Codegen::new(program, arena);

    let res = gen.compile(program);
    
    match res {
        Ok(o) => {
            println!("{:?}", o);
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
    //#[cfg(debug_assertions)]
    //println!("{:?}", &program);

    let mut runtime = Runtime::new();

    let start = Instant::now();
    let res = runtime.visit(&program, "main");

    match res {
        Ok(o) => {
            println!("{:?}", o);
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    }

    let end = Instant::now();
    println!("Elapsed time: {:?}", (end - start));
    Ok(())
}
