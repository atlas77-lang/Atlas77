use std::{path::PathBuf, time::Instant};
#[allow(unused)]
pub mod atlas_frontend;
pub mod atlas_hir;
pub mod atlas_macro;
pub mod atlas_memory;
pub mod atlas_runtime;
pub mod atlas_stdlib;

use atlas_frontend::{parse, parser::error::ParseResult};
use atlas_runtime::{visitor::Visitor, Runtime};

use bumpalo::Bump;
use clap::{command, Parser};
use miette::Result;

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

fn main() -> ParseResult<()> {
    std::env::set_var("RUST_BACKTRACE", "1");
    match AtlasRuntimeCLI::parse() {
        AtlasRuntimeCLI::Run { file_path } => run(file_path),
        AtlasRuntimeCLI::Build { file_path } => build(file_path),
    }
}

pub(crate) fn build(path: String) -> ParseResult<()> {
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

    Ok(())
}

pub(crate) fn run(path: String) -> ParseResult<()> {
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
