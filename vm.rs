pub enum Value
{
    Int(i64),
    Str(String),
}

// Opcode enumeration
pub enum Instr
{
    // Local variable access
    GetLocal { idx: usize },
    SetLocal { idx: usize },

    // Stack manipulation
    Push { val: Value },
    Pop,
    Dup,

    // 32-bit integer operations
    //I32Add,
    //I32Sub,
    //I32Mul,

    // Branch instructions
    Jump { offset: isize },
    IfTrue { offset: isize },
    IfFalse { offset: isize },
    Call { target: usize },
    Ret,
}

struct Function
{

}

struct VM
{

}
