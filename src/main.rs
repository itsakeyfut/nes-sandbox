fn main() {
    println!("Hello, world!");
}

// A simple CPU struct to represent the state of a 6502-like CPU
pub struct CPU {
    pub register_a: u8,
    pub status: u8,
    pub program_counter: u16,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            status: 0,
            program_counter: 0,
        }
    }

    // A very simple interpreter for a subset of 6502 instructions
    pub fn interpret(&mut self, program: Vec<u8>) {
        self.program_counter = 0;

        loop {
            let opscode = program[self.program_counter as usize];
            self.program_counter += 1;

            match opscode {
                0xA9 => { // LDA - Load Accumulator Immediate
                    let param = program[self.program_counter as usize];
                    self.program_counter += 1;
                    self.register_a = param;

                    if self.register_a == 0 {
                        self.status |= 0b0000_0010; // Set zero flag
                    } else {
                        self.status &= 0b1111_1101; // Clear zero flag
                    }

                    if self.register_a & 0b1000_0000 != 0 {
                        self.status |= 0b1000_0000; // Set negative flag
                    } else {
                        self.status &= 0b0111_1111; // Clear negative flag
                    }
                }
                0x00 => return, // BRK - Break
                _ => todo!("Implement opscode: {opscode:#X}"),
            }
        }
    }
}