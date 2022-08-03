use crate::vm::Value;
use Value::*;

pub type HostFn = fn(args: *const Value, argc: usize) -> Value;

fn println(args: *const Value, argc: usize) -> Value
{
    for i in 0..argc {
        let arg = unsafe { &*args.add(i) };

        match arg {
            Int64(v) => print!("{}", v),
            Str(s) => print!("{}", s),
            _ => panic!()
        }
    }

    println!();

    Value::Nil
}

pub fn get_runtime_fn(name: &str) -> Option<HostFn>
{
    match name {
        "println" => Some(println),
        _ => None
    }
}
