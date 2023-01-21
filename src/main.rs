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

    let age:i32 = 8;

    if (age >=1) && (age <= 18) {
        print!("Important birthday");
    } else if (age == 21) || (age == 50) {
        print!("Important birthday");
    } else if (age >= 65) {
        print!("Important birthday");
    } else {
        print!("Not important birthday");
    }

    let mut my_age = 47;
    let can_vote = if my_age >= 18 {
        true
    } else {
        false
    };

    print!("can vote: {}", can_vote);

    let age2 = 8;
    match age2 {
        1..=18 => print!("Important birthday"),
        21 | 50 => print!("Important birthday"),
        65..=i32::MAX => print!("Important birthday"),
        _ => print!("Not important")
    };

    let my_age = 18;
    let voting_age = 18;

    match my_age.cmp(&voting_age) {
        Ordering::Less => print!("can't vote"),
        Ordering::Greater => print!("can vote"),
        Ordering::Equal => print!("gained the right to vote")
    };

    let arr_1 = [1,2,3,4,5];

    print!("1st: {}", arr_1[0]);
    print!("lenght: {}", arr_1.len());

    // loop

    let mut loop_index = 0;
    loop {
        if arr_1[loop_index] % 2 == 0 {
            loop_index += 1;
            continue;
        }
        if arr_1[loop_index] == 5 {
            break;
        }
        print!("Val : {}",arr_1[loop_index]);
        loop_index += 1;
    }

    let mut while_loop_idx = 0;

    while (while_loop_idx < arr_1.len()) {
        print!("VaL : {}",arr_1[while_loop_idx]);
        while_loop_idx += 1;
    }

    for val in arr_1 {
        print!("VAL : {}", val);
    }

    let my_tuple: (u8, String, f64) = (47, "Sumit".to_string(), 50_000.00);
    print!("Name: {}", my_tuple.1);
    let(v1, v2, v3) = my_tuple;
    print!("Age : {}", my_tuple.0);

    


}
