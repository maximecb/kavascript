use std::collections::HashMap;
use crate::runtime::HostFn;

/// Dynamically typed value
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Value
{
    Int64(i64),
    UInt64(u64),
    HostFn(HostFn),
    Fun(*mut Function),
    Str(*mut String),
    Nil,
}

/// Instruction opcode types
#[derive(Debug, Copy, Clone)]
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
    Mod,

    // Unary negation
    Neg,

    // Comparisons
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,

    // Unary logical not
    Not,

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

    /// Parameter list
    pub params: Vec<String>,

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
            params: Vec::default(),
            num_locals: 0,
            insns: Vec::default(),
        }
    }
}

/// Hold an object to be placed in the GC heap and mark bits
#[repr(C)]
pub struct HeapObject<T>
{
    /// Mark bits/boolean
    mark: usize,

    /// Object stored on the heap
    object: T
}

pub enum GCObject
{
    Fun(Box<HeapObject<Function>>),
    Str(Box<HeapObject<String>>),
}

impl GCObject
{
    /// Get a GC heap pointer for this object wrapped
    /// into a dynamically-typed value
    fn get_ptr_value(&mut self) -> Value
    {
        match self {
            Self::Fun(gc_box) => Value::Fun(&mut (gc_box.object) as *mut Function),
            Self::Str(gc_box) => Value::Str(&mut (gc_box.object) as *mut String),
        }
    }

    fn clear_mark(&mut self)
    {
        match self {
            Self::Fun(gc_box) => gc_box.mark = 0,
            Self::Str(gc_box) => gc_box.mark = 0,
        }
    }

    fn is_marked(&self) -> bool
    {
        match self {
            Self::Fun(gc_box) => gc_box.mark != 0,
            Self::Str(gc_box) => gc_box.mark != 0,
        }
    }
}

impl From<Function> for GCObject {
    fn from(fun: Function) -> GCObject {
        let heap_obj = HeapObject {
            mark: 0,
            object: fun
        };
        GCObject::Fun(Box::new(heap_obj))
    }
}

impl From<String> for GCObject {
    fn from(str: String) -> GCObject {
        let heap_obj = HeapObject {
            mark: 0,
            object: str
        };
        GCObject::Str(Box::new(heap_obj))
    }
}

impl From<&str> for GCObject {
    fn from(str: &str) -> GCObject {
        let heap_obj = HeapObject {
            mark: 0,
            object: String::from(str)
        };
        GCObject::Str(Box::new(heap_obj))
    }
}

impl Value
{
    /// Check if a value is marked (or not a markable object)
    fn is_marked(self) -> bool
    {
        let mark_bits_ptr = match self {
            Value::Fun(ptr) => unsafe { (ptr as *mut usize).offset(-1) },
            Value::Str(ptr) => unsafe { (ptr as *mut usize).offset(-1) },
            _ => return true
        };

        return unsafe { *mark_bits_ptr != 0 };
    }

    /// Mark a GC object
    fn mark(self)
    {
        let mark_bits_ptr = match self {
            Value::Fun(ptr) => unsafe { (ptr as *mut usize).offset(-1) },
            Value::Str(ptr) => unsafe { (ptr as *mut usize).offset(-1) },
            _ => return
        };

        unsafe { *mark_bits_ptr = 1 };
    }
}

pub struct VM
{
    /// Value stack
    stack: Vec<Value>,

    /// Program counter / instruction pointer
    pc: *const Insn,

    /// Frame pointer (index of the bottom of the frame)
    fp: usize,

    /// List of objects allocated in the GC heap
    gc_objects: Vec<GCObject>,

    /// Current total size of allocated objects in bytes
    heap_size: usize,

    /// Maximum heap size in bytes
    max_heap_size: usize,
}

impl VM
{
    pub fn new() -> Self
    {
        Self {
            stack: Vec::default(),
            pc: 0 as *const Insn,
            fp: 0,
            gc_objects: Vec::default(),
            heap_size: 0,
            max_heap_size: 10_000_000
        }
    }

    /// Place an object under management of the GC heap
    pub fn into_gc_heap<T>(&mut self, obj: T) -> Value where GCObject: From<T>
    {
        let mut obj: GCObject = obj.into();
        let val = obj.get_ptr_value();

        // Update the heap size
        self.heap_size += std::mem::size_of::<T>();

        // If we've exceeded the max heap size
        if self.heap_size > self.max_heap_size {
            // Don't trigger a GC if we're not currently executing anything
            // i.e. during compilation
            if self.stack.len() > 0 {
                self.gc_collect();
            }
        }

        // We push the new object after a potential collection
        // because we don't want to collect the new object
        self.gc_objects.push(obj);

        val
    }

    /// Transitively mark a GC root and everything reachable from it
    fn mark_root(&self, root: Value)
    {
        let mut stack: Vec<Value> = Vec::default();

        stack.push(root);

        while stack.len() > 0 {
            let val = stack.pop().unwrap();

            if val.is_marked() {
                continue;
            }

            match val {
                Value::Fun(fun_ptr) => {
                    let fun = unsafe { &*fun_ptr };

                    for insn in &fun.insns {
                        match insn {
                            Insn::Push { val } => stack.push(*val),
                            _ => {}
                        }
                    }

                    todo!()
                }

                _ => {}
            }

            val.mark();
        }
    }

    /// Perform a GC collection cycle (mark & sweep)
    pub fn gc_collect(&mut self)
    {
        //println!("gc objs before collection: {}", self.gc_objects.len());

        // Clear all the marks
        for obj in &mut self.gc_objects {
            obj.clear_mark();
        }

        // Mark all stack values as roots
        for val in &self.stack {
            self.mark_root(*val);
        }

        // Delete unmarked objects
        self.gc_objects.retain(|obj| obj.is_marked());

        //println!("gc objs after collection: {}", self.gc_objects.len());
    }

    /// Get the size of the stack
    pub fn stack_size(&self) -> usize
    {
        self.stack.len()
    }

    /// Pop a value from the stack
    pub fn stack_pop(&mut self) -> Value
    {
        self.stack.pop().expect("stack empty")
    }

    /// Push a value on the stack
    pub fn stack_push(&mut self, val: Value)
    {
        self.stack.push(val);
    }

    /// Push a Rust string onto the value stack
    pub fn push_str(&mut self, val: String)
    {
        let val = self.into_gc_heap(val);
        self.stack.push(val);
    }

    /// Push a Rust boolean onto the value stack
    pub fn push_bool(&mut self, val: bool)
    {
        let val = Value::Int64(
            if val { 1 } else { 0 }
        );

        self.stack.push(val);
    }

    pub fn eval(&mut self, fun: &Function) -> Value
    {
        use Insn::*;
        use Value::*;

        // Push the return address
        self.stack.push(Nil);

        // Push the previous frame pointer
        self.stack.push(Value::UInt64(self.fp as u64));

        // Set the frame pointer
        self.fp = self.stack.len();

        // Push space for all the locals
        self.stack.resize(self.stack.len() + fun.num_locals, Value::Nil);

        // Set the instruction pointer
        self.pc = &fun.insns[0] as *const Insn;

        loop
        {
            let insn = unsafe { *self.pc };
            //dbg!(insn);

            match insn {
                Panic => panic!("panic"),

                Halt => return Value::Nil,

                Push { val } => {
                    self.stack.push(val);
                }

                Pop => {
                    self.stack.pop();
                }

                Dup => {
                    let val = self.stack_pop();
                    self.stack.push(val);
                    self.stack.push(val);
                }

                SetLocal{ idx } => {
                    let val = self.stack_pop();
                    self.stack[self.fp + idx] = val;
                }

                GetLocal{ idx } => {
                    let val = self.stack[self.fp + idx];
                    self.stack.push(val);
                }

                Add => {
                    let v1 = self.stack_pop();
                    let v0 = self.stack_pop();
                    match (v0, v1) {
                        (Int64(v0), Int64(v1)) => self.stack.push(Int64(v0 + v1)),
                        (Str(s0), Str(s1)) => unsafe {
                            let mut out_str = String::from("");
                            out_str.push_str(&*s0);
                            out_str.push_str(&*s1);
                            self.push_str(out_str);
                        }
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

                Neg => {
                    let v0 = self.stack_pop();
                    match v0 {
                        Int64(v0) => self.stack.push(Int64(-v0)),
                        _ => panic!()
                    }
                }

                Mod => {
                    let v1 = self.stack_pop();
                    let v0 = self.stack_pop();
                    match (v0, v1) {
                        (Int64(v0), Int64(v1)) => self.stack.push(Int64(v0 % v1)),
                        _ => panic!()
                    }
                }

                Eq => {
                    let v1 = self.stack_pop();
                    let v0 = self.stack_pop();
                    match (v0, v1) {
                        (Int64(v0), Int64(v1)) => self.push_bool(v0 == v1),
                        (Str(s0), Str(s1)) => unsafe {
                            self.push_bool(&*s0 == &*s1);
                        }
                        _ => panic!()
                    };
                }

                Ne => {
                    let v1 = self.stack_pop();
                    let v0 = self.stack_pop();
                    match (v0, v1) {
                        (Int64(v0), Int64(v1)) => self.push_bool(v0 != v1),
                        _ => panic!()
                    };
                }

                Lt => {
                    let v1 = self.stack_pop();
                    let v0 = self.stack_pop();
                    match (v0, v1) {
                        (Int64(v0), Int64(v1)) => self.push_bool(v0 < v1),
                        _ => panic!()
                    };
                }

                Not => {
                    let v0 = self.stack_pop();
                    match v0 {
                        Int64(v0) => self.push_bool(v0 == 0),
                        _ => panic!()
                    };
                }

                Jump{ offset } => {
                    self.pc = unsafe { self.pc.offset(offset as isize) };
                }

                IfTrue{ offset } => {
                    let v = self.stack_pop();
                    match v {
                        Int64(v) => {
                            if v != 0 {
                                self.pc = unsafe { self.pc.offset(offset as isize) }
                            }
                        }
                        _ => panic!()
                    }
                }

                IfFalse{ offset } => {
                    let v = self.stack_pop();
                    match v {
                        Int64(v) => {
                            if v == 0 {
                                self.pc = unsafe { self.pc.offset(offset as isize) }
                            }
                        }
                        _ => panic!()
                    }
                }

                Call { argc } => {
                    // The callee was pushed on the stack first
                    let callee = &self.stack[self.stack.len() - argc - 1];

                    // The last argument is at the top
                    // This pointer is invalid if argc is zero
                    let args = match argc {
                        0 => 0 as *const Value,
                        _ => &self.stack[self.stack.len() - argc] as *const Value
                    };

                    match callee {
                        HostFn(host_fn) => {
                            let retv = host_fn(args, argc);
                            self.stack.push(retv);
                        }
                        _ => panic!()
                    }
                }

                Return => {
                    return self.stack_pop();
                }

                #[allow(unreachable_patterns)]
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
        let mut vm = VM::new();
        let mut input = Input::new(src, "test_src");
        let unit_fn = parse_unit(&mut vm, &mut input).unwrap();
        return vm.eval(&unit_fn);
    }

    fn eval_file(file_name: & str) -> Value
    {
        dbg!(file_name);
        let mut vm = VM::new();
        let unit_fn = parse_file(&mut vm, file_name).unwrap();
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
        assert_eq!(eval_src("return -3;"), Int64(-3));
    }

    #[test]
    fn test_infix_priority()
    {
        // Priority of operations
        assert_eq!(eval_src("return 1 + 2 * 3;"), Int64(7));
        assert_eq!(eval_src("return 1 + 2 + 3 + 4;"), Int64(10));
        assert_eq!(eval_src("return 1 * 2 + 3 * 4;"), Int64(14));
        assert_eq!(eval_src("return (1 + 2) * 3;"), Int64(9));
        assert_eq!(eval_src("return (1 * 2) + (3 * 4);"), Int64(14));
        assert_eq!(eval_src("return 1 + 2 * 3 + 4;"), Int64(11));
        assert_eq!(eval_src("return -(1 + 2 * 3) + 4;"), Int64(-3));

        // Subtract and operand ordering
        assert_eq!(eval_src("return 5 - 3;"), Int64(2));
        assert_eq!(eval_src("return 5 + 2 - 3;"), Int64(4));
    }

    #[test]
    fn test_let_stmt()
    {
        eval_src("let x = 3;");
        assert_eq!(eval_src("let x = 2; return x;"), Int64(2));
        assert_eq!(eval_src("let x = 2; let y = 3; return x + y;"), Int64(5));
    }

    #[test]
    fn test_assign()
    {
        assert_eq!(eval_src("let x = 2; x = 3; return x;"), Int64(3));
        assert_eq!(eval_src("let x = 1; let y = 1; x = y = 3; return x+y;"), Int64(6));
        assert_eq!(eval_src("let x = 1; let y = 1; x = y = x + 3; return x+y;"), Int64(8));

        // Nested scopes
        assert_eq!(eval_src("let x = 2; { return x; }"), Int64(2));
        assert_eq!(eval_src("let x = 2; { let x = 3; return x; }"), Int64(3));
        assert_eq!(eval_src("let x = 2; { let x = 3; x; } return x; "), Int64(2));
    }

    #[test]
    fn test_assert()
    {
        eval_src("assert 3;");
    }

    #[test]
    fn test_if()
    {
        assert_eq!(eval_src("let x = 0; if (0) x = 2; return x;"), Int64(0));
        assert_eq!(eval_src("let x = 0; if (1) x = 2; return x;"), Int64(2));
        assert_eq!(eval_src("let x = 0; if (0) x = x+2; else x = x+1; return x;"), Int64(1));
        assert_eq!(eval_src("let x = 0; if (1) x = x+2; else x = x+1; return x;"), Int64(2));
        assert_eq!(eval_src("let x = 0; if (x) return 1; else return 0;"), Int64(0));
        assert_eq!(eval_src("let x = 0; if (!x) return 1; else return 0;"), Int64(1));
    }

    #[test]
    fn test_while()
    {
        assert_eq!(eval_src("let i = 0; while (i < 10) i = i + 1; return i;"), Int64(10));
    }

    #[test]
    fn test_gc()
    {
        let mut vm = VM::new();
        let str_val = vm.into_gc_heap("hello");

        vm.stack_push(str_val);
        vm.gc_collect();
        assert!(vm.gc_objects.len() == 1);

        match str_val {
            Str(str_ptr) => unsafe {
                let str = &*str_ptr;
                assert!(str == "hello");
            }
            _ => panic!()
        }

        vm.stack_pop();
        vm.gc_collect();
        assert!(vm.gc_objects.len() == 0);
    }

    #[test]
    fn test_strings()
    {
        assert_eq!(eval_src("return 'foo' + 'bar' == 'foobar';"), Int64(1));
    }

    #[test]
    fn test_examples()
    {
        // Make sure that we can run the examples successfully
        eval_file("examples/syntax.ks");
        eval_file("examples/fizzbuzz.ks");
    }
}
