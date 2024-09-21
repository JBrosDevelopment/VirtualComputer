use vc_8bit::{assembly, vc_8bit::Computer};

fn main() {
    let program = std::fs::read_to_string("src/program.asm").unwrap();
    
    // assemble code
    let contents = assembly::compile_assembly_to_binary(&program);
    let bytes = assembly::string_to_bytes(contents.as_str());
    
    // run on VC
    let mut computer: Computer = Computer::new();
    computer.ram.insert_bytes(bytes);
    computer.run();
}
