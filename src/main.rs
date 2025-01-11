use std::{path::PathBuf, time::Instant};

pub mod atlas_frontend;
pub mod atlas_memory;
pub mod atlas_runtime;
pub mod atlas_stdlib;

use atlas_frontend::parse;
use atlas_runtime::{visitor::Visitor, Runtime};

use clap::{command, Parser};

#[derive(Parser)] // requires `derive` feature
#[command(name = "Atlas 77")]
#[command(bin_name = "atlas_77", author = "Gipson62", version("v0.5-beta"), about = "Programming language made in Rust", long_about = None)]
enum AtlasRuntimeCLI {
    #[command(arg_required_else_help = true)]
    Run { file_path: String },
}

fn main() {
    let AtlasRuntimeCLI::Run { file_path } = AtlasRuntimeCLI::parse();

    run(file_path);
}

pub(crate) fn run(path: String) {
    let mut path_buf = PathBuf::from(path.clone());

    if let Ok(current_dir) = std::env::current_dir() {
        if !path_buf.is_absolute() {
            path_buf = current_dir.join(path_buf);
        }
    } else {
        eprintln!("Failed to get current directory");
    }

    let program = parse(path_buf.to_str().unwrap()).expect("Failed to open the file");
    #[cfg(debug_assertions)]
    println!("{:?}", &program);

    let mut runtime = Runtime::new();

    // file
    runtime.add_extern_fn("read_dir", atlas_stdlib::file::read_dir);
    runtime.add_extern_fn("read_file", atlas_stdlib::file::read_file);
    runtime.add_extern_fn("write_file", atlas_stdlib::file::write_file);
    runtime.add_extern_fn("file_exists", atlas_stdlib::file::file_exists);
    runtime.add_extern_fn("remove_file", atlas_stdlib::file::remove_file);

    // io
    runtime.add_extern_fn("print", atlas_stdlib::io::print);
    runtime.add_extern_fn("println", atlas_stdlib::io::println);
    runtime.add_extern_fn("input", atlas_stdlib::io::input);

    // list
    runtime.add_extern_fn("len", atlas_stdlib::list::len);
    runtime.add_extern_fn("get", atlas_stdlib::list::get);
    runtime.add_extern_fn("set", atlas_stdlib::list::set);
    runtime.add_extern_fn("push", atlas_stdlib::list::push);
    runtime.add_extern_fn("pop", atlas_stdlib::list::pop);
    runtime.add_extern_fn("remove", atlas_stdlib::list::remove);
    runtime.add_extern_fn("slice", atlas_stdlib::list::slice);

    // math
    runtime.add_extern_fn("abs", atlas_stdlib::math::abs);
    runtime.add_extern_fn("pow", atlas_stdlib::math::pow);
    runtime.add_extern_fn("sqrt", atlas_stdlib::math::sqrt);
    runtime.add_extern_fn("min", atlas_stdlib::math::min);
    runtime.add_extern_fn("max", atlas_stdlib::math::max);
    runtime.add_extern_fn("round", atlas_stdlib::math::round);
    runtime.add_extern_fn("random", atlas_stdlib::math::random);

    // string
    runtime.add_extern_fn("str_len", atlas_stdlib::string::str_len);
    runtime.add_extern_fn("trim", atlas_stdlib::string::trim);
    runtime.add_extern_fn("to_upper", atlas_stdlib::string::to_upper);
    runtime.add_extern_fn("to_lower", atlas_stdlib::string::to_lower);
    runtime.add_extern_fn("split", atlas_stdlib::string::split);

    // time
    runtime.add_extern_fn("now", atlas_stdlib::time::now);
    runtime.add_extern_fn("format_time_iso", atlas_stdlib::time::format_time_iso);
    runtime.add_extern_fn("format_time", atlas_stdlib::time::format_time);
    runtime.add_extern_fn("elapsed", atlas_stdlib::time::elapsed);

    let start = Instant::now();
    println!("{}", runtime.visit(&program));

    let end = Instant::now();
    println!("Elapsed time: {:?}", (end - start));
}
