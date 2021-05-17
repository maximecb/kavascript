#![allow(unused_imports)]
#![allow(dead_code)]

mod vm;
use vm::*;

mod parser;
use parser::*;

fn main()
{
    let _x = vm::Instr::Push{ val:3 };



}
