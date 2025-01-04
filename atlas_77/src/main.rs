use std::{path::PathBuf, time::Instant};

use atlas_frontend::parse;
use atlas_memory::vm_data::VMData;
use atlas_runtime::{visitor::Visitor, vm_state::VMState, Runtime};

use clap::Parser as ClapParser;
use rand::prelude::*;

#[derive(ClapParser, Debug)]
#[clap(author = "Gipson62", version, about = "Programming language made in Rust", long_about = None)]
struct Args {
    /// Run you program at the given <PATH>
    #[arg(short, long, value_name = "PATH")]
    run: String,
}

fn main() {
    let args = Args::parse();

    run(args.run);

}

fn print(state: VMState) -> Result<VMData, ()> {
    let val = state.stack.last().unwrap();
    match val.tag {
        VMData::TAG_BOOL
        | VMData::TAG_CHAR
        | VMData::TAG_FLOAT
        | VMData::TAG_I64
        | VMData::TAG_U64 => {
            println!("{}", val);
        }
        _ => {
            println!("{}", state.object_map.get(val.as_object()));
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
        println!("Failed to get current directory");
    }

    let start = Instant::now();

    let program = parse(path_buf.to_str().unwrap()).expect("Failed to open the file");

    let mut runtime = Runtime::new();
    runtime.add_extern_fn("print", print);
    runtime.add_extern_fn("read_int", read_int);
    runtime.add_extern_fn("random", random);

    println!("{:?}", runtime.visit(&program));

    let end = Instant::now();
    println!("Elapsed time: {:?}", (end - start));
}
