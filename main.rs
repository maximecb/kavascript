#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_parens)]
#![allow(unused_mut)]

use std::env;

mod vm;
use vm::*;

mod parser;
use parser::*;

mod runtime;

fn main()
{
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    // If an input file was specified
    if args.len() == 2 {
        let mut vm = VM::new();
        let unit_fn = parse_file(&mut vm, &args[1]).unwrap();
        vm.eval(&unit_fn);
    }
}
