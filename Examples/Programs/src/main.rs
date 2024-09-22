use vc_8bit::{assembly, c_lang, vc_8bit::Computer};
fn main() { 
    run_assembly_from_file();
    //run_compiled_code_with_debugging();
}
fn run_assembly(value: String) {
    let contents = assembly::compile_assembly_to_binary(&value);
    let bytes = assembly::string_to_bytes(contents.as_str());
    std::fs::write("src/test/test.o", contents).unwrap();
    let mut computer: Computer = Computer::new();
    computer.ram.insert_bytes(bytes);
    computer.run();
}
fn run_assembly_from_file() {
    let full_contents = std::fs::read_to_string("src/test/test.asm").unwrap();
    run_assembly(full_contents);
}
fn run_binary() {
    let full_contents = std::fs::read_to_string("src/test/test.bin").unwrap();
    let contents = full_contents.replace(" ", "").replace("\n", "s").replace("\r", "");
    let bytes = assembly::string_to_bytes(contents.as_str());
    let mut computer: Computer = Computer::new();
    computer.ram.insert_bytes(bytes);
    computer.run();
}
fn compile_and_run(value: String) {
    let asm = c_lang::compile(&value);
    run_assembly(asm);
}
fn compile_and_run_from_file() {
    let contents = std::fs::read_to_string("src/test/test.c").unwrap();
    let value = c_lang::compile(&contents);
    run_assembly(value);
}
fn run_compiled_code_with_debugging() {
    println!("- CLANG:");
    // constants are required for accessing array index and shifting operations
    // array[CONSTANT]
    // value << CONSTANT
    let contents = std::fs::read_to_string("src/test/test.c").unwrap();
    let lex = c_lang::get_lexer_lines(&contents);
    let par = c_lang::parse(lex);
    for l in par.clone() { println!("{}", c_lang::fmt_expr(&l.unwrap())) }
    
    println!("\n- ASM:"); 
    let value = c_lang::interpret(par.clone());
    println!("{value}");

    println!("\n- OUT:");
    println!("{}", assembly::compile_assembly_to_binary(&value));

    println!("\n- RUN:");
    run_assembly(value);
}