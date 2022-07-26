use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Value
{
    Int64(i64),
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

    // Arithmetic operations
    Add,
    Sub,
    Mul,

    // Comparisons
    Eq,
    Ne,

    // Branch instructions
    Jump { offset: isize },
    IfTrue { offset: isize },
    IfFalse { offset: isize },
    Call { argc: usize },
    Return,
}

pub struct Function
{
    /// Name of the function
    pub name: String,

    // TODO: arguments list


    /// Number of local variables
    pub num_locals: usize,

    /// Bytecode making up this function
    pub insns: Vec<Insn>,
}

impl Function
{
    pub fn new(name: &str) -> Self
    {
        Self {
            name: name.to_string(),
            num_locals: 0,
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
        use Value::*;

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

                Add => {
                    let v1 = self.stack_pop();
                    let v0 = self.stack_pop();
                    match (v0, v1) {
                        (Int64(v0), Int64(v1)) => self.stack.push(Int64(v0 + v1)),
                        _ => panic!()
                    }
                }

                Sub => {
                    let v1 = self.stack_pop();
                    let v0 = self.stack_pop();
                    match (v0, v1) {
                        (Int64(v0), Int64(v1)) => self.stack.push(Int64(v0 - v1)),
                        _ => panic!()
                    }
                }

                Mul => {
                    let v1 = self.stack_pop();
                    let v0 = self.stack_pop();
                    match (v0, v1) {
                        (Int64(v0), Int64(v1)) => self.stack.push(Int64(v0 * v1)),
                        _ => panic!()
                    }
                }

                Return => {
                    return self.stack_pop();
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
    use Value::*;

    fn eval_src(src: &str) -> Value
    {
        dbg!(src);
        let mut input = Input::new(src, "test_src");
        let unit_fn = parse_unit(&mut input).unwrap();
        let mut vm = VM::new();
        return vm.eval(&unit_fn);
    }

    #[test]
    fn test_eval()
    {
        assert_eq!(eval_src(""), Nil);
        assert_eq!(eval_src("1;"), Nil);
        assert_eq!(eval_src("return 7;"), Int64(7));
        assert_eq!(eval_src("return 1 + 7;"), Int64(8));
        assert_eq!(eval_src("return 1 + 2 + 3;"), Int64(6));

        // Priority of operations
        assert_eq!(eval_src("return 1 + 2 * 3;"), Int64(7));
        assert_eq!(eval_src("return 1 + 2 + 3 + 4;"), Int64(10));
        assert_eq!(eval_src("return 1 * 2 + 3 * 4;"), Int64(14));
        assert_eq!(eval_src("return (1 + 2) * 3;"), Int64(9));
        assert_eq!(eval_src("return (1 * 2) + (3 * 4);"), Int64(14));
        assert_eq!(eval_src("return 1 + 2 * 3 + 4;"), Int64(11));

        // Subtract and operand ordering
        assert_eq!(eval_src("return 5 - 3;"), Int64(2));
        assert_eq!(eval_src("return 5 + 2 - 3;"), Int64(4));
    }
}
