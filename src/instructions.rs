// https://www.teach.cs.toronto.edu/~ajr/258/pdp11.pdf

use std::fmt::Display;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Instruction {
    ZeroOp(ZeroOp),
    OneOp {
        op: OneOp,
        addr: PdpAddress,
        operand: Operand,
    },
    RegOp {
        op: RegOp,
        reg: Register,
        addr: PdpAddress,
        operand: Operand,
    },
    TwoOp {
        op: TwoOp,
        src: PdpAddress,
        dst: PdpAddress,
        src_operand: Operand,
        dst_operand: Operand,
    },
    Branch(Branch),
    Jump(Jump),
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::ZeroOp(zero_op) => todo!(),
            Instruction::OneOp { op, addr, operand } => todo!(),
            Instruction::RegOp {
                op,
                reg,
                addr,
                operand,
            } => todo!(),
            Instruction::TwoOp {
                op,
                src,
                dst,
                src_operand,
                dst_operand,
            } => todo!(),
            Instruction::Branch(branch) => todo!(),
            Instruction::Jump(jump) => todo!(),
        }
    }
}

impl From<u16> for Instruction {
    fn from(value: u16) -> Self {
        // Zero-operand instructions
        match value {
            0 => return Instruction::ZeroOp(ZeroOp::Halt),
            1 => return Instruction::ZeroOp(ZeroOp::Wait),
            5 => return Instruction::ZeroOp(ZeroOp::Reset),
            0o240 => return Instruction::ZeroOp(ZeroOp::Nop),
            _ => {}
        }

        // One-operand instructions
        let opcode = (value >> 6) & 0b111111111;
        let byte_or_word: Size = (value >> 15).into();
        let addr: PdpAddress = (value & 0b111111).into();
        match opcode {
            0o003 => return OneOp::Swab.make_instruction(addr),
            0o050 => return OneOp::Clr(byte_or_word).make_instruction(addr),
            0o051 => return OneOp::Com(byte_or_word).make_instruction(addr),
            0o052 => return OneOp::Inc(byte_or_word).make_instruction(addr),
            0o053 => return OneOp::Dec(byte_or_word).make_instruction(addr),
            0o054 => return OneOp::Neg(byte_or_word).make_instruction(addr),
            0o055 => return OneOp::Adc(byte_or_word).make_instruction(addr),
            0o056 => return OneOp::Sbc(byte_or_word).make_instruction(addr),
            0o057 => return OneOp::Tst(byte_or_word).make_instruction(addr),
            0o060 => return OneOp::Ror(byte_or_word).make_instruction(addr),
            0o061 => return OneOp::Rol(byte_or_word).make_instruction(addr),
            0o062 => return OneOp::Asr(byte_or_word).make_instruction(addr),
            0o063 => return OneOp::Asl(byte_or_word).make_instruction(addr),
            0o067 => return OneOp::Sxt.make_instruction(addr),
            _ => {}
        }

        // one-and-a-half operand instructions
        let opcode = (value >> 9) & 0b1111111;
        let reg: Register = ((value >> 6) & 0b111).into();
        let addr: PdpAddress = (value & 0b111111).into();
        match opcode {
            0o070 => return RegOp::Mul.make_instruction(reg, addr),
            0o071 => return RegOp::Div.make_instruction(reg, addr),
            0o072 => return RegOp::Ash.make_instruction(reg, addr),
            0o073 => return RegOp::Ashc.make_instruction(reg, addr),
            0o074 => return RegOp::Xor.make_instruction(reg, addr),
            _ => {}
        }

        // two-operand instructions
        let opcode = (value >> 12) & 0b0111;
        let byte_or_word: Size = (value >> 15).into();
        let src: PdpAddress = ((value >> 6) & 0b111111).into();
        let dst: PdpAddress = (value & 0b111111).into();
        match opcode {
            0o1 => return TwoOp::Mov(byte_or_word).make_instruction(src, dst),
            0o2 => return TwoOp::Mov(byte_or_word).make_instruction(src, dst),
            0o3 => return TwoOp::Mov(byte_or_word).make_instruction(src, dst),
            0o4 => return TwoOp::Mov(byte_or_word).make_instruction(src, dst),
            0o5 => return TwoOp::Mov(byte_or_word).make_instruction(src, dst),
            0o6 => {
                if (value >> 15) == 0 {
                    return TwoOp::Add.make_instruction(src, dst);
                } else {
                    return TwoOp::Sub.make_instruction(src, dst);
                }
            }
            _ => {}
        }

        Instruction::ZeroOp(ZeroOp::Halt)
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum ZeroOp {
    Halt,
    Wait,
    Reset,
    Nop,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct PdpAddress {
    mode: Mode,
    reg: Register,
}
impl PdpAddress {
    fn is_pc(&self) -> bool {
        self.reg == Register::R7
    }
}

impl From<u16> for PdpAddress {
    fn from(value: u16) -> Self {
        let reg = value & 0b111;
        let mode = (value >> 3) & 0b111;
        Self {
            mode: mode.into(),
            reg: reg.into(),
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Register {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
}

impl From<u16> for Register {
    fn from(value: u16) -> Self {
        match value {
            0 => Self::R0,
            1 => Self::R1,
            2 => Self::R2,
            3 => Self::R3,
            4 => Self::R4,
            5 => Self::R5,
            6 => Self::R6,
            7 => Self::R7,
            _ => panic!("invalid register {value}"),
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Operand {
    None,
    NextWord,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Size {
    Byte,
    Word,
}

impl From<u16> for Size {
    fn from(value: u16) -> Self {
        match value {
            0 => Self::Word,
            1 => Self::Byte,
            _ => panic!("not a size value {value}"),
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum OneOp {
    Clr(Size),
    Inc(Size),
    Dec(Size),
    Adc(Size),
    Sbc(Size),
    Tst(Size),
    Neg(Size),
    Com(Size),
    Ror(Size),
    Rol(Size),
    Asr(Size),
    Asl(Size),
    Swab,
    Sxt,
}

impl OneOp {
    fn make_instruction(self, addr: PdpAddress) -> Instruction {
        Instruction::OneOp {
            op: self,
            addr,
            operand: if addr.is_pc() {
                Operand::NextWord
            } else {
                Operand::None
            },
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum RegOp {
    Mul,
    Div,
    Ash,
    Ashc,
    Xor,
}

impl RegOp {
    fn make_instruction(self, reg: Register, addr: PdpAddress) -> Instruction {
        Instruction::RegOp {
            op: self,
            reg,
            addr,
            operand: if addr.is_pc() {
                Operand::NextWord
            } else {
                Operand::None
            },
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum TwoOp {
    Mov(Size),
    Add,
    Sub,
    Cmp(Size),
    Bis(Size),
    Bic(Size),
    Bit(Size),
}

impl TwoOp {
    fn make_instruction(self, src: PdpAddress, dst: PdpAddress) -> Instruction {
        Instruction::TwoOp {
            op: self,
            src,
            dst,
            src_operand: if src.is_pc() {
                Operand::NextWord
            } else {
                Operand::None
            },
            dst_operand: if dst.is_pc() {
                Operand::NextWord
            } else {
                Operand::None
            },
        }
    }
}

pub type Offset = u8;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Branch {
    Br(Offset),
    Bne(Offset),
    Beq(Offset),
    Bpl(Offset),
    Bmi(Offset),
    Bvc(Offset),
    Bvs(Offset),
    Bhis(Offset),
    Bcc(Offset),
    Blo(Offset),
    Bcs(Offset),
    Bge(Offset),
    Blt(Offset),
    Bgt(Offset),
    Ble(Offset),
    Bhi(Offset),
    Blos(Offset),
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Jump {
    Jump(PdpAddress),
    Sob(Register, Offset),
    Jsr(Register, PdpAddress),
    Rts(Register),
    Rti,
    Trap(u8),
    Bpt,
    Iot,
    Emt(u8),
    Rtt,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Mode {
    Register,
    RegisterIndirect,
    AutoIncrement,         // if R7 = Immediate
    AutoIncrementIndirect, // if R7 = aboslute indirect
    AutoDecrement,
    AutoDecrementIndirect,
    Index,         // if R7 = relative direct
    IndexIndirect, // if R7 = relative indirect
}

impl From<u16> for Mode {
    fn from(value: u16) -> Self {
        match value {
            0 => Self::Register,
            1 => Self::RegisterIndirect,
            2 => Self::AutoIncrement,
            3 => Self::AutoIncrementIndirect,
            4 => Self::AutoDecrement,
            5 => Self::AutoDecrementIndirect,
            6 => Self::Index,
            7 => Self::IndexIndirect,
            _ => panic!("invalid register {value}"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_zero_op() {
        let halt: Instruction = 0o000000u16.into();
        assert_eq!(Instruction::ZeroOp(ZeroOp::Halt), halt);

        let wait: Instruction = 0o000001u16.into();
        assert_eq!(Instruction::ZeroOp(ZeroOp::Wait), wait);

        let reset: Instruction = 0o000005u16.into();
        assert_eq!(Instruction::ZeroOp(ZeroOp::Reset), reset);

        let nop: Instruction = 0o000240u16.into();
        assert_eq!(Instruction::ZeroOp(ZeroOp::Nop), nop);
    }

    #[test]
    fn test_address_decode() {
        let addr: PdpAddress = 0u16.into();
        assert_eq!(
            PdpAddress {
                mode: Mode::Register,
                reg: Register::R0
            },
            addr
        );

        let addr: PdpAddress = 0b101100u16.into();
        assert_eq!(
            PdpAddress {
                mode: Mode::AutoDecrementIndirect,
                reg: Register::R4
            },
            addr
        );
    }

    #[test]
    fn test_decode_clr() {
        let inst: Instruction = 0o005023.into();
        assert_eq!(
            Instruction::OneOp {
                op: OneOp::Clr(Size::Word),
                addr: PdpAddress {
                    mode: Mode::AutoIncrement,
                    reg: Register::R3
                },
                operand: Operand::None
            },
            inst
        );

        let inst: Instruction = 0o105023.into();
        assert_eq!(
            Instruction::OneOp {
                op: OneOp::Clr(Size::Byte),
                addr: PdpAddress {
                    mode: Mode::AutoIncrement,
                    reg: Register::R3
                },
                operand: Operand::None
            },
            inst
        );
    }

    #[test]
    fn test_decode_mul() {
        let inst: Instruction = 0o070711.into();
        assert_eq!(
            Instruction::RegOp {
                op: RegOp::Mul,
                reg: Register::R7,
                addr: PdpAddress {
                    mode: Mode::RegisterIndirect,
                    reg: Register::R1,
                },
                operand: Operand::None,
            },
            inst
        );
    }
}
