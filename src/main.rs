fn main() {
    println!("Hello, world!");
}

// A simple CPU struct to represent the state of a 6502-like CPU
pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub status: u8,
    pub program_counter: u16,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
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
                0xAA => { // TAX - Transfer Accumulator to X
                    self.register_x = self.register_a;
                    
                    if self.register_x == 0 {
                        self.status |= 0b0000_0010; // Set zero flag
                    } else {
                        self.status &= 0b1111_1101; // Clear zero flag
                    }

                    if self.register_x & 0b1000_0000 != 0 {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0xa9_lda_immediate_load_data() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0x05, 0x00]); // LDA #$05; BRK
        assert_eq!(cpu.register_a, 0x05);
        assert!(cpu.status & 0b0000_0010 == 0b00); // Zero flag should be clear
        assert!(cpu.status & 0b1000_0000 == 0); // Negative flag should be clear
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0x00, 0x00]); // LDA #$00; BRK
        assert!(cpu.status & 0b0000_0010 == 0b10); // Zero flag should be set
    }

    #[test]
    fn test_0xa9_lda_negative_flag() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0x80, 0x00]); // LDA #$80; BRK
        assert!(cpu.status & 0b0100_0000 == 0); // Negative flag should be set
    }

    #[test]
    fn test_0xaa_tax_transfer_a_to_x() {
        let mut cpu = CPU::new();
        cpu.register_a = 10;
        cpu.interpret(vec![0xaa, 0x00]); // TAX; BRK
        assert_eq!(cpu.register_x, 10)
    }
}