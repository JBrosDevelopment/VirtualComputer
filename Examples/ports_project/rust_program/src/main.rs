use vc_8bit::{assembly, c_lang, vc_8bit::Computer};
use regex::Regex;

fn main() {
    let program = std::fs::read_to_string("src/program.c").unwrap();

    // Regular expression to match binary literals
    let re = Regex::new(r"0b[01]+").unwrap();

    let updated_program = re.replace_all(&program, |caps: &regex::Captures| {
        // Extract the binary number without the "0b" prefix
        let binary_str = &caps[0][2..]; 
        // Convert the binary string to an integer
        let integer_value = isize::from_str_radix(binary_str, 2).unwrap();
        // Return the integer value as a string
        integer_value.to_string()
    }).into_owned();

    let mut constants: Vec<(String, String)> = vec![];
    let mut lines: Vec<String> = vec![];
    for line in updated_program.lines().map(|x| x.to_string()).collect::<Vec<String>>() {
        let lexer_line = c_lang::get_lexer_line(line.as_str(), 0);
        let parts = lexer_line.tokens.iter().map(|x| x.value.to_string()).collect::<Vec<String>>();
        if parts.len() == 4 && parts[0] == "const" && parts[2] == "=" {
            let name = parts[1].to_string();
            let value = parts[3].to_string();
            constants.push((name.to_string(), value.to_string()));
        } 
        else {
            if parts.iter().any(|x| constants.iter().any(|y| x == &y.0)) {
                let mut new_parts: Vec<String> = vec![]; 
                for i in 0..parts.len() {
                    match constants.iter().find(|x| x.0 == parts[i]) {
                        Some(c) => new_parts.push(c.1.to_string()),
                        None => new_parts.push(parts[i].to_string()),
                    };
                }
                lines.push(format!("{};", new_parts.join(" ")));
            }
            else {
                lines.push(format!("{};", line.to_string()));
            }
        }
    };
    let program_with_constants = lines.join("\n");

    println!("- UPDATED CODE:\n{}", program_with_constants);

    c_lang::run_compiled_code_with_debugging(&program_with_constants);
    /*
    // compile code
    let asm = c_lang::compile(&program_with_constants);
    
    // assemble code
    let contents = assembly::compile_assembly_to_binary(&asm);
    let bytes = assembly::string_to_bytes(contents.as_str());
    
    // run on VC
    let mut computer: Computer = Computer::new();
    computer.ram.insert_bytes(bytes);
    computer.run();
    */
}
