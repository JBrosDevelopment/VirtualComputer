use vc_8bit::{assembly, c_lang, vc_8bit::Computer};

fn main() {
    // compile code
    let program = std::fs::read_to_string("src/program2.c").unwrap();
    let compiled_program = c_lang::compile(&program);
println!("{}", compiled_program);
    // assemble code
    let contents = assembly::compile_assembly_to_binary(&compiled_program);
    let bytes = assembly::string_to_bytes(contents.as_str());
    
    // run on VC
    let mut computer: Computer = Computer::new();
    computer.ram.insert_bytes(bytes);
    computer.run();
}
