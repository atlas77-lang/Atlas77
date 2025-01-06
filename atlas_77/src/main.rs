use std::{path::PathBuf, time::Instant};

use atlas_frontend::parse;
use atlas_hlir::translate;
use atlas_memory::vm_data::VMData;
use atlas_runtime::{visitor::Visitor, vm_state::VMState, Runtime};

use clap::{command, Parser};
use rand::prelude::*;

#[derive(Parser)] // requires `derive` feature
#[command(name = "Atlas 77")]
#[command(bin_name = "atlas_77", author = "Gipson62", version("v0.4-beta"), about = "Programming language made in Rust", long_about = None)]
enum AtlasRuntimeCLI {
    #[command(arg_required_else_help = true)]
    Run { file_path: String },
}

fn main() {
    let AtlasRuntimeCLI::Run { file_path } = AtlasRuntimeCLI::parse();

    run(file_path);
}

#[cfg(not(debug_assertions))]
fn print(state: VMState) -> Result<VMData, ()> {
    let val = state.stack.pop().unwrap();
    match val.tag {
        VMData::TAG_BOOL => {
            println!("{}", val.as_bool());
        }
        VMData::TAG_CHAR => {
            println!("{}", val.as_char());
        }
        VMData::TAG_FLOAT => {
            println!("{}", val.as_f64());
        }
        VMData::TAG_I64 => {
            println!("{}", val.as_i64());
        }
        VMData::TAG_U64 => {
            println!("{}", val.as_u64());
        }
        _ => {
            println!("{}", state.object_map.get(val.as_object()));
        }
    }
    Ok(VMData::new_unit())
}
#[cfg(debug_assertions)]
fn print(state: VMState) -> Result<VMData, ()> {
    let val = state.stack.pop().unwrap();
    match val.tag {
        VMData::TAG_BOOL
        | VMData::TAG_CHAR
        | VMData::TAG_FLOAT
        | VMData::TAG_I64
        | VMData::TAG_U64 => println!("{}", val),
        _ => {
            println!("Object: {}", state.object_map.get(val.as_object()));
        }
    }
    Ok(VMData::new_unit())
}
fn read_int(_state: VMState) -> Result<VMData, ()> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim().parse::<i64>().unwrap();
    Ok(VMData::new_i64(input))
}

fn random(state: VMState) -> Result<VMData, ()> {
    let range = (
        state.stack.pop().unwrap().as_i64(),
        state.stack.pop().unwrap().as_i64(),
    );
    let mut rng = thread_rng();
    let random = rng.gen_range(range.1..range.0);
    Ok(VMData::new_i64(random))
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
    runtime.add_extern_fn("print", print);
    runtime.add_extern_fn("read_int", read_int);
    runtime.add_extern_fn("random", random);

    let hlir = translate(&program);
    #[cfg(debug_assertions)]
    println!("{:?}", &hlir);

    let start = Instant::now();
    println!("{}", runtime.visit(&program));

    let end = Instant::now();
    println!("Elapsed time: {:?}", (end - start));
}
