// https://www.teach.cs.toronto.edu/~ajr/258/pdp11.pdf

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
    },
    Branch(Branch),
    Jump(Jump),
}

impl From<u16> for Instruction {
    fn from(value: u16) -> Self {
        match value {
            0 => Instruction::ZeroOp(ZeroOp::Halt),
            1 => Instruction::ZeroOp(ZeroOp::Wait),
            5 => Instruction::ZeroOp(ZeroOp::Reset),
            0o240 => Instruction::ZeroOp(ZeroOp::Nop),
            _ => todo!(),
        }
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

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Operand(u16);

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Size {
    Byte,
    Word,
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

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum RegOp {
    Mul,
    Div,
    Ash,
    Ashc,
    Xor,
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
    AutoDecrentIndirect,
    Index,         // if R7 = relative direct
    IndexIndirect, // if R7 = relative indirect
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
}
