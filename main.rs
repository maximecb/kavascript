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

fn main()
{
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);














}
