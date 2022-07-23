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

    // 64-bit integer operations
    AddI64,
    SubI64,
    MulI64,

    // Branch instructions
    Jump { offset: isize },
    IfTrue { offset: isize },
    IfFalse { offset: isize },
    Call { target: usize },
    Ret,
}

pub struct Function
{
    /// Name of the function
    pub name: String,

    /// Map of local variables to indices in the stack frame
    pub locals: HashMap<String, usize>,

    /// Bytecode making up this function
    pub insns: Vec<Insn>,


}

impl Function
{
    pub fn new(name: &str) -> Self
    {
        Self {
            name: name.to_string(),
            locals: HashMap::default(),
            insns: Vec::default(),
        }
    }
}

pub struct VM
{
    stack: Vec<Value>,

    //pc


}

impl VM
{
    pub fn eval()
    {

    }
}
