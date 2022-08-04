use std::io;
use crate::vm::Value;
use Value::*;

pub type HostFn = fn(args: *const Value, argc: usize) -> Value;

/// Print values to standard output
fn print(args: *const Value, argc: usize) -> Value
{
    for i in 0..argc {
        let arg = unsafe { &*args.add(i) };

        match arg {
            Int64(v) => print!("{}", v),
            Str(s) => print!("{}", s),
            _ => panic!()
        }
    }

    Value::Nil
}

/// Print values to standard output, and then output a newline
fn println(args: *const Value, argc: usize) -> Value
{
    print(args, argc);
    println!();
    Value::Nil
}

/// Read an integer from standard input
fn read_int(args: *const Value, argc: usize) -> Value
{
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: i64 = input.trim().parse().unwrap();
    Value::Int64(n)
}

/// Look up a runtime function by name
pub fn get_runtime_fn(name: &str) -> Option<HostFn>
{
    match name {
        "print" => Some(print),
        "println" => Some(println),
        "read_int" => Some(read_int),
        _ => None
    }
}
