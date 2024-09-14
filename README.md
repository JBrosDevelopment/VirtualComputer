# Virtual Computer, Assembler, and Compiler

![Cover](https://raw.githubusercontent.com/JBrosDevelopment/VirtualComputer/refs/heads/master/blog/blog%20cover.png)

This project is a virtual 8 bit computer that takes a vector of bytes and runs it as instructions It includes the virtual machine, assembler, and compiler for custom assembly and high level language.

I created a blog post about this project and you can find that on **[Hashnode](https://jbrosdev.hashnode.dev/making-a-virtual-machine-binary-to-assembly-to-c-all-in-rust)**. It goes deeper into explaining the different parts of this project.

## Add to Your Project

You can add this to your Rust project with the terminal command:

```
cargo add vc_8bit
```

The code shown below assumes you are using the following modules:

```rs
use vc_8bit::{assembly, c_lang, vc_8bit::Computer};
```


## Virtual Computer

The virtual computer works by instantiating the Computer and inserting the program into the ram as bytes. 

```rs
let bytes = vec![Byte::from_string("11010100"), Byte::from_string("1001100")];
let mut computer: Computer = Computer::new();
computer.ram.insert_bytes(bytes);
computer.run();
```

The **VC** (Virtual Computer) is basically a big function that will take an array of bytes and run the instructions associated with the bytes. I have emulated components like a **Binary Decoder**, **RAM**, **ALU**, and **CPU**.

What the VC does is take the binary and figure out the instructions that go with it. It uses Binary Decoders in a *match* statement to decide what instruction it is. Every Byte is an instruction. Some examples of instructions are moving from memory to registers, adding the values from one register to another using the ALU. This is why assembly is basically binary. Maybe the binary sequence `01001110` is the `MOV` instruction. All an assembler does is convert the instructions like `CPY`, `LDR`, and `ADD` to there corrosponding bytes. It gets a little more complex than this. For this VC, the first 6 bits are for the instruction, and the last 2 bits are for the register. If this sounds interesting to you, I highly recommend watching [Core Dumpped](https://www.youtube.com/@CoreDumpped) and his videos. 

When working with the VC, remember the RAM has 256 byte limit because the VC is only an 8 bit computer compared to modern 64 bit computers.  

## Assembler

The assembler works by first assembling the code to binary. It will then turn the binary into an array of bytes. 
```rs
let value = "MOV R0 50";

// assemble code
let contents = assembly::compile_assembly_to_binary(value.to_string());
let bytes = assembly::string_to_bytes(contents.as_str());

// run on VC
let mut computer: Computer = Computer::new();
computer.ram.insert_bytes(bytes);
computer.run();
```

The assembler will go line by line the code into binary. I created a custom assembly language to work with the VC. 

- `HALT`: Stops the program
- `STR R0 #0000000`: Stores the value in the register to the address in memory
- `LDR R0 #0000000`: Loads the value in the memory address to the register
- `MOV R0 #0000000`: Moves a byte value into a register
- `CPY R0 R1`: Copys the value of 1 register to another
- `SHR R0 #0000000`: Shifts a register value by the left many times the number in the byte is
- `SHL R0 #0000000`: Shifts a register value by the right however many times the number in the byte is
- `OUT R0`: Outputs the value in the register to the console as a byte value
- `MSG R0`: Outputs the value in the register to the console as an ASCII character
- `INC R0`: Increments the value in the register
- `DEC R0`: Decrements the value in the register
- `JMP #0000000`: Moves the RAM index 
- `JMP_NEG #0000000`: Moves the RAM index if the ALU Negative flag is on
- `JMP_ZRO #0000000`: Moves the RAM index if the ALU Zero flag is on
- `JMP_ABV #0000000`: Moves the RAM index if neither the ALU negative flag or Zero flag is on
- `CMP_NEG R0 #0000000`: Moves `11111111` to the register if the ALU negative flag is on. If not, it moves `00000000` to the register
- `CMP_ZRO R0 #0000000`: Moves `11111111` to the register if the ALU zero flag is on. If not, it moves `00000000` to the register
- `CMP_ABV R0 #0000000`: Moves `11111111` to the register if neither the ALU negative flag or Zero flag is on. If not, it moves `00000000` to the register
- `ADD R0 R1`: Adds the byte value from registers 0 and 1 and moves the result to the first register 
- `SUB R0 R1`: Subtracts the byte value from registers 0 and 1 and moves the result to the first register 
- `MUL R0 R1`: Multiplies the byte value from registers 0 and 1 and moves the result to the first register 
- `DIV R0 R1`: Divides the byte value from registers 0 and 1 and moves the result to the first register 
- `AND R0 R1`: Does an and operation on the bytes from registers 0 and 1 and moves the result to the first register
- `OR R0 R1`: Does an or operation on the bytes from registers 0 and 1 and moves the result to the first register
- `XOR R0 R1`: Does an exclusive or operation on the bytes from registers 0 and 1 and moves the result to the first register
- `NOT R0`: Does a not operation on the byte in register 1 and moves result to it
- `RPRT R0 #0000000`: Reads the value in the port at the address and writes the value to register 0. The address needs to be 0 through 7.
- `WPRT R0 #0000000`: Writes the value in register 0 to the port at the address. The address needs to be 0 through 7.

The assembler will identify integers, bytes, and hexadecimals:

```
MOV R0 5 ; moves 5 into R0
MOV R1 0x3A ; moves the hexadecimal 3A (integer 58) to R1
MOV R2 #00110100 ; moves the byte value 00110100 (integer 52) to R2
```

There is also the `%ASSIGN` feature which can be used as *constants* inside the assembly

```
%ASSIGN VARIABLE_ADDRESS #11110100

MOV R0 37
STR R0 VARIABLE_ADDRESS
```

## Compiler

The compiler works by compiling the code into assembly.

```rs
let value = "print('a');";

// compile code
let asm = c_lang::compile(value.to_string());

// assemble code
let contents = assembly::compile_assembly_to_binary(&asm);
let bytes = assembly::string_to_bytes(contents.as_str());

// run on VC
let mut computer: Computer = Computer::new();
computer.ram.insert_bytes(bytes);
computer.run();
```

### Language

The language looks like C with a few key distinctions. 

There are 3 types:

- `uint8` an unsigned 8 bit integer. Any whole number between 0 and 255.
- `bool` a true of false value. Is either `00000000` or `11111111` in binary
- `char` a character value. Uses the ASCII codes to convert to binary

There are 5 functions:
- `to_char(uint8)` turns a number into a character. This works for numbers 0 - 9. Any number above that will get the ASCII character of that number plus 48. This is because 48 is the ASCII character for 0. Here is an example of th function: `char c = to_char(7)`
- `out(any)` will output the byte value to the console of any type. For example, the value 5 will output 00000101.
- `print(char)` will output the character to the console.
- `write_port(uint8, any)` will write the byte value to the port number 0 - 7. The address needs to be a constant. For example `write_port(2, 'a')` is fine but `write_port(99 - 3, 'a')` and `write_port(variable, 'a')` will not work.
- `read_port(uint8)` will read the byte value from the port address 0 - 7. The address needs to be a constant. For example `read_port(2)` is fine but `read_port(99 - 3)` and `read_port(variable)` will not work.

There is only the `if` and `while` statements. The code inside of the statement needs to be seperated by commas and not semicolons. The last line inside the statement can not have a comma. The ending bracket of the statement needs to end with a semicolon. Here is what it looks like:

```c
if (true && 0 == 0) {
    char c = 'a',
    print(c)
};
```

All operators work except for the `+=` type of operators. Just use `A = A + B` instead. For the `<<` and `>>` operators, the right operand needs to be constant. For example, `'a' >> 3` works but `'a' >> 3 + 1` or `'a' >> variable` do not work.

- `++`, `--` increment and decrement. Works with `uint8`.
- `+`, `-`, `*`, `/` arithmatic operators. Works with `uint8`.
- `&&`, `||`, `==`, `!=` logic operators. Works with `bool`.
- `<<`, `>>` bit shift operators. Shifting amount needs to be constant. Works with any type.
- `&`, `|`, `^`, `!` and, or, excluseive or, and not operators. Works with any type.
- `=` set variable.

There is no way to define functions in this language. Also remember there are only 256 bytes in memory to work with in the VC. This means your program must be less than 255 bytes, and any variables that you use will take up one of those bytes.

### Example

This simple program took me 27 bytes to write it in assembly. The compiler was able to use 38 bytes. It's not the smartest compiler it works.
```c
uint8 a = 0;

while (a < 5) {
    a = a + 1,
    char c = to_char(a),
    print(c)
}
```

The assembly output for this code is as follows:

```
MOV R0 #00000000
STR R0 #11111110 ; store created variable ; BYTE ADDRESS 4
LDR R0 #11111110 ; load variable ; value left
MOV R1 #00000101 ; value right
SUB R0 R1
CMP_NEG R0 ; compare
CPY R3 R0 ; copy value right ; get value for statement
MOV R2 0 ; set R2 to 0
SUB R2 R3 ; check if statement is true
JMP_ZRO 37 ; jump if false
LDR R0 #11111110 ; load variable ; value left
MOV R1 #00000001 ; value right
ADD R0 R1 ; math
STR R0 #11111110 ; store variable
LDR R3 #11111110 ; load variable
MOV R2 48 ; 48 is ascii '0'
ADD R3 R2 ; get ascii
CPY R0 R3 ; move value to correct register
STR R0 #11111101 ; store created variable
LDR R2 #11111101 ; load variable
MSG R2 ; print value
JMP 4 ; jump back to start ; BYTE ADDRESS 38
HALT
```

The binary code that was assembled from this:

```
110010000000000011000000111111101100010011111110110010010000010100010001111100001100111100000000110010100000000000011011111010100010010111000100111111101100100100000001000000011100000011111110110001111111111011001010001100000000111011001100110000001100000011111101110001101111110111011110111010000000010011111111
```