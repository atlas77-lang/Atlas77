use atlas_77::{build, run};
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
