// https://www.teach.cs.toronto.edu/~ajr/258/pdp11.pdf

use std::fmt::Display;

use crate::aout::Memory;

pub struct Instruction {
    addr: u16,
    opcode: Opcode,
    op1: Option<u16>,
    op2: Option<u16>,
}

impl Instruction {
    pub fn new(addr: usize, mem: &Memory) -> Self {
        let opcode_value = mem.get_word_at(addr);
        let opcode: Opcode = opcode_value.into();
        let op1 = if opcode.pc_relative1() {
            Some(mem.get_word_at(addr + 2))
        } else {
            None
        };
        let op2 = if opcode.pc_relative2() {
            Some(mem.get_word_at(addr + 4))
        } else {
            None
        };

        Self {
            addr: addr as u16,
            opcode,
            op1,
            op2,
        }
    }

    pub fn size(&self) -> usize {
        self.opcode.size()
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.opcode)?;
        if let Some(op1) = self.opcode.get_operand1(self.op1) {
            write!(f, " {op1}")?;
            if let Some(op2) = self.opcode.get_operand2(self.op2) {
                write!(f, ", {op2}")?;
            }
        }

        Ok(())
    }
}

// xxxxxx: oooooo 111111 222222 | MNEMONIC [arg1], [arg2]

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Opcode {
    Invalid,
    ZeroOp(ZeroOp),
    OneOp {
        op: OneOp,
        addr: PdpAddress,
    },
    RegOp {
        op: RegOp,
        reg: Register,
        addr: PdpAddress,
    },
    TwoOp {
        op: TwoOp,
        src: PdpAddress,
        dst: PdpAddress,
    },
    Branch(Branch),
    Jump(Jump),
    Fp(FpOpcode),
}

impl Opcode {
    fn size(&self) -> usize {
        2 + match self {
            Opcode::Invalid => 0,
            Opcode::ZeroOp(zero_op) => 0,
            Opcode::OneOp { op, addr } => addr.is_pc() as usize * 2,
            Opcode::RegOp { op, reg, addr } => addr.is_pc() as usize * 2,
            Opcode::TwoOp { op, src, dst } => (src.is_pc() as usize + dst.is_pc() as usize) * 2,
            Opcode::Branch(branch) => 0,
            Opcode::Jump(jump) => todo!(),
            Opcode::Fp(fp_opcode) => fp_opcode.size() * 2,
        }
    }

    fn get_operand1(&self, op1: Option<u16>) -> Option<String> {
        match self {
            Opcode::Invalid => None,
            Opcode::ZeroOp(zero_op) => None,
            Opcode::OneOp { op, addr } => Some(format!("{}", addr.display(op1))),
            Opcode::RegOp { op, reg, addr } => todo!(),
            Opcode::TwoOp { src, .. } => Some(format!("{}", src.display(op1))),
            Opcode::Branch(branch) => todo!(),
            Opcode::Jump(jump) => todo!(),
            Opcode::Fp(fp_opcode) => None, // TODO
        }
    }

    fn get_operand2(&self, op2: Option<u16>) -> Option<String> {
        match self {
            Opcode::Invalid => todo!(),
            Opcode::ZeroOp(zero_op) => todo!(),
            Opcode::OneOp { op, addr } => todo!(),
            Opcode::RegOp { op, reg, addr } => todo!(),
            Opcode::TwoOp { dst, .. } => Some(format!("{}", dst.display(op2))),
            Opcode::Branch(branch) => todo!(),
            Opcode::Jump(jump) => todo!(),
            Opcode::Fp(fp_opcode) => todo!(),
        }
    }

    fn pc_relative1(&self) -> bool {
        match self {
            Opcode::Invalid => false,
            Opcode::ZeroOp(zero_op) => false,
            Opcode::OneOp { op, addr } => addr.is_pc(),
            Opcode::RegOp { op, reg, addr } => addr.is_pc(),
            Opcode::TwoOp { op, src, dst } => src.is_pc(),
            Opcode::Branch(branch) => false,
            Opcode::Jump(jump) => false,    // TODO
            Opcode::Fp(fp_opcode) => false, // TODO
        }
    }

    fn pc_relative2(&self) -> bool {
        match self {
            Opcode::Invalid => false,
            Opcode::ZeroOp(zero_op) => false,
            Opcode::OneOp { op, addr } => false,
            Opcode::RegOp { op, reg, addr } => false,
            Opcode::TwoOp { op, src, dst } => dst.is_pc(),
            Opcode::Branch(branch) => false,
            Opcode::Jump(jump) => false,
            Opcode::Fp(fp_opcode) => false, // TODO
        }
    }
}

impl Display for Opcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Opcode::Invalid => write!(f, "*INVALID*"),
            Opcode::ZeroOp(zero_op) => todo!(),
            Opcode::OneOp { op, .. } => write!(f, "{op}"),
            Opcode::RegOp { op, .. } => todo!(),
            Opcode::TwoOp { op, .. } => write!(f, "{op}"),
            Opcode::Branch(branch) => todo!(),
            Opcode::Jump(jump) => todo!(),
            Opcode::Fp(fp_opcode) => write!(f, "{fp_opcode}"),
        }
    }
}

impl From<u16> for Opcode {
    fn from(value: u16) -> Self {
        // Zero-operand instructions
        match value {
            0 => return Opcode::ZeroOp(ZeroOp::Halt),
            1 => return Opcode::ZeroOp(ZeroOp::Wait),
            5 => return Opcode::ZeroOp(ZeroOp::Reset),
            0o240 => return Opcode::ZeroOp(ZeroOp::Nop),
            _ => {}
        }

        // One-operand instructions
        let opcode = (value >> 6) & 0b111111111;
        let byte_or_word: Size = (value >> 15).into();
        let addr: PdpAddress = (value & 0b111111).into();
        match opcode {
            0o003 => return OneOp::Swab.make_opcode(addr),
            0o050 => return OneOp::Clr(byte_or_word).make_opcode(addr),
            0o051 => return OneOp::Com(byte_or_word).make_opcode(addr),
            0o052 => return OneOp::Inc(byte_or_word).make_opcode(addr),
            0o053 => return OneOp::Dec(byte_or_word).make_opcode(addr),
            0o054 => return OneOp::Neg(byte_or_word).make_opcode(addr),
            0o055 => return OneOp::Adc(byte_or_word).make_opcode(addr),
            0o056 => return OneOp::Sbc(byte_or_word).make_opcode(addr),
            0o057 => return OneOp::Tst(byte_or_word).make_opcode(addr),
            0o060 => return OneOp::Ror(byte_or_word).make_opcode(addr),
            0o061 => return OneOp::Rol(byte_or_word).make_opcode(addr),
            0o062 => return OneOp::Asr(byte_or_word).make_opcode(addr),
            0o063 => return OneOp::Asl(byte_or_word).make_opcode(addr),
            0o067 => return OneOp::Sxt.make_opcode(addr),
            _ => {}
        }

        // one-and-a-half operand instructions
        let opcode = (value >> 9) & 0b1111111;
        let reg: Register = ((value >> 6) & 0b111).into();
        let addr: PdpAddress = (value & 0b111111).into();
        match opcode {
            0o070 => return RegOp::Mul.make_opcode(reg, addr),
            0o071 => return RegOp::Div.make_opcode(reg, addr),
            0o072 => return RegOp::Ash.make_opcode(reg, addr),
            0o073 => return RegOp::Ashc.make_opcode(reg, addr),
            0o074 => return RegOp::Xor.make_opcode(reg, addr),
            _ => {}
        }

        // two-operand instructions
        let opcode = (value >> 12) & 0b0111;
        let byte_or_word: Size = (value >> 15).into();
        let src: PdpAddress = ((value >> 6) & 0b111111).into();
        let dst: PdpAddress = (value & 0b111111).into();
        match opcode {
            0o1 => return TwoOp::Mov(byte_or_word).make_opcode(src, dst),
            0o2 => return TwoOp::Mov(byte_or_word).make_opcode(src, dst),
            0o3 => return TwoOp::Mov(byte_or_word).make_opcode(src, dst),
            0o4 => return TwoOp::Mov(byte_or_word).make_opcode(src, dst),
            0o5 => return TwoOp::Mov(byte_or_word).make_opcode(src, dst),
            0o6 => {
                if (value >> 15) == 0 {
                    return TwoOp::Add.make_opcode(src, dst);
                } else {
                    return TwoOp::Sub.make_opcode(src, dst);
                }
            }
            _ => {}
        }

        // floating-point instructions
        let top_bits = (value >> 12) & 0b1111;
        if top_bits == 0o17 {
            match value {
                0o170001 => return FpOpcode::SetF.make_opcode(),
                0o170011 => return FpOpcode::SetD.make_opcode(),
                _ => {}
            }
        }

        Opcode::Invalid
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
        self.reg == Register::R7 || self.mode == Mode::Index || self.mode == Mode::IndexIndirect
    }

    fn display(&self, operand: Option<u16>) -> String {
        match self.mode {
            Mode::Register => format!("{}", self.reg),
            Mode::RegisterIndirect => format!("@{}", self.reg),
            Mode::AutoIncrement => format!("({})+", self.reg),
            Mode::AutoIncrementIndirect => format!("@({})+", self.reg),
            Mode::AutoDecrement => format!("-({})", self.reg),
            Mode::AutoDecrementIndirect => format!("@-({})", self.reg),
            Mode::Index => format!("{}({})", operand.unwrap(), self.reg),
            Mode::IndexIndirect => format!("@{}({})", operand.unwrap(), self.reg),
        }
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

impl Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Register::R0 => "R0",
                Register::R1 => "R1",
                Register::R2 => "R2",
                Register::R3 => "R3",
                Register::R4 => "R4",
                Register::R5 => "R5",
                Register::R6 => "SP",
                Register::R7 => "PC",
            }
        )
    }
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
pub enum Size {
    Byte,
    Word,
}

impl Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", if *self == Self::Byte { "B" } else { "" })
    }
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
    fn make_opcode(self, addr: PdpAddress) -> Opcode {
        Opcode::OneOp { op: self, addr }
    }
}

impl Display for OneOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OneOp::Clr(size) => write!(f, "CLR{size}"),
            OneOp::Inc(size) => write!(f, "INC{size}"),
            OneOp::Dec(size) => write!(f, "DEC{size}"),
            OneOp::Adc(size) => write!(f, "ADC{size}"),
            OneOp::Sbc(size) => write!(f, "SBC{size}"),
            OneOp::Tst(size) => write!(f, "TST{size}"),
            OneOp::Neg(size) => write!(f, "NEG{size}"),
            OneOp::Com(size) => write!(f, "COM{size}"),
            OneOp::Ror(size) => write!(f, "ROR{size}"),
            OneOp::Rol(size) => write!(f, "ROL{size}"),
            OneOp::Asr(size) => write!(f, "ASR{size}"),
            OneOp::Asl(size) => write!(f, "ASL{size}"),
            OneOp::Swab => write!(f, "SWAB"),
            OneOp::Sxt => write!(f, "SXT"),
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
    fn make_opcode(self, reg: Register, addr: PdpAddress) -> Opcode {
        Opcode::RegOp {
            op: self,
            reg,
            addr,
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
    fn make_opcode(self, src: PdpAddress, dst: PdpAddress) -> Opcode {
        Opcode::TwoOp { op: self, src, dst }
    }
}

impl Display for TwoOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TwoOp::Mov(size) => write!(f, "MOV{size}"),
            TwoOp::Add => write!(f, "ADD"),
            TwoOp::Sub => write!(f, "SUB"),
            TwoOp::Cmp(size) => write!(f, "CMP{size}"),
            TwoOp::Bis(size) => write!(f, "BIS{size}"),
            TwoOp::Bic(size) => write!(f, "BIC{size}"),
            TwoOp::Bit(size) => write!(f, "BIT{size}"),
        }
    }
}

pub type Offset = i8;

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
pub enum FpOpcode {
    SetD,
    SetF,
}

impl FpOpcode {
    fn make_opcode(self) -> Opcode {
        Opcode::Fp(self)
    }

    fn size(&self) -> usize {
        match self {
            FpOpcode::SetD => 0,
            FpOpcode::SetF => 0,
        }
    }
}

impl Display for FpOpcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                FpOpcode::SetD => "SETD",
                FpOpcode::SetF => "SETF",
            }
        )
    }
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
        let halt: Opcode = 0o000000u16.into();
        assert_eq!(Opcode::ZeroOp(ZeroOp::Halt), halt);

        let wait: Opcode = 0o000001u16.into();
        assert_eq!(Opcode::ZeroOp(ZeroOp::Wait), wait);

        let reset: Opcode = 0o000005u16.into();
        assert_eq!(Opcode::ZeroOp(ZeroOp::Reset), reset);

        let nop: Opcode = 0o000240u16.into();
        assert_eq!(Opcode::ZeroOp(ZeroOp::Nop), nop);
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
        let inst: Opcode = 0o005023.into();
        assert_eq!(
            Opcode::OneOp {
                op: OneOp::Clr(Size::Word),
                addr: PdpAddress {
                    mode: Mode::AutoIncrement,
                    reg: Register::R3
                },
            },
            inst
        );

        let inst: Opcode = 0o105023.into();
        assert_eq!(
            Opcode::OneOp {
                op: OneOp::Clr(Size::Byte),
                addr: PdpAddress {
                    mode: Mode::AutoIncrement,
                    reg: Register::R3
                },
            },
            inst
        );
    }

    #[test]
    fn test_decode_mul() {
        let inst: Opcode = 0o070711.into();
        assert_eq!(
            Opcode::RegOp {
                op: RegOp::Mul,
                reg: Register::R7,
                addr: PdpAddress {
                    mode: Mode::RegisterIndirect,
                    reg: Register::R1,
                },
            },
            inst
        );
    }

    #[test]
    fn test_sub_decode() {
        let inst: Opcode = 0o016600.into();
        assert_eq!(
            Opcode::TwoOp {
                op: TwoOp::Mov(Size::Word),
                src: PdpAddress {
                    mode: Mode::Index,
                    reg: Register::R6,
                },
                dst: PdpAddress {
                    mode: Mode::Register,
                    reg: Register::R0
                }
            },
            inst
        );
    }
}
