#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_parens)]

mod vm;
use vm::*;

mod parser;
use parser::*;

mod ast;
use ast::*;

fn test_ptrs(x_ptr: *mut i32, size: usize)
{

    // Initialize elements via raw pointer writes, then set length.
    unsafe {
        for i in 0..size {
            *x_ptr.add(i) = (i as i32);
        }
    }


}



fn main()
{
    println!("Hello");

    let _x = Instr::Push{ val:3 };


    let _input = Input::new("test src str".to_string(), "input".to_string());
    let _b = '0' == '1';
    let _b2 = '0' < '1';
    let _b2 = '0' < 0x95 as char;
    let _x = '0' as u8;








    // Allocate vector big enough for 4 elements.
    let size = 4;
    let mut x: Vec<i32> = Vec::with_capacity(size);
    x.resize(4, 0);
    let x_ptr = x.as_mut_ptr();

    test_ptrs(x_ptr, size);
    assert_eq!(&*x, &[0, 1, 2, 3]);








}
