fn main() {
    println!("Hello, world!");
}

const ZERO_FLAG: u8 = 0x02;
const NEGATIVE_FLAG: u8 = 0x80;

const LDA_OPCODE: u8 = 0xA9;
const TAX_OPCODE: u8 = 0xAA;
const INX_OPCODE: u8 = 0xE8;
const BRK_OPCODE: u8 = 0x00;

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

    fn lda(&mut self, value: u8) {
        self.register_a = value;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn update_zero_and_negative_flags(&mut self, result: u8) {
        if result == 0 {
            self.status |= ZERO_FLAG; // Set zero flag
        } else {
            self.status &= 0b1111_1101; // Clear zero flag
        }

        if result & NEGATIVE_FLAG != 0 {
            self.status |= NEGATIVE_FLAG; // Set negative flag
        } else {
            self.status &= 0b0111_1111; // Clear negative flag
        }
    }

    // A very simple interpreter for a subset of 6502 instructions
    pub fn interpret(&mut self, program: Vec<u8>) {
        self.program_counter = 0;

        loop {
            let opscode = program[self.program_counter as usize];
            self.program_counter = self.program_counter.wrapping_add(1);

            match opscode {
                LDA_OPCODE => {
                    // LDA - Load Accumulator Immediate
                    let param = program[self.program_counter as usize];
                    self.program_counter = self.program_counter.wrapping_add(1);

                    self.lda(param);
                }
                TAX_OPCODE => self.tax(), // TAX - Transfer Accumulator to X
                INX_OPCODE => self.inx(), // INX - Increment X Register
                BRK_OPCODE => return,     // BRK - Break
                _ => todo!("Implement opscode: {opscode:#X}"),
            }
        }
    }
}

impl Default for CPU {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0xa9_lda_immediate_load_data() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![LDA_OPCODE, 0x05, BRK_OPCODE]); // LDA #$05; BRK
        assert_eq!(cpu.register_a, 0x05);
        assert!(cpu.status & ZERO_FLAG == 0x00); // Zero flag should be clear
        assert!(cpu.status & NEGATIVE_FLAG == 0); // Negative flag should be clear
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![LDA_OPCODE, BRK_OPCODE, BRK_OPCODE]); // LDA #$00; BRK
        assert!(cpu.status & ZERO_FLAG == ZERO_FLAG); // Zero flag should be set
    }

    #[test]
    fn test_0xa9_lda_negative_flag() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![LDA_OPCODE, NEGATIVE_FLAG, BRK_OPCODE]); // LDA #$80; BRK
        assert!(cpu.status & NEGATIVE_FLAG == NEGATIVE_FLAG); // Negative flag should be set
    }

    #[test]
    fn test_0xaa_tax_transfer_a_to_x() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x0A;
        cpu.interpret(vec![TAX_OPCODE, BRK_OPCODE]); // TAX; BRK
        assert_eq!(cpu.register_x, 0x0A);
    }

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![LDA_OPCODE, 0xc0, TAX_OPCODE, INX_OPCODE, BRK_OPCODE]); // LDA #$C0; TAX; INX; BRK
        assert_eq!(cpu.register_x, 0xc1);
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.register_x = 0xFF;
        cpu.interpret(vec![INX_OPCODE, INX_OPCODE, BRK_OPCODE]); // INX; INX; BRK
        assert_eq!(cpu.register_x, 1);
    }
}
