pub mod assembly;
pub mod c_lang;

pub mod vc_8bit {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Bit {
        pub value: bool
    }
    impl From<bool> for Bit {
        fn from(b: bool) -> Self {
            Bit { value: b }
        }
    }
    impl std::convert::TryFrom<i32> for Bit {
        type Error = &'static str;

        fn try_from(value: i32) -> Result<Self, Self::Error> {
            match value {
                0 | 1 => Ok(Bit { value: value != 0 }),
                _ => Err("Invalid value for Bit. Must be 0 or 1."),
            }
        }
    }
    impl Bit {
        pub fn zero() -> Self {
            Bit { value: false }
        }
        pub fn one() -> Self {
            Bit { value: true }
        }
        pub fn to_i32(&self) -> i32 {
            if self.value {
                1
            } else {
                0
            }
        }
        pub fn to_bool(&self) -> bool {
            self.value
        }
        pub fn new(value: bool) -> Self {
            Bit { value }
        }
        pub fn not(&self) -> Self {
            Bit { value: !self.value }
        }
        pub fn and(&self, other: &Self) -> Self {
            Bit { value: self.value & other.value }
        }
        pub fn or(&self, other: &Self) -> Self {
            Bit { value: self.value | other.value }
        }
        pub fn xor(&self, other: &Self) -> Self {
            Bit { value: self.value ^ other.value }
        }
        pub fn nand(&self, other: &Self) -> Self {
            Bit { value: !(self.value & other.value) }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Byte {
        pub value: [Bit; 8]
    }
    pub const MAXBYTE: i32 = 255;
    impl std::convert::TryFrom<i32> for Byte {
        type Error = &'static str;

        fn try_from(value: i32) -> Result<Self, Self::Error> {
            match value {
                0..=MAXBYTE => {
                    let mut bits = [Bit::new(false); 8];
                    let bit_count = bits.len();
                    for (i, bit) in bits.iter_mut().enumerate() {
                        bit.value = (value >> (bit_count - 1 - i)) & 1 != 0;
                    }
                    Ok(Byte { value: bits })
                }
                _ => Err("Invalid value for Byte. Must be between 0 and 255."),
            }
        }
    }
    impl Byte {
        pub fn from_string(value: String) -> Byte {
            if value.len() == 8 {   
                let mut bits = [Bit::new(false); 8];
                for i in 0..8 {
                    bits[i] = Bit { value: value.chars().nth(i).unwrap() == '1' };
                }
                Byte::new(bits)
            }
            else {
                Byte::try_from(value.parse::<i32>().unwrap()).unwrap()
            }
        }
        pub fn reverse(&mut self) -> Self {
            let mut bits = self.value;
            for i in 0..8 {
                bits[7 - i] = self.value[i];
            }
            Byte { value: bits }
        } 
        pub fn zero() -> Self {
            Byte { value: [Bit::new(false); 8] }
        } 
        pub fn full() -> Self {
            Byte { value: [Bit::new(true); 8] }
        }
        pub fn new(bits: [Bit; 8]) -> Self {
            Byte { value: bits }
        }
        pub fn to_i32(&self) -> i32 {
            let mut value = 0;
            let bit_count = self.value.len();
            for (i, bit) in self.value.iter().enumerate() {
                if bit.value {
                    value |= 1 << (bit_count - 1 - i);
                }
            }
            value
        }
        pub fn to_u8(&self) -> u8 {
            let mut value = 0;
            for (i, bit) in self.value.iter().enumerate() {
                if bit.value {
                    value |= 1 << i;
                }
            }
            value
        }
        pub fn from_u8(mut value: u8) -> Self {
            let mut bits = [Bit { value: false }; 8];

            for i in 0..8 {
                bits[i].value = (value & 1) == 1;
                value >>= 1;
            }
            
            bits.reverse();
            Byte { value: bits }
        }
        pub fn to_u8_array(&self) -> [u8; 8] {
            let mut value = [0; 8];
            for (i, bit) in self.value.iter().enumerate() {
                value[i] = if bit.value { 1 } else { 0 };
            }
            value
        }
        pub fn not(&self) -> Self {
            let mut bits = [Bit::new(false); 8];
            for (i, bit) in bits.iter_mut().enumerate() {
                bit.value = !self.value[i].to_bool();
            }
            Byte { value: bits }
        }
        pub fn and(&self, other: &Self) -> Self {
            let mut bits = [Bit::new(false); 8];
            for (i, bit) in bits.iter_mut().enumerate() {
                bit.value = self.value[i].and(&other.value[i]).to_bool();
            }
            Byte { value: bits }
        }
        pub fn or(&self, other: &Self) -> Self {
            let mut bits = [Bit::new(false); 8];
            for (i, bit) in bits.iter_mut().enumerate() {
                bit.value = self.value[i].or(&other.value[i]).to_bool();
            }
            Byte { value: bits }
        }
        pub fn xor(&self, other: &Self) -> Self {
            let mut bits = [Bit::new(false); 8];
            for (i, bit) in bits.iter_mut().enumerate() {
                bit.value = self.value[i].xor(&other.value[i]).to_bool();
            }
            Byte { value: bits }
        }
        pub fn nand(&self, other: &Self) -> Self {
            let mut bits = [Bit::new(false); 8];
            for (i, bit) in bits.iter_mut().enumerate() {
                bit.value = self.value[i].nand(&other.value[i]).to_bool();
            }
            Byte { value: bits }
        }
        pub fn to_char(&self) -> char {
            std::char::from_u32(self.to_i32() as u32).unwrap()    
        }
        pub fn to_string(&self) -> String {
            format!("{}{}{}{}{}{}{}{}",
                self.value[0].to_i32(),
                self.value[1].to_i32(),
                self.value[2].to_i32(),
                self.value[3].to_i32(),
                self.value[4].to_i32(),
                self.value[5].to_i32(),
                self.value[6].to_i32(),
                self.value[7].to_i32())
        }
        pub fn from_byte_to_bit_array(byte: Byte) -> [Bit; 8] {
            byte.value
        }
        pub fn increment(&mut self) {
            self.value = Byte::from_byte_to_bit_array(self.and(&Byte::full()).or(&self.not().and(&Byte::zero())));
        }
        pub fn decrement(&mut self) {
            let i32 = self.to_i32();
            if i32 == 0 {
                self.value = Byte::try_from(MAXBYTE).unwrap().value
            }
            self.value = Byte::try_from(i32 - 1).unwrap().value;
        }
        pub fn shift_array(value: Byte, shift: i32) -> Byte {
            let arr = [value.value[0].to_bool(), value.value[1].to_bool(), value.value[2].to_bool(), value.value[3].to_bool(), value.value[4].to_bool(), value.value[5].to_bool(), value.value[6].to_bool(), value.value[7].to_bool()];
            let mut result = [false; 8];
            let len = arr.len() as i32;
        
            if shift == 0 {
                return Byte::new([arr[0].try_into().unwrap(), arr[1].try_into().unwrap(), arr[2].try_into().unwrap(), arr[3].try_into().unwrap(), arr[4].try_into().unwrap(), arr[5].try_into().unwrap(), arr[6].try_into().unwrap(), arr[7].try_into().unwrap()]);
            } else if shift > 0 {
                // Right shift
                for i in 0..len {
                    if i + shift < len {
                        result[(i + shift) as usize] = arr[i as usize];
                    }
                }
            } else {
                // Left shift (negative shift)
                for i in 0..len {
                    if i - shift < len {
                        result[i as usize] = arr[(i - shift) as usize];
                    }
                }
            }
        
            Byte::new([result[0].try_into().unwrap(), result[1].try_into().unwrap(), result[2].try_into().unwrap(), result[3].try_into().unwrap(), result[4].try_into().unwrap(), result[5].try_into().unwrap(), result[6].try_into().unwrap(), result[7].try_into().unwrap()])
        }
        pub fn shift(&mut self, direction: i32) {
            self.value = Byte::shift_array(*self, direction).value;
        }
        pub fn to_bool_array(&self) -> [bool; 8] {
            let mut result = [false; 8];
            for (i, bit) in self.value.iter().enumerate() {
                result[i] = bit.to_bool();
            }
            result
        }
        pub fn to_hex(&self) -> String {
            format!("{:02x}", self.to_i32())
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Register {
        pub value: Byte,
        pub address: Byte
    }
    impl Register {
        pub fn new(address: Byte, value: Byte) -> Self {
            Register { address, value }
        }
        pub fn read(&self) -> Byte {
            self.value
        }
        pub fn write(&mut self, value: Byte) {
            self.value = value;
        }
    }

    const RAM_SIZE: usize = 256;
    const STREAM_SIZE: usize = 2;
    #[derive(Debug, Clone, Copy)]
    pub struct RAM {
        pub value: [Register; RAM_SIZE],
        pub index: Byte
    }
    impl RAM {
        pub fn new() -> Self {
            let mut registers: [Register; RAM_SIZE] = [Register::new(Byte::new([Bit::new(false); 8]), Byte::new([Bit::new(false); 8])); RAM_SIZE];
            for i in 0..RAM_SIZE {
                registers[i] = Register::new(
                    Byte::try_from(i as i32).unwrap(),
                    Byte::zero()
                    );
            }
            RAM { value: registers, index: Byte::zero() }
        }
        pub fn insert_bytes(&mut self, bytes: Vec<Byte>) {
            for (i, byte) in bytes.iter().enumerate() {
                self.write(Byte::try_from(i as i32).unwrap(), *byte);
            }
        }
        pub fn read(&self, address: Byte) -> Byte {
            self.value[address.to_i32() as usize].value
        }
        pub fn write(&mut self, address: Byte, value: Byte) {
            self.value[address.to_i32() as usize].value = value;
        }
        pub fn increment(&mut self) {
            self.index.increment();
        }
        pub fn decrement(&mut self) {
            self.index.decrement();
        }
        pub fn get_index(&self) -> Byte {
            self.index
        }
        pub fn set_index(&mut self, index: Byte) {
            self.index = index;
        }
        pub fn get_byte_stream(&mut self) -> [Byte; STREAM_SIZE] {
            let mut bytes: [Byte; STREAM_SIZE] = [Byte::zero(); STREAM_SIZE];
            let i32_index = self.index.to_i32();
            for i in 0..STREAM_SIZE {
                let index = i + i32_index as usize;
                bytes[i] = self.read(Byte::try_from(index as i32).unwrap());
            }
            self.index = Byte::try_from(i32_index + STREAM_SIZE as i32).unwrap();
            bytes
        }
    }

    const PORTS_SIZE: usize = 8;
    pub struct Ports {
        pub bus_dir: String
    }
    impl Ports {
        pub fn new(bus_dir: String) -> Self {
            if std::path::Path::new(&bus_dir).exists() {
                std::fs::remove_dir_all(&bus_dir).unwrap();
            }
            std::fs::create_dir_all(&bus_dir).unwrap();
            for i in 0..PORTS_SIZE {
                std::fs::File::create(format!("{}{}", bus_dir, i)).unwrap();
                _ = std::fs::write(format!("{}{}", bus_dir, i), Byte::zero().to_string());
            }
            Ports { bus_dir }
        }
        pub fn default() -> Self {
            Ports::new(String::from("src/ports/"))
        }
        pub fn read(&self, address: Byte) -> Byte {
            let file_contents = std::fs::read_to_string(format!("{}{}", self.bus_dir, address.to_i32())).unwrap();
            let byte = Byte::new(
                file_contents.chars()
                    .map(|x| x.to_digit(10).unwrap() as u8)
                    .map(|x| Bit::new(x == 1))
                    .collect::<Vec<Bit>>()
                    .try_into()
                    .unwrap()
            );
            byte
        }
        pub fn write(&self, address: Byte, value: Byte) {
            std::fs::File::create(format!("{}{}", self.bus_dir, address.to_i32())).unwrap();
            _ = std::fs::write(format!("{}{}", self.bus_dir, address.to_i32()), value.to_string());
        }
        pub fn clear(&self) {
            for i in 0..PORTS_SIZE {
                self.write(Byte::try_from(i as i32).unwrap(), Byte::zero());
            }
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct ByteArithmetic {
        pub value1: Byte,
        pub value2: Byte,
        pub carry: Bit,
        pub neg: Bit,
        pub zero: Bit,
    }

    impl ByteArithmetic {
        pub fn new(value1: Byte, value2: Byte) -> Self {
            ByteArithmetic {
                value1,
                value2,
                carry: Bit::zero(),
                neg: Bit::zero(),
                zero: Bit::zero(),
            }
        }

        pub fn add(&mut self) -> (Byte, Bit) {
            let (result, overflow) = self.value1.to_i32().overflowing_add(self.value2.to_i32());
            self.carry = Bit::from(overflow);
            self.neg = Bit::from(result < 0);
            self.zero = Bit::from(result == 0);
            (Byte::try_from(result).unwrap(), self.carry)
        }

        pub fn sub(&mut self) -> (Byte, Bit) {
            let (result, overflow) = self.value1.to_i32().overflowing_sub(self.value2.to_i32());
            self.carry = Bit::from(overflow);
            self.neg = Bit::from(result < 0);
            self.zero = Bit::from(result == 0);
            (Byte::try_from(result.abs()).unwrap(), self.carry)
        }

        pub fn mul(&mut self) -> (Byte, Bit) {
            let (result, overflow) = self.value1.to_i32().overflowing_mul(self.value2.to_i32());
            self.carry = Bit::from(overflow);
            self.neg = Bit::from(result < 0);
            self.zero = Bit::from(result == 0);
            (Byte::try_from(result.abs()).unwrap(), self.carry)
        }

        pub fn div(&mut self) -> (Byte, Bit) {
            if self.value2.to_i32() == 0 {
                panic!("Division by zero");
            }
            let (result, overflow) = self.value1.to_i32().overflowing_div(self.value2.to_i32());
            self.carry = Bit::from(overflow);
            self.neg = Bit::from(result < 0);
            self.zero = Bit::from(result == 0);
            (Byte::try_from(result.abs()).unwrap(), self.carry)
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct BinaryDecoder {
        pub axis_x: Bit,
        pub axis_y: Bit,
        pub result: [Bit; 4],
    }
    impl BinaryDecoder {
        pub fn new() -> Self {
            BinaryDecoder {
                axis_x: Bit::zero(),
                axis_y: Bit::zero(),
                result: [Bit::zero(); 4],
            }
        }
        pub fn new_with_value(axis_x: Bit, axis_y: Bit) -> Self {
            BinaryDecoder {
                axis_x,
                axis_y,
                result: [Bit::zero(); 4],
            }
        }
        pub fn update(&mut self, axis_x: Bit, axis_y: Bit) {
            self.axis_x = axis_x;
            self.axis_y = axis_y;
            self.decode();
        }
        pub fn decode(&mut self) {
            self.result = Self::decode_internal(self.axis_x, self.axis_y);
        }
        pub fn decode_internal(axis_x: Bit, axis_y: Bit) -> [Bit; 4] {
            if !axis_x.value && !axis_y.value {
                Self::BIT1
            }
            else if !axis_x.value && axis_y.value {
                Self::BIT2
            }
            else if axis_x.value && !axis_y.value {
                Self::BIT3
            }
            else {
                Self::BIT4
            }
        }
        pub fn decode_internal_i32(axis_x: Bit, axis_y: Bit) -> i32 {
            if !axis_x.value && !axis_y.value {
                1
            }
            else if !axis_x.value && axis_y.value {
                2
            }
            else if axis_x.value && !axis_y.value {
                3
            }
            else {
                4
            }
        }
        
        pub const BIT0: [Bit; 4] = [Bit { value: false }, Bit { value: false }, Bit { value: false }, Bit { value: false }];
        pub const BIT1: [Bit; 4] = [Bit { value: true }, Bit { value: false }, Bit { value: false }, Bit { value: false }];
        pub const BIT2: [Bit; 4] = [Bit { value: false }, Bit { value: true }, Bit { value: false }, Bit { value: false }];
        pub const BIT3: [Bit; 4] = [Bit { value: false }, Bit { value: false }, Bit { value: true }, Bit { value: false }];
        pub const BIT4: [Bit; 4] = [Bit { value: false }, Bit { value: false }, Bit { value: false }, Bit { value: true }];
    }

    #[derive(Debug, Clone, Copy)]
    pub struct ALU {
        pub value1: Byte,
        pub value2: Byte,
        pub decoder: BinaryDecoder,
        pub math: ByteArithmetic,
        pub overflow: Bit,
        pub zero: Bit,
        pub negative: Bit,
    }

    impl ALU {
        pub fn new(value1: Byte, value2: Byte) -> Self {
            ALU {
                value1,
                value2,
                decoder: BinaryDecoder::new(),
                math: ByteArithmetic::new(value1, value2),
                overflow: Bit::new(false),
                zero: Bit::new(false),
                negative: Bit::new(false),
            }
        }
        pub fn set_decoder(&mut self, axis_x: Bit, axis_y: Bit) {
            self.decoder = BinaryDecoder { axis_x, axis_y, result: [Bit::zero(); 4] };
        }
        pub fn move_bytes_in(&mut self, value1: Byte, value2: Byte) {
            self.value1 = value1;
            self.value2 = value2; 
        }
        pub fn compute(&mut self) {
            self.decoder.decode();
            self.compute_decoder(self.decoder);
        }
        pub fn compute_decoder(&mut self, decoder: BinaryDecoder) {
            let half_byte = BinaryDecoder::decode_internal(decoder.axis_x, decoder.axis_y);
            match half_byte {
                BinaryDecoder::BIT1 => self.add(),
                BinaryDecoder::BIT2 => self.sub(),
                BinaryDecoder::BIT3 => self.mul(),
                BinaryDecoder::BIT4 => self.div(),
                _ => {}
            }
        }
        pub fn add(&mut self) {
            self.math.value1 = self.value1;
            self.math.value2 = self.value2;
            let (result, carry) = self.math.add();
            self.value1 = result;
            self.overflow = carry;

            self.zero = Bit::new(result.to_i32() == 0);
            self.negative = Bit::new(result.to_i32() < 0);
        }

        pub fn sub(&mut self) {
            self.math.value1 = self.value1;
            self.math.value2 = self.value2;
            let (result, carry) = self.math.sub();
            self.value1 = result;
            self.overflow = carry;

            self.zero = Bit::new(result.to_i32() == 0);
            self.negative = Bit::new(result.to_i32() < 0);
        }

        pub fn mul(&mut self) {
            self.math.value1 = self.value1;
            self.math.value2 = self.value2;
            let (result, carry) = self.math.mul();
            self.value1 = result;
            self.overflow = carry;

            self.zero = Bit::new(result.to_i32() == 0);
            self.negative = Bit::new(result.to_i32() < 0);
        }

        pub fn div(&mut self) {
            if self.value2.to_i32() == 0 {
                panic!("Division by zero");
            }
            self.math.value1 = self.value1;
            self.math.value2 = self.value2;
            let (result, carry) = self.math.div();
            self.value1 = result;
            self.overflow = carry;

            self.zero = Bit::new(result.to_i32() == 0);
            self.negative = Bit::new(result.to_i32() < 0);
        }
    }

    #[derive(Debug, Clone)]
    pub struct CPU {
        pub alu: ALU,
        pub reg_1: Register,
        pub reg_2: Register,
        pub reg_3: Register,
        pub reg_4: Register,
    }
    impl CPU {
        pub fn new() -> Self {
            CPU {
                alu: ALU::new(Byte::zero(), Byte::zero()),
                reg_1: Register::new(Byte::try_from(65).unwrap(), Byte::zero()),
                reg_2: Register::new(Byte::try_from(66).unwrap(), Byte::zero()),
                reg_3: Register::new(Byte::try_from(67).unwrap(), Byte::zero()),
                reg_4: Register::new(Byte::try_from(68).unwrap(), Byte::zero()),
            }
        }
        pub fn move_byte_in_register_1(&mut self, reg_1: Byte) {
            self.reg_1.value = reg_1;
        }
        pub fn move_byte_in_register_2(&mut self, reg_2: Byte) {
            self.reg_2.value = reg_2;
        }
        pub fn move_byte_in_register_3(&mut self, reg_3: Byte) {
            self.reg_3.value = reg_3;
        }
        pub fn move_byte_in_register_4(&mut self, reg_4: Byte) {
            self.reg_4.value = reg_4;
        }
        pub fn get_register(&mut self, reg: i32) -> Register {
            match reg {
                1 => self.reg_1,
                2 => self.reg_2,
                3 => self.reg_3,
                4 => self.reg_4,
                _ => panic!("Invalid register"),
            }
        }
    }

    pub struct Computer {
        pub cpu: CPU,
        pub ports: Ports,
        pub ram: RAM,
    }

    impl Computer {
        pub fn new() -> Self {
            Computer {
                cpu: CPU::new(),
                ports: Ports::default(),
                ram: RAM::new(),
            }
        }
        pub fn move_byte_in_memory(&mut self, address_1: Byte, address_2: Byte) {
            let read = self.ram.read(address_1);
            self.ram.write(address_2, read);
        }
        pub fn byte_into_register(&mut self, mut register: Register, value: Byte) {
            register.write(value);
        }
        pub fn move_byte_to_register(&mut self, reg: i32, address_1: Byte) {
            self.cpu.get_register(reg).write(self.ram.read(address_1));
        }
        pub fn move_byte_from_register(&mut self, register: Register, address_1: Byte) {
            self.ram.write(address_1, register.read());
        }
        pub fn run(&mut self) {
            loop {
                // get the 2 byte stream
                let data = self.ram.get_byte_stream();
                
                // run the stream with the first byte being the instruction
                let halted = self.run_stream(data);
                
                // stop the program if the instruction is HALT 111111111
                if halted {
                    break;
                }
            }
        }
        pub fn run_stream(&mut self, stream: [Byte; STREAM_SIZE]) -> bool {
            let first_byte = stream[0].value;
            let mut halted = false;

            // 00 ALU 
            // 01 Boolean
            // 10 Ports
            // 11 RAM 
            
            if !first_byte[0].value { // 0X XX XX XX
                // ALU and logic takes 1 byte, Memory takes 2 bytes
                // decrement so next instruction won't be skipped
                self.ram.decrement();

                if first_byte[1].value { // 01 XX XX XX
                    // Boolean Logic
                    let mut reg1_data: [u8; 2] = [0; 2];
                    for i in 0..2 {
                        reg1_data[i] = first_byte[4 + i].to_i32() as u8;
                    } 
                    let mut reg2_data: [u8; 2] = [0; 2];
                    for i in 0..2 {
                        reg2_data[i] = first_byte[6 + i].to_i32() as u8;
                    } 
                    let register1_address = BinaryDecoder::decode_internal_i32(Bit::try_from(reg1_data[0] as i32).unwrap(), Bit::try_from(reg1_data[1] as i32).unwrap());
                    let register2_address = BinaryDecoder::decode_internal_i32(Bit::try_from(reg2_data[0] as i32).unwrap(), Bit::try_from(reg2_data[1] as i32).unwrap());
                    let mut cpu = self.cpu.clone();
                    let mut register1 = cpu.get_register(register1_address);
                    let register2 = cpu.get_register(register2_address);
                    match stream[0].to_u8_array() {
                        [0, 1, 0, 0, _, _, _, _] => {
                            // AND
                            register1.write(register1.value.and(&register2.value));
                        }
                        [0, 1, 0, 1, _, _, _, _] => {
                            // OR
                            register1.write(register1.value.or(&register2.value));
                        }
                        [0, 1, 1, 0, _, _, _, _] => {
                            // NOT
                            register1.write(register1.value.not());
                        }
                        [0, 1, 1, 1, _, _, _, _] => {
                            // XOR
                            register1.write(register1.value.xor(&register2.value));
                        }
                        _ => {
                            panic!("Invalid boolean logic instruction");
                        }
                    }
                    match register1_address {
                        1 => cpu.reg_1 = register1,
                        2 => cpu.reg_2 = register1,
                        3 => cpu.reg_3 = register1,
                        4 => cpu.reg_4 = register1,
                        _ => panic!("Invalid register address"),
                    }
                    self.cpu = cpu;
                }
                else { // 00 XX XX XX
                    // ALU
                    let op1_a = first_byte[4].to_i32();
                    let op1_b = first_byte[5].to_i32();
                    let op2_a = first_byte[6].to_i32();
                    let op2_b = first_byte[7].to_i32();
                    let register_address1 = BinaryDecoder::decode_internal_i32(Bit::try_from(op1_a).unwrap(), Bit::try_from(op1_b).unwrap());
                    let register_address2 = BinaryDecoder::decode_internal_i32(Bit::try_from(op2_a).unwrap(), Bit::try_from(op2_b).unwrap());
                    let operand_1 = self.cpu.get_register(register_address1).value.to_i32();
                    let operand_2 = self.cpu.get_register(register_address2).value.to_i32();
        
                    self.cpu.alu.value1 = Byte::try_from(operand_1).unwrap();
                    self.cpu.alu.value2 = Byte::try_from(operand_2).unwrap();
                    self.cpu.alu.decoder = BinaryDecoder {axis_x: first_byte[2], axis_y: first_byte[3], result: [Bit::new(false); 4]};
                    self.cpu.alu.compute();
        
                    let mut cpu = self.cpu.clone();
                    let mut register = cpu.get_register(register_address1);
                    register.write(Byte::try_from(self.cpu.alu.value1).unwrap());
                    match register_address1 {
                        1 => cpu.reg_1 = register,
                        2 => cpu.reg_2 = register,
                        3 => cpu.reg_3 = register,
                        4 => cpu.reg_4 = register,
                        _ => panic!("Invalid register address"),
                    }
                    self.cpu = cpu;
                }
            }
            else { // 1X XX XX XX
                if !first_byte[1].value { // 10 XX XX XX
                    // Ports

                    // Ports takes 1 byte, Memory takes 2 bytes
                    // decrement so next instruction won't be skipped
                    self.ram.decrement();

                    let register_address = BinaryDecoder::decode_internal_i32(Bit::try_from(first_byte[3].value as i32).unwrap(), Bit::try_from(first_byte[4].value as i32).unwrap());

                    let port = [first_byte[5].value, first_byte[6].value, first_byte[7].value];
                    let port_address: i32 = match port {
                        [false, false, false] => 0,
                        [false, false, true] => 1,
                        [false, true, false] => 2,
                        [false, true, true] => 3,
                        [true, false, false] => 4,
                        [true, false, true] => 5,
                        [true, true, false] => 6,
                        [true, true, true] => 7,
                    };
                    
                    if !first_byte[2].value { // 10 0X XX XX
                        // read
                        let mut cpu = self.cpu.clone();
                        let mut register = cpu.get_register(register_address);
                        let value = self.ports.read(port_address.try_into().unwrap());
                        register.write(value);
                        match register_address {
                            1 => cpu.reg_1 = register,
                            2 => cpu.reg_2 = register,
                            3 => cpu.reg_3 = register,
                            4 => cpu.reg_4 = register,
                            _ => panic!("Invalid register address"),
                        }
                        self.cpu = cpu;
                    }
                    else { // 10 1X XX XX
                        // write
                        let register = self.cpu.get_register(register_address);
                        self.ports.write(port_address.try_into().unwrap(), register.value);
                    }
                } 
                else { // 11 XX XX XX
                    match stream[0].to_u8_array() {
                        [1, 1, 1, 1, 1, 1, 1, 1] => {
                            // Halt
                
                            // Halting takes 1 byte from memory
                            // decrement so next instruction won't be skipped
                            self.ram.decrement();

                            // stop running
                            halted = true;
                        }
                        [1, 1, 0, 0, 0, 0, _, _] => {
                            // Store
                            let mut reg_data: [u8; 2] = [0; 2];
                            for i in 0..2 {
                                reg_data[i] = first_byte[6 + i].to_i32() as u8;
                            } 
                            let address = stream[1];
                            let register_address = BinaryDecoder::decode_internal_i32(Bit::try_from(reg_data[0] as i32).unwrap(), Bit::try_from(reg_data[1] as i32).unwrap());
                            let data = self.cpu.get_register(register_address);
                            self.ram.write(address, data.value);
                        }
                        [1, 1, 0, 0, 0, 1, _, _] => {
                            // Load
                            let mut reg_data: [u8; 2] = [0; 2];
                            for i in 0..2 {
                                reg_data[i] = first_byte[6 + i].to_i32() as u8;
                            } 
                            let address = stream[1];
                            let register_address = BinaryDecoder::decode_internal_i32(Bit::try_from(reg_data[0] as i32).unwrap(), Bit::try_from(reg_data[1] as i32).unwrap());
                            let mut cpu = self.cpu.clone();
                            let mut register = cpu.get_register(register_address);
                            register.write(self.ram.read(address));
                            match register_address {
                                1 => cpu.reg_1 = register,
                                2 => cpu.reg_2 = register,
                                3 => cpu.reg_3 = register,
                                4 => cpu.reg_4 = register,
                                _ => panic!("Invalid register address"),
                            }
                            self.cpu = cpu;
                        }
                        [1, 1, 0, 0, 1, 0, _, _] => {
                            // Move
                            let mut reg_data: [u8; 2] = [0; 2];
                            for i in 0..2 {
                                reg_data[i] = first_byte[6 + i].to_i32() as u8;
                            } 
                            let binary: Byte = stream[1];
                            let register_address = BinaryDecoder::decode_internal_i32(Bit::try_from(reg_data[0] as i32).unwrap(), Bit::try_from(reg_data[1] as i32).unwrap());
                            let mut cpu = self.cpu.clone();
                            let mut register = cpu.get_register(register_address);
                            register.write(binary);
                            match register_address {
                                1 => cpu.reg_1 = register,
                                2 => cpu.reg_2 = register,
                                3 => cpu.reg_3 = register,
                                4 => cpu.reg_4 = register,
                                _ => panic!("Invalid register address"),
                            }
                            self.cpu = cpu;
                        }
                        [1, 1, 0, 0, 1, 1, _, _] => {
                            // Copy
                            let mut reg1_data: [u8; 2] = [0; 2];
                            for i in 0..2 {
                                reg1_data[i] = first_byte[6 + i].to_i32() as u8;
                            } 
                            let mut reg2_data: [u8; 2] = [0; 2];
                            for i in 0..2 {
                                reg2_data[i] = stream[1].value[i].to_i32() as u8;
                            } 
                            let register1_address = BinaryDecoder::decode_internal_i32(Bit::try_from(reg1_data[0] as i32).unwrap(), Bit::try_from(reg1_data[1] as i32).unwrap());
                            let register2_address = BinaryDecoder::decode_internal_i32(Bit::try_from(reg2_data[0] as i32).unwrap(), Bit::try_from(reg2_data[1] as i32).unwrap());
                            let mut cpu = self.cpu.clone();
                            let mut register1 = cpu.get_register(register1_address);
                            let register2 = cpu.get_register(register2_address);
                            register1.write(register2.value);
                            match register1_address {
                                1 => cpu.reg_1 = register1,
                                2 => cpu.reg_2 = register1,
                                3 => cpu.reg_3 = register1,
                                4 => cpu.reg_4 = register1,
                                _ => panic!("Invalid register address"),
                            }
                            self.cpu = cpu;
                        }
                        [1, 1, 0, 1, 0, 0, _, _] => {
                            // Shift Left
                            let mut reg1_data: [u8; 2] = [0; 2];
                            for i in 0..2 {
                                reg1_data[i] = first_byte[6 + i].to_i32() as u8;
                            } 
                            let binary: Byte = stream[1];
                            let register1_address = BinaryDecoder::decode_internal_i32(Bit::try_from(reg1_data[0] as i32).unwrap(), Bit::try_from(reg1_data[1] as i32).unwrap());
                            let mut cpu = self.cpu.clone();
                            let mut register1 = cpu.get_register(register1_address);
                            register1.write(Byte::shift_array(register1.value, -binary.to_i32()));
                            match register1_address {
                                1 => cpu.reg_1 = register1,
                                2 => cpu.reg_2 = register1,
                                3 => cpu.reg_3 = register1,
                                4 => cpu.reg_4 = register1,
                                _ => panic!("Invalid register address"),
                            }
                            self.cpu = cpu;
                        }
                        [1, 1, 0, 1, 0, 1, _, _] => {
                            // Shift Right
                            let mut reg1_data: [u8; 2] = [0; 2];
                            for i in 0..2 {
                                reg1_data[i] = first_byte[6 + i].to_i32() as u8;
                            } 
                            let binary: Byte = stream[1];
                            let register1_address = BinaryDecoder::decode_internal_i32(Bit::try_from(reg1_data[0] as i32).unwrap(), Bit::try_from(reg1_data[1] as i32).unwrap());
                            let mut cpu = self.cpu.clone();
                            let mut register1 = cpu.get_register(register1_address);
                            register1.write(Byte::shift_array(register1.value, binary.to_i32()));
                            match register1_address {
                                1 => cpu.reg_1 = register1,
                                2 => cpu.reg_2 = register1,
                                3 => cpu.reg_3 = register1,
                                4 => cpu.reg_4 = register1,
                                _ => panic!("Invalid register address"),
                            }
                            self.cpu = cpu;
                        }
                        [1, 1, 0, 1, 1, 0, _, _] => {
                            // Out

                            // only takes 1 byte from memory
                            // decrement so next instruction won't be skipped
                            self.ram.decrement();

                            let mut reg1_data: [u8; 2] = [0; 2];
                            for i in 0..2 {
                                reg1_data[i] = first_byte[6 + i].to_i32() as u8;
                            } 
                            let register_address = BinaryDecoder::decode_internal_i32(Bit::try_from(reg1_data[0] as i32).unwrap(), Bit::try_from(reg1_data[1] as i32).unwrap());
                            let value = self.cpu.get_register(register_address).value;
                            print!("{}", value.to_string());
                        }
                        [1, 1, 0, 1, 1, 1, _, _] => {
                            // Msg

                            // only takes 1 byte from memory
                            // decrement so next instruction won't be skipped
                            self.ram.decrement();

                            let mut reg1_data: [u8; 2] = [0; 2];
                            for i in 0..2 {
                                reg1_data[i] = first_byte[6 + i].to_i32() as u8;
                            } 
                            let register_address = BinaryDecoder::decode_internal_i32(Bit::try_from(reg1_data[0] as i32).unwrap(), Bit::try_from(reg1_data[1] as i32).unwrap());
                            let value = self.cpu.get_register(register_address).value;
                            let c = bool_array_to_ascii(value.to_bool_array());
                            print!("{}", c);
                        }
                        [1, 1, 1, 0, 0, 0, _, _] => {
                            // Increment

                            // only takes 1 byte from memory
                            // decrement so next instruction won't be skipped
                            self.ram.decrement();

                            let mut reg1_data: [u8; 2] = [0; 2];
                            for i in 0..2 {
                                reg1_data[i] = first_byte[6 + i].to_i32() as u8;
                            } 
                            let register_address = BinaryDecoder::decode_internal_i32(Bit::try_from(reg1_data[0] as i32).unwrap(), Bit::try_from(reg1_data[1] as i32).unwrap());
                            let mut cpu = self.cpu.clone();
                            let mut register = cpu.get_register(register_address);
                            register.write((register.value.to_i32() + 1).try_into().unwrap());
                            match register_address {
                                1 => cpu.reg_1 = register,
                                2 => cpu.reg_2 = register,
                                3 => cpu.reg_3 = register,
                                4 => cpu.reg_4 = register,
                                _ => panic!("Invalid register address"),
                            }
                            self.cpu = cpu;
                        }
                        [1, 1, 1, 0, 0, 1, _, _] => {
                            // Decrement

                            // only takes 1 byte from memory
                            // decrement so next instruction won't be skipped
                            self.ram.decrement();

                            let mut reg1_data: [u8; 2] = [0; 2];
                            for i in 0..2 {
                                reg1_data[i] = first_byte[6 + i].to_i32() as u8;
                            } 
                            let register_address = BinaryDecoder::decode_internal_i32(Bit::try_from(reg1_data[0] as i32).unwrap(), Bit::try_from(reg1_data[1] as i32).unwrap());
                            let mut cpu = self.cpu.clone();
                            let mut register = cpu.get_register(register_address);
                            register.write((register.value.to_i32() - 1).try_into().unwrap());
                            match register_address {
                                1 => cpu.reg_1 = register,
                                2 => cpu.reg_2 = register,
                                3 => cpu.reg_3 = register,
                                4 => cpu.reg_4 = register,
                                _ => panic!("Invalid register address"),
                            }
                            self.cpu = cpu;
                        }
                        [1, 1, 1, 0, 1, 0, 0, 0] => {
                            // Jump
                            let binary = stream[1];
                            self.ram.set_index(binary);
                        }
                        [1, 1, 1, 0, 1, 0, 0, 1] => {
                            // Jump Neg
                            let binary = stream[1];
                            if self.cpu.alu.math.neg.to_bool() {
                                self.ram.set_index(binary);
                            }
                        }
                        [1, 1, 1, 0, 1, 0, 1, 0] => {
                            // Jump Zero
                            let binary = stream[1];
                            if self.cpu.alu.math.zero.to_bool() {
                                self.ram.set_index(binary);
                            }
                        }
                        [1, 1, 1, 0, 1, 0, 1, 1] => {
                            // Jump Above
                            let binary = stream[1];
                            if !self.cpu.alu.math.neg.to_bool() && !self.cpu.alu.math.zero.to_bool() {
                                self.ram.set_index(binary);
                            }
                        }
                        [1, 1, 1, 1, 0, 0, _, _] => {
                            // CMP_NEG

                            // only takes 1 byte from memory
                            // decrement so next instruction won't be skipped
                            self.ram.decrement();
                            
                            let mut reg_data: [u8; 2] = [0; 2];
                            for i in 0..2 {
                                reg_data[i] = first_byte[6 + i].to_i32() as u8;
                            }
                            let binary = if self.cpu.alu.math.neg.to_bool() { Byte::full() } else { Byte::zero() };
                            let register_address = BinaryDecoder::decode_internal_i32(Bit::try_from(reg_data[0] as i32).unwrap(), Bit::try_from(reg_data[1] as i32).unwrap());
                            let mut cpu = self.cpu.clone();
                            let mut register = cpu.get_register(register_address);
                            register.write(binary);
                            match register_address {
                                1 => cpu.reg_1 = register,
                                2 => cpu.reg_2 = register,
                                3 => cpu.reg_3 = register,
                                4 => cpu.reg_4 = register,
                                _ => panic!("Invalid register address"),
                            }
                            self.cpu = cpu;
                        }
                        [1, 1, 1, 1, 0, 1, _, _] => {
                            // CMP_ZRO

                            // only takes 1 byte from memory
                            // decrement so next instruction won't be skipped
                            self.ram.decrement();

                            let mut reg_data: [u8; 2] = [0; 2];
                            for i in 0..2 {
                                reg_data[i] = first_byte[6 + i].to_i32() as u8;
                            }
                            let binary = if self.cpu.alu.math.zero.to_bool() { Byte::full() } else { Byte::zero() };
                            let register_address = BinaryDecoder::decode_internal_i32(Bit::try_from(reg_data[0] as i32).unwrap(), Bit::try_from(reg_data[1] as i32).unwrap());
                            let mut cpu = self.cpu.clone();
                            let mut register = cpu.get_register(register_address);
                            register.write(binary);
                            match register_address {
                                1 => cpu.reg_1 = register,
                                2 => cpu.reg_2 = register,
                                3 => cpu.reg_3 = register,
                                4 => cpu.reg_4 = register,
                                _ => panic!("Invalid register address"),
                            }
                            self.cpu = cpu;
                        }
                        [1, 1, 1, 1, 1, 0, _, _] => {
                            // CMP_ABV

                            // only takes 1 byte from memory
                            // decrement so next instruction won't be skipped
                            self.ram.decrement();

                            let mut reg_data: [u8; 2] = [0; 2];
                            for i in 0..2 {
                                reg_data[i] = first_byte[6 + i].to_i32() as u8;
                            }
                            let binary = if !self.cpu.alu.math.neg.to_bool() && !self.cpu.alu.math.zero.to_bool() { Byte::full() } else { Byte::zero() };
                            let register_address = BinaryDecoder::decode_internal_i32(Bit::try_from(reg_data[0] as i32).unwrap(), Bit::try_from(reg_data[1] as i32).unwrap());
                            let mut cpu = self.cpu.clone();
                            let mut register = cpu.get_register(register_address);
                            register.write(binary);
                            match register_address {
                                1 => cpu.reg_1 = register,
                                2 => cpu.reg_2 = register,
                                3 => cpu.reg_3 = register,
                                4 => cpu.reg_4 = register,
                                _ => panic!("Invalid register address"),
                            }
                            self.cpu = cpu;
                        }
                        _ => {
                            panic!("Invalid function {}", stream[0].to_string());
                        }
                    }
                }
            }
            halted
        }
    }

    pub fn bool_array_to_ascii(arr: [bool; 8]) -> char {
        let mut value = 0u8;

        for (i, &bit) in arr.iter().enumerate() {
            if bit {
                value |= 1 << (7 - i); // Set the appropriate bit
            }
        }

        value as char
    }
}