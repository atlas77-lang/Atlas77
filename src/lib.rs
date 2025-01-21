use atlas_codegen::{arena::CodeGenArena, CodeGenUnit};
use atlas_frontend::parse;
use atlas_frontend::parser::arena::AstArena;
use atlas_hir::type_check_pass::TypeChecker;
use atlas_hir::{arena::HirArena, syntax_lowering_pass::AstSyntaxLoweringPass};
use bumpalo::Bump;

use std::{io::Write, path::PathBuf, time::Instant};

//todo: The pipeline of the compiler should be more straightforward and should include the "debug" and "release" modes
pub fn build(path: String) -> miette::Result<()> {
    let mut path_buf = PathBuf::from(path.clone());
    if let Ok(current_dir) = std::env::current_dir() {
        if !path_buf.is_absolute() {
            path_buf = current_dir.join(path_buf);
        }
    } else {
        eprintln!("Failed to get current directory");
    }

    let source = std::fs::read_to_string(path).unwrap();
    //parse
    let bump = Bump::new();
    let ast_arena = AstArena::new(&bump);
    let program = parse(path_buf.to_str().unwrap(), &ast_arena, source.clone())?;

    //hir
    let hir_arena = HirArena::new();
    let lower = AstSyntaxLoweringPass::new(&hir_arena, &program, &ast_arena, source.clone());
    let mut hir = lower.lower()?;

    //type-check
    let mut type_checker = TypeChecker::new(&hir_arena, source.clone());
    type_checker.check(&mut hir)?;

    //codegen
    let bump = Bump::new();
    let arena = CodeGenArena::new(&bump);
    let mut codegen = CodeGenUnit::new(hir, arena, source);
    let program = codegen.compile()?;
    let output = ron::ser::to_string_pretty(&program, Default::default()).unwrap();
    let mut file = std::fs::File::create("output.atlasc").unwrap();
    file.write_all(output.as_bytes()).unwrap();

    //run
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

//The "run" function needs a bit of refactoring
pub fn run(path: String) -> miette::Result<()> {
    let mut path_buf = PathBuf::from(path.clone());
    if let Ok(current_dir) = std::env::current_dir() {
        if !path_buf.is_absolute() {
            path_buf = current_dir.join(path_buf);
        }
    } else {
        eprintln!("Failed to get current directory");
    }

    let source = std::fs::read_to_string(path).unwrap();
    //parse
    let bump = Bump::new();
    let ast_arena = AstArena::new(&bump);
    let program = parse(path_buf.to_str().unwrap(), &ast_arena, source.clone())?;

    //hir
    let hir_arena = HirArena::new();
    let lower = AstSyntaxLoweringPass::new(&hir_arena, &program, &ast_arena, source.clone());
    let mut hir = lower.lower()?;

    //type-check
    let mut type_checker = TypeChecker::new(&hir_arena, source.clone());
    type_checker.check(&mut hir)?;

    //codegen
    let bump = Bump::new();
    let arena = CodeGenArena::new(&bump);
    let mut codegen = CodeGenUnit::new(hir, arena, source);
    let program = codegen.compile()?;
    let output = ron::ser::to_string_pretty(&program, Default::default()).unwrap();
    let mut file = std::fs::File::create("output.atlasc").unwrap();
    file.write_all(output.as_bytes()).unwrap();

    //run
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
