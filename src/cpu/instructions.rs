pub enum Instruction {
    /// *00E0 - CLS*
    ///
    /// Clear the display.
    OpCode00E0,

    /// *00EE - RET*
    ///
    /// Return from a subroutine.
    ///
    /// The interpreter sets the program counter to the address at the top of the stack, then subtracts 1 from the stack pointer.
    OpCode00EE,

    /// *1nnn - JP addr*
    ///
    /// Jump to location nnn.
    ///
    /// The interpreter sets the program counter to nnn.
    OpCode1NNN(u16),

    /// *2nnn - CALL addr*
    ///
    /// Call subroutine at nnn.
    ///
    /// The interpreter increments the stack pointer, then puts the current PC on the top of the stack. The PC is then set to nnn.
    OpCode2NNN(u16),

    /// *3xnn - SE Vx, byte*
    ///
    /// Skip next instruction if Vx = nn.
    ///
    /// The interpreter compares register Vx to nn, and if they are equal, increments the program counter by 2.
    OpCode3XNN(usize, u8),

    /// *4xnn - SNE Vx, byte*
    ///
    /// Skip next instruction if Vx != nn.
    ///
    /// The interpreter compares register Vx to nn, and if they are not equal, increments the program counter by 2.
    OpCode4XNN(usize, u8),

    /// *5xy0 - SE Vx, Vy*
    ///
    /// Skip next instruction if Vx = Vy.
    ///
    /// The interpreter compares register Vx to register Vy, and if they are equal, increments the program counter by 2.
    OpCode5XY0(usize, usize),

    /// *6xnn - LD Vx, byte*
    ///
    /// Set Vx = nn.
    ///
    /// The interpreter puts the value nn into register Vx.
    OpCode6XNN(usize, u8),

    /// *7xnn - ADD Vx, byte*
    ///
    /// Set Vx = Vx + nn.
    ///
    /// Adds the value nn to the value of register Vx, then stores the result in Vx.
    OpCode7XNN(usize, u8),

    /// *8xy0 - LD Vx, Vy*
    ///
    /// Set Vx = Vy.
    ///
    /// Stores the value of register Vy in register Vx.
    OpCode8XY0(usize, usize),

    /// *8xy1 - OR Vx, Vy*
    ///
    /// Set Vx = Vx OR Vy.
    ///
    /// Performs a bitwise OR on the values of Vx and Vy, then stores the result in Vx. A bitwise OR compares the corrseponding bits from two values, and if either bit is 1, then the same bit in the result is also 1. Otherwise, it is 0.
    OpCode8XY1(usize, usize),

    /// *8xy2 - AND Vx, Vy*
    ///
    /// Set Vx = Vx AND Vy.
    ///
    /// Performs a bitwise AND on the values of Vx and Vy, then stores the result in Vx. A bitwise AND compares the corrseponding bits from two values, and if both bits are 1, then the same bit in the result is also 1. Otherwise, it is 0.
    OpCode8XY2(usize, usize),

    /// *8xy3 - XOR Vx, Vy*
    ///
    /// Set Vx = Vx XOR Vy.
    ///
    /// Performs a bitwise exclusive OR on the values of Vx and Vy, then stores the result in Vx. An exclusive OR compares the corrseponding bits from two values, and if the bits are not both the same, then the corresponding bit in the result is set to 1. Otherwise, it is 0.
    OpCode8XY3(usize, usize),

    /// *8xy4 - ADD Vx, Vy*
    ///
    /// Set Vx = Vx + Vy, set VF = carry.
    ///
    /// The values of Vx and Vy are added together. If the result is greater than 8 bits (i.e., > 255,) VF is set to 1, otherwise 0. Only the lowest 8 bits of the result are kept, and stored in Vx.
    OpCode8XY4(usize, usize),

    /// *8xy5 - SUB Vx, Vy*
    ///
    /// Set Vx = Vx - Vy, set VF = NOT borrow.
    ///
    /// If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from Vx, and the results stored in Vx.
    OpCode8XY5(usize, usize),

    /// *8xy6 - SHR Vx {, Vy}*
    ///
    /// Set Vx = Vx SHR 1.
    ///
    /// If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided by 2.
    OpCode8XY6(usize, usize),

    /// *8xy7 - SUBN Vx, Vy*
    ///
    /// Set Vx = Vy - Vx, set VF = NOT borrow.
    ///
    /// If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy, and the results stored in Vx.
    OpCode8XY7(usize, usize),

    /// *8xyE - SHL Vx {, Vy}*
    ///
    /// Set Vx = Vx SHL 1.
    ///
    /// If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx is multiplied by 2.
    OpCode8XYE(usize, usize),

    /// *9xy0 - SNE Vx, Vy*
    ///
    /// Skip next instruction if Vx != Vy.
    ///
    /// The values of Vx and Vy are compared, and if they are not equal, the program counter is increased by 2.
    OpCode9XY0(usize, usize),

    /// *Annn - LD I, addr*
    ///
    /// Set I = nnn.
    ///
    /// The value of register I is set to nnn.
    OpCodeANNN(u16),

    /// *Bnnn - JP V0, addr*
    ///
    /// Jump to location nnn + V0.
    ///
    /// The program counter is set to nnn plus the value of V0.
    OpCodeBNNN(u16),

    /// *Cxkk - RND Vx, byte*
    ///
    /// Set Vx = random byte AND nn.
    ///
    /// The interpreter generates a random number from 0 to 255, which is then ANDed with the value nn. The results are stored in Vx. See instruction 8xy2 for more information on AND.
    OpCodeCXNN(usize, u8),

    /// *Dxyn - DRW Vx, Vy, nibble*
    ///
    /// Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
    ///
    /// The interpreter reads n bytes from memory, starting at the address stored in I. These bytes are then displayed as sprites on screen at coordinates (Vx, Vy). Sprites are XORed onto the existing screen. If this causes any pixels to be erased, VF is set to 1, otherwise it is set to 0. If the sprite is positioned so part of it is outside the coordinates of the display, it wraps around to the opposite side of the screen. See instruction 8xy3 for more information on XOR, and section 2.4, Display, for more information on the Chip-8 screen and sprites.
    OpCodeDXYN(usize, usize, u8),

    /// *Ex9E - SKP Vx*
    ///
    /// Skip next instruction if key with the value of Vx is pressed.
    ///
    /// Checks the keyboard, and if the key corresponding to the value of Vx is currently in the down position, PC is increased by 2.
    OpCodeEX9E(usize),

    /// *ExA1 - SKNP Vx*
    ///
    /// Skip next instruction if key with the value of Vx is not pressed.
    ///
    /// Checks the keyboard, and if the key corresponding to the value of Vx is currently in the up position, PC is increased by 2.
    OpCodeEXA1(usize),

    /// *Fx07 - LD Vx, DT*
    ///
    /// Set Vx = delay timer value.
    ///
    ///The value of DT is placed into Vx.
    OpCodeFX07(usize),

    /// *Fx0A - LD Vx, K*
    ///
    /// Wait for a key press, store the value of the key in Vx.
    ///
    /// All execution stops until a key is pressed, then the value of that key is stored in Vx.
    OpCodeFX0A(usize),

    /// *Fx15 - LD DT, Vx*
    ///
    /// Set delay timer = Vx.
    ///
    /// DT is set equal to the value of Vx.
    OpCodeFX15(usize),

    /// *Fx18 - LD ST, Vx*
    ///
    /// Set sound timer = Vx.
    ///
    /// ST is set equal to the value of Vx.
    OpCodeFX18(usize),

    /// *Fx1E - ADD I, Vx*
    ///
    /// Set I = I + Vx.
    ///
    /// The values of I and Vx are added, and the results are stored in I.
    OpCodeFX1E(usize),

    /// *Fx29 - LD F, Vx*
    ///
    /// Set I = location of sprite for digit Vx.
    ///
    /// The value of I is set to the location for the hexadecimal sprite corresponding to the value of Vx. See section 2.4, Display, for more information on the Chip-8 hexadecimal font.
    OpCodeFX29(usize),

    /// *Fx33 - LD B, Vx*
    ///
    /// Store BCD representation of Vx in memory locations I, I+1, and I+2.
    ///
    /// The interpreter takes the decimal value of Vx, and places the hundreds digit in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2.
    OpCodeFX33(usize),

    /// *Fx55 - LD [I], Vx*
    ///
    /// Store registers V0 through Vx in memory starting at location I.
    ///
    /// The interpreter copies the values of registers V0 through Vx into memory, starting at the address in I.
    OpCodeFX55(usize),

    /// *Fx65 - LD Vx, [I]*
    ///
    /// Read registers V0 through Vx from memory starting at location I.
    ///
    /// The interpreter reads values from memory starting at location I into registers V0 through Vx.
    OpCodeFX65(usize),
}

impl From<u16> for Instruction {
    fn from(opcode: u16) -> Self {
        let op_type = ((opcode & 0xf000) >> 12) as usize;
        let x = ((opcode & 0x0f00) >> 8) as usize;
        let y = ((opcode & 0x00f0) >> 4) as usize;
        let nnn = opcode & 0x0fff;
        let nn = (opcode & 0x00ff) as u8;
        let n = (opcode & 0x000f) as u8;

        match (op_type, x, y, n) {
            (0x0, 0x0, 0xe, 0x0) => Instruction::OpCode00E0,
            (0x0, 0x0, 0xe, 0xe) => Instruction::OpCode00EE,
            (0x1, _, _, _) => Instruction::OpCode1NNN(nnn),
            (0x2, _, _, _) => Instruction::OpCode2NNN(nnn),
            (0x3, _, _, _) => Instruction::OpCode3XNN(x, nn),
            (0x4, _, _, _) => Instruction::OpCode4XNN(x, nn),
            (0x5, _, _, 0x0) => Instruction::OpCode5XY0(x, y),
            (0x6, _, _, _) => Instruction::OpCode6XNN(x, nn),
            (0x7, _, _, _) => Instruction::OpCode7XNN(x, nn),
            (0x8, _, _, 0x0) => Instruction::OpCode8XY0(x, y),
            (0x8, _, _, 0x1) => Instruction::OpCode8XY1(x, y),
            (0x8, _, _, 0x2) => Instruction::OpCode8XY2(x, y),
            (0x8, _, _, 0x3) => Instruction::OpCode8XY3(x, y),
            (0x8, _, _, 0x4) => Instruction::OpCode8XY4(x, y),
            (0x8, _, _, 0x5) => Instruction::OpCode8XY5(x, y),
            (0x8, _, _, 0x6) => Instruction::OpCode8XY6(x, y),
            (0x8, _, _, 0x7) => Instruction::OpCode8XY7(x, y),
            (0x8, _, _, 0xe) => Instruction::OpCode8XYE(x, y),
            (0x9, _, _, 0x0) => Instruction::OpCode9XY0(x, y),
            (0xa, _, _, _) => Instruction::OpCodeANNN(nnn),
            (0xb, _, _, _) => Instruction::OpCodeBNNN(nnn),
            (0xc, _, _, _) => Instruction::OpCodeCXNN(x, nn),
            (0xd, _, _, _) => Instruction::OpCodeDXYN(x, y, n),
            (0xe, _, 0x9, 0xe) => Instruction::OpCodeEX9E(x),
            (0xe, _, 0xa, 0x1) => Instruction::OpCodeEXA1(x),
            (0xf, _, 0x0, 0x7) => Instruction::OpCodeFX07(x),
            (0xf, _, 0x0, 0xa) => Instruction::OpCodeFX0A(x),
            (0xf, _, 0x1, 0x5) => Instruction::OpCodeFX15(x),
            (0xf, _, 0x1, 0x8) => Instruction::OpCodeFX18(x),
            (0xf, _, 0x1, 0xe) => Instruction::OpCodeFX1E(x),
            (0xf, _, 0x2, 0x9) => Instruction::OpCodeFX29(x),
            (0xf, _, 0x3, 0x3) => Instruction::OpCodeFX33(x),
            (0xf, _, 0x5, 0x5) => Instruction::OpCodeFX55(x),
            (0xf, _, 0x6, 0x5) => Instruction::OpCodeFX65(x),
            _ => panic!("Invalid opcode: {:x}", opcode),
        }
    }
}
