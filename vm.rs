use std::collections::HashMap;

pub enum Value
{
    Int(i64),
    Str(String),
}

// Opcode enumeration
pub enum Insn
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

pub struct Function
{
    pub name: String,

    /// Map of local variables to indices in the stack frame
    pub locals: HashMap<String, usize>,



    pub insns: Vec<Insn>,


}

impl Function
{
    fn new() -> Self
    {
        todo!();
    }
}

pub struct VM
{
    stack: Vec<Value>,

    //pc


}

impl VM
{
}
