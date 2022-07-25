use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Value
{
    Int(i64),
    Str(String),
    Nil,
}

// Opcode enumeration
#[derive(Debug, Clone)]
pub enum Insn
{
    Panic,
    Halt,

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
    Call,
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

    pc: *const Insn,
}

impl VM
{
    pub fn new() -> Self
    {
        Self {
            stack: Vec::default(),
            pc: 0 as *const Insn,
        }
    }

    pub fn stack_size(&self) -> usize
    {
        self.stack.len()
    }

    pub fn stack_pop(&mut self) -> Value
    {
        self.stack.pop().expect("stack empty")
    }

    pub fn eval(&mut self, unit: &Function) -> Value
    {
        use Insn::*;

        self.pc = &unit.insns[0] as *const Insn;

        loop
        {
            //let insn = *self.pc as &Insn;

            let insn = unsafe { &*self.pc };

            match insn {
                Halt => return Value::Nil,

                Push { val } => {
                    self.stack.push(val.clone());
                }

                Pop => {
                    self.stack.pop();
                }

                Ret => {
                    return self.stack.pop().unwrap();
                }





                _ => panic!("unknown instruction in eval: {:?}", insn)
            }

            // Increment the PC
            self.pc = unsafe { self.pc.add(1) };
        }
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    use crate::parser::*;

    fn eval_src(src: &str) -> Value
    {
        let mut input = Input::new("1;", "test_src");
        let unit_fn = parse_unit(&mut input).unwrap();
        let mut vm = VM::new();
        return vm.eval(&unit_fn);
    }

    #[test]
    fn test_eval()
    {
        assert_eq!(eval_src(""), Value::Nil);
        assert_eq!(eval_src(";"), Value::Nil);
        assert_eq!(eval_src("1;"), Value::Nil);




    }
}
