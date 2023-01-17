use core::num;
use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    println!("What is your name?");

    let mut name = String::new();
    let greeting = "Nice to meet you";
    io::stdin().read_line(&mut name)
        .expect("Didn't receive input");

    println!("Hello, {}! {}", name.trim_end(), greeting);

    const ONE_MIL: u32 = 1_000_000;
    const PI: f32 = 3.141592654;

    // shadowing
    let age = "47";
    let mut age:u32 = age.trim().parse()
        .expect("Not a number");

    age = age + 1;

    print!("I am {} and I want {}", age, ONE_MIL);

    // Data types //

    // Unsigned integers - u8, u16, u32, u64, u128, usize
    // Signed integers - i8, i16, i32, i64, i128, isize

    print!("MAX u32: {} ", u8::MAX);
    print!("MAX usize: {} ", usize::MAX);
    print!("MAX f32: {} ", f32::MAX);

    let is_true = true;
    let my_grade = 'A';

    let num_1: f32 = 1.111111111111111;
    print!("f32 is : {} ", num_1 + 0.111111111111111);

    let num_2: f64 = 1.111111111111111;
    print!("f64 is : {} ", num_2 + 0.111111111111111);

    let mut num_3 = 12;
    num_3 += 3;
    print!("num_3 is {}", num_3);

    let random_num = rand::thread_rng().gen_range(1..101);
    print!("Random: {}", random_num);

    


}
