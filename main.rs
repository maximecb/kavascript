#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

mod vm;
use vm::*;

mod parser;
use parser::*;

fn main()
{
    println!("Hello");

    let _x = vm::Instr::Push{ val:3 };


    let _input = parser::Input::new("test src str".to_string(), "input".to_string());


    let _b = '0' == '1';
    let _b2 = '0' < '1';
    let _b2 = '0' < 0x95 as char;
    let _x = '0' as u8;



}
