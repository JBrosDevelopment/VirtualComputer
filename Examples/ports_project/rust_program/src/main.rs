use vc_8bit::{assembly, c_lang, vc_8bit::Computer};

fn main() {
    // compile code
    let program = std::fs::read_to_string("src/program.c").unwrap();
    let compiled_program = c_lang::compile(&program);
    //println!("Compiled program:\n{}", compiled_program);

    let fixed = c_lang::constants_and_bytes(&program);
    let lex = c_lang::get_lexer_lines(&fixed);
    let par = c_lang::parse(lex);
    for l in par.clone() { println!("{}", c_lang::fmt_expr(&l.unwrap())) }

    // assemble code
    let contents = assembly::compile_assembly_to_binary(&compiled_program);
    let bytes = assembly::string_to_bytes(contents.as_str());
    
    // run on VC
    let mut computer: Computer = Computer::new();
    computer.ram.insert_bytes(bytes);
    computer.run();
}
