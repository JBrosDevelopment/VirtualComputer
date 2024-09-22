use crate::vc_8bit::{Byte, Bit};
use regex::Regex;

/// # string_to_bytes
/// Converts a string to a vector of bytes
/// # Arguments
/// * `contents` - The string to convert
/// # Returns
/// * `Vec<Byte>` - The vector of bytes
/// # Examples
/// ```
/// use vc_8bit::vc_8bit::{Byte, Bit};
/// use vc_8bit::assembly::string_to_bytes;
/// let bytes = string_to_bytes("11010100");
/// assert_eq!(bytes, vec![Byte::new([Bit::new(true), Bit::new(false), Bit::new(true), Bit::new(false), Bit::new(true), Bit::new(false), Bit::new(false), Bit::new(true)])]);
/// ```
pub fn string_to_bytes(contents: &str) -> Vec<Byte> {
    let mut chars = contents.chars().collect::<Vec<char>>();
    chars.reverse();
    let mut bytes: Vec<Byte> = vec![];
    while let Some(c) = chars.pop() {
        let bit1 = if c == '1' { Bit::new(true) } else { Bit::new(false) };
        let bit2 = if chars.pop() == Some('1') { Bit::new(true) } else { Bit::new(false) };
        let bit3 = if chars.pop() == Some('1') { Bit::new(true) } else { Bit::new(false) };
        let bit4 = if chars.pop() == Some('1') { Bit::new(true) } else { Bit::new(false) };
        let bit5: Bit = if chars.pop() == Some('1') { Bit::new(true) } else { Bit::new(false) };
        let bit6: Bit = if chars.pop() == Some('1') { Bit::new(true) } else { Bit::new(false) };
        let bit7: Bit = if chars.pop() == Some('1') { Bit::new(true) } else { Bit::new(false) };
        let bit8: Bit = if chars.pop() == Some('1') { Bit::new(true) } else { Bit::new(false) };
        bytes.push(Byte::new([bit1, bit2, bit3, bit4, bit5, bit6, bit7, bit8]));
    }
    bytes
}
/// # compile_assembly_to_binary
/// Compiles assembly code to binary
/// # Arguments
/// * `contents` - The assembly code to compile
/// # Returns
/// * `String` - The binary code
/// # Examples
/// ```
/// use vc_8bit::vc_8bit::{Byte, Bit};
/// use vc_8bit::assembly::compile_assembly_to_binary;
/// let bytes = compile_assembly_to_binary("MOV R2 255");
/// assert_eq!(bytes, "1100101011111111");
/// ```
/// # Panics
/// This function will panic if the assembly code is invalid
/// ```should_panic
/// use vc_8bit::vc_8bit::{Byte, Bit};
/// use vc_8bit::assembly::compile_assembly_to_binary;
/// let bytes = compile_assembly_to_binary("BLAH 256");
/// ```
pub fn compile_assembly_to_binary(contents: &str) -> String {
    let mut out = String::new();
    let mut vars: Vec<(String, Byte)> = vec![];
    let lines = contents.split("\n").map(|x| x.trim()).filter(|x| *x != "").collect::<Vec<&str>>();
    for line in lines {
        let mut parts = line.split(" ").map(|x| x.trim()).filter(|x: &&str| *x != "").collect::<Vec<&str>>().into_iter().peekable();
        let mut stream = String::new();
        while let Some(part) = parts.peek() {
            if part.starts_with(";") {
                break;
            }
            match part.to_uppercase().as_str() {
                "%ASSIGN" => {
                    parts.next();
                    let name = parts.next().unwrap();
                    let value = get_binary(parts.next().unwrap(), &vars).chars().collect::<Vec<char>>();
                    vars.push((name.to_string(), Byte::new([Bit::new(value[0] == '1'), Bit::new(value[1] == '1'), Bit::new(value[2] == '1'), Bit::new(value[3] == '1'), Bit::new(value[4] == '1'), Bit::new(value[5] == '1'), Bit::new(value[6] == '1'), Bit::new(value[7] == '1')])));
                    break;
                }
                "ADD" => {
                    stream += "0000";
                    parts.next();
                    stream += get_register(parts.next().unwrap());
                    stream += get_register(parts.next().unwrap());
                }
                "SUB" => {
                    stream += "0001";
                    parts.next();
                    stream += get_register(parts.next().unwrap());
                    stream += get_register(parts.next().unwrap());
                }
                "MUL" => {
                    stream += "0010";
                    parts.next();
                    stream += get_register(parts.next().unwrap());
                    stream += get_register(parts.next().unwrap());
                }
                "DIV" => {
                    stream += "0011";
                    parts.next();
                    stream += get_register(parts.next().unwrap());
                    stream += get_register(parts.next().unwrap());
                }
                "STR" => {
                    stream += "110000";
                    parts.next();
                    stream += get_register(parts.next().unwrap());
                    stream += get_binary(parts.next().unwrap(), &vars).as_str();
                }
                "LDR" => {
                    stream += "110001";
                    parts.next();
                    stream += get_register(parts.next().unwrap());
                    stream += get_binary(parts.next().unwrap(), &vars).as_str();
                }
                "MOV" => {
                    stream += "110010";
                    parts.next();
                    stream += get_register(parts.next().unwrap());
                    stream += get_binary(parts.next().unwrap(), &vars).as_str();
                }
                "CPY" => {
                    stream += "110011";
                    parts.next();
                    stream += get_register(parts.next().unwrap());
                    stream += get_register(parts.next().unwrap());
                    stream += "000000";
                }
                "SHL" => {
                    stream += "110100";
                    parts.next();
                    stream += get_register(parts.next().unwrap());
                    stream += get_binary(parts.next().unwrap(), &vars).as_str();
                }
                "SHR" => {
                    stream += "110101";
                    parts.next();
                    stream += get_register(parts.next().unwrap());
                    stream += get_binary(parts.next().unwrap(), &vars).as_str();
                }
                "OUT" => {
                    stream += "110110";
                    parts.next();
                    stream += get_register(parts.next().unwrap());
                }
                "MSG" => {
                    stream += "110111";
                    parts.next();
                    stream += get_register(parts.next().unwrap());
                }
                "INC" => {
                    stream += "111000";
                    parts.next();
                    stream += get_register(parts.next().unwrap());
                }
                "DEC" => {
                    stream += "111001";
                    parts.next();
                    stream += get_register(parts.next().unwrap());
                }
                "JMP" => {
                    stream += "11101000";
                    parts.next();
                    stream += get_binary(parts.next().unwrap(), &vars).as_str();
                }
                "JMP_NEG" => {
                    stream += "11101001";
                    parts.next();
                    stream += get_binary(parts.next().unwrap(), &vars).as_str();
                }
                "JMP_ZRO" => {
                    stream += "11101010";
                    parts.next();
                    stream += get_binary(parts.next().unwrap(), &vars).as_str();
                }
                "JMP_ABV" => {
                    stream += "11101011";
                    parts.next();
                    stream += get_binary(parts.next().unwrap(), &vars).as_str();
                }
                "CMP_NEG" => {
                    stream += "111100";
                    parts.next();
                    stream += get_register(parts.next().unwrap());
                }
                "CMP_ZRO" => {
                    stream += "111101";
                    parts.next();
                    stream += get_register(parts.next().unwrap());
                }
                "CMP_ABV" => {
                    stream += "111110";
                    parts.next();
                    stream += get_register(parts.next().unwrap());
                }
                "HALT" => {
                    stream += "11111111";
                    parts.next();
                }
                "AND" => {
                    stream += "0100";
                    parts.next();
                    stream += get_register(parts.next().unwrap());
                    stream += get_register(parts.next().unwrap());
                }
                "OR" => {
                    stream += "0101";
                    parts.next();
                    stream += get_register(parts.next().unwrap());
                    stream += get_register(parts.next().unwrap());
                }
                "NOT" => {
                    stream += "0110";
                    parts.next();
                    stream += get_register(parts.next().unwrap());
                    stream += "00";
                }
                "XOR" => {
                    stream += "0111";
                    parts.next();
                    stream += get_register(parts.next().unwrap());
                    stream += get_register(parts.next().unwrap());
                }
                "RPRT" => {
                    stream += "100";
                    parts.next();
                    stream += get_register(parts.next().unwrap());
                    let address = get_binary(parts.next().unwrap(), &vars);
                    let bit3 = match address.as_str() {
                        "00000000" => "000",
                        "00000001" => "001",
                        "00000010" => "010",
                        "00000011" => "011",
                        "00000100" => "100",
                        "00000101" => "101",
                        "00000110" => "110",
                        "00000111" => "111",
                        _ => panic!("Invalid port address {address}. Must be 0-7"),
                    };
                    stream += bit3;
                }
                "WPRT" => {
                    stream += "101";
                    parts.next();
                    stream += get_register(parts.next().unwrap());
                    let address = get_binary(parts.next().unwrap(), &vars);
                    let bit3 = match address.as_str() {
                        "00000000" => "000",
                        "00000001" => "001",
                        "00000010" => "010",
                        "00000011" => "011",
                        "00000100" => "100",
                        "00000101" => "101",
                        "00000110" => "110",
                        "00000111" => "111",
                        _ => panic!("Invalid port address {address}. Must be 0-7"),
                    };
                    stream += bit3;
                }
                ";" => {
                    break;
                }
                _ => {
                    panic!("Invalid function call {part}");
                }
            }
            out += &stream;
        }
    }
    out
}
/// # Get Register
/// Gets the register from a string
/// # Arguments
/// * `content` - The string to get the register from
/// # Returns
/// * `String` - The register
/// # Examples
/// ```
/// use vc_8bit::vc_8bit::{Byte, Bit};
/// use vc_8bit::assembly::get_register;
/// let register = get_register("R0");
/// assert_eq!(register, "00");
/// ```    
/// # Panics
/// Will panic if the register is invalid
fn get_register(content: &str) -> &str {
    match content.chars().take_while(|x| x != &';').collect::<String>().to_uppercase().as_str() {
        "R0" => "00",
        "R1" => "01",
        "R2" => "10",
        "R3" => "11",
        _ => panic!("Invalid register")
    }
}

/// # Get Binary
/// Gets the binary from a string
/// # Arguments
/// * `content` - The string to get the binary from
/// # Returns
/// * `String` - The binary
/// # Examples
/// ```
/// use vc_8bit::vc_8bit::{Byte, Bit};
/// use vc_8bit::assembly::get_binary;
/// let binary = get_binary("255", &Vec::new());
/// assert_eq!(binary, "11111111");
/// ```
/// # Panics
/// Will panic if value is not a number, hexadecimal, binary sequence, or variable
fn get_binary(_content: &str, vars: &Vec<(String, Byte)>) -> String {
    let content = _content.chars().take_while(|x| x != &';').collect::<String>();
    let binary_regex = Regex::new(r"^#[01]{8}$").unwrap(); // Matches #00000000 (binary)
    let hex_regex = Regex::new(r"^0x[0-9A-Fa-f]+$").unwrap(); // Matches 0x00 (hexadecimal)
    let decimal_regex = Regex::new(r"\d+").unwrap(); // Matches any number (decimal)

    if binary_regex.is_match(content.as_str()) {
        // It's a binary string
        content.replace("#", "")
    } else if hex_regex.is_match(content.as_str()) {
        // It's a hexadecimal string
        hex_to_binary(content.replace("0x", "")).unwrap()
    } else if decimal_regex.is_match(content.as_str()) {
        // It's a decimal string
        decimal_to_binary(content.parse().unwrap())
    } else {
        // It's a variable string
        if vars.iter().any(|x| x.0 == content) {
            vars.iter().find(|x| x.0 == content).unwrap().1.to_string()
        }
        else {
            panic!("Variable not found");
        }
    }
}
fn hex_to_binary(hex: String) -> Result<String, String> {
    let mut binary_string = String::new();

    for c in hex.chars().take_while(|x| x != &';') {
        match c {
            '0' => binary_string.push_str("0000"),
            '1' => binary_string.push_str("0001"),
            '2' => binary_string.push_str("0010"),
            '3' => binary_string.push_str("0011"),
            '4' => binary_string.push_str("0100"),
            '5' => binary_string.push_str("0101"),
            '6' => binary_string.push_str("0110"),
            '7' => binary_string.push_str("0111"),
            '8' => binary_string.push_str("1000"),
            '9' => binary_string.push_str("1001"),
            'A' | 'a' => binary_string.push_str("1010"),
            'B' | 'b' => binary_string.push_str("1011"),
            'C' | 'c' => binary_string.push_str("1100"),
            'D' | 'd' => binary_string.push_str("1101"),
            'E' | 'e' => binary_string.push_str("1110"),
            'F' | 'f' => binary_string.push_str("1111"),
            _ => return Err(format!("Invalid hex character: {}", c)),
        }
    }

    Ok(binary_string)
}

fn decimal_to_binary(decimal: u8) -> String {
    format!("{:08b}", decimal)
}