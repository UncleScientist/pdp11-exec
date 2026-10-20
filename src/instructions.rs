// https://www.teach.cs.toronto.edu/~ajr/258/pdp11.pdf
pub enum Instruction {
    // zero operand
    Halt,
    Wait,
    Reset,
    Nop,

    OneOp(OneOp, Mode, Register, Operand),
    // RegOp(RegOp),
    // TwoOp(TwoOp),
    // Branch(Branch),
}

pub struct Register;
pub struct Operand;

pub enum OneOp {
    Clr,
    Inc,
    Dec,
    Adc,
    Sbc,
    Tst,
    Neg,
    Com,
    Ror,
    Rol,
    Asr,
    Asl,
    Swab,
    Sxt,
}

pub enum InstSize {
    Byte,
    Word,
}

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
