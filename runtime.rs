use crate::vm::*;

fn print(args: *const Value) -> Value
{
    todo!();



}

fn get_runtime_fn(name: &str) -> Option<HostFn>
{
    match name {
        "print" => Some(print),
        _ => None
    }
}
