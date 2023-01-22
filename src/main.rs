use core::num;
use std::{io, vec};
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

    println!("I am {} and I want {}", age, ONE_MIL);

    // Data types //

    // Unsigned integers - u8, u16, u32, u64, u128, usize
    // Signed integers - i8, i16, i32, i64, i128, isize

    println!("MAX u32: {} ", u8::MAX);
    println!("MAX usize: {} ", usize::MAX);
    println!("MAX f32: {} ", f32::MAX);

    let is_true = true;
    let my_grade = 'A';

    let num_1: f32 = 1.111111111111111;
    println!("f32 is : {} ", num_1 + 0.111111111111111);

    let num_2: f64 = 1.111111111111111;
    println!("f64 is : {} ", num_2 + 0.111111111111111);

    let mut num_3 = 12;
    num_3 += 3;
    println!("num_3 is {}", num_3);

    let random_num = rand::thread_rng().gen_range(1..101);
    println!("Random: {}", random_num);

    let age:i32 = 8;

    if (age >=1) && (age <= 18) {
        println!("Important birthday");
    } else if (age == 21) || (age == 50) {
        println!("Important birthday");
    } else if (age >= 65) {
        println!("Important birthday");
    } else {
        println!("Not important birthday");
    }

    let mut my_age = 47;
    let can_vote = if my_age >= 18 {
        true
    } else {
        false
    };

    println!("can vote: {}", can_vote);

    let age2 = 8;
    match age2 {
        1..=18 => println!("Important birthday"),
        21 | 50 => println!("Important birthday"),
        65..=i32::MAX => println!("Important birthday"),
        _ => println!("Not important")
    };

    let my_age = 18;
    let voting_age = 18;

    match my_age.cmp(&voting_age) {
        Ordering::Less => println!("can't vote"),
        Ordering::Greater => println!("can vote"),
        Ordering::Equal => println!("gained the right to vote")
    };

    let arr_1 = [1,2,3,4,5];

    println!("1st: {}", arr_1[0]);
    println!("lenght: {}", arr_1.len());

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
        println!("Val : {}",arr_1[loop_index]);
        loop_index += 1;
    }

    let mut while_loop_idx = 0;

    while (while_loop_idx < arr_1.len()) {
        println!("VaL : {}",arr_1[while_loop_idx]);
        while_loop_idx += 1;
    }

    for val in arr_1 {
        println!("VAL : {}", val);
    }

    let my_tuple: (u8, String, f64) = (47, "Sumit".to_string(), 50_000.00);
    println!("Name: {}", my_tuple.1);
    let(v1, v2, v3) = my_tuple;
    println!("Age : {}", my_tuple.0);

    let mut st1 = String::new();
    st1.push('A');
    st1.push_str(" word");

    for word in st1.split_whitespace() {
        println!("{}",word);
    }
    let st2 = st1.replace("A", "another");
    println!("{}",st2);

    let st3 = String::from("x r t b kk l og f c a f");
    let mut v1: Vec<char> = st3.chars().collect();
    v1.sort();
    v1.dedup();

    for char in v1 {
        println!("{}", char);
    }

    let st4: &str = "Random string";
    let mut st5: String = st4.to_string();
    println!("{}", st5);

    let byte_arr1 = st5.as_bytes();
    let st6 = &st5[0..6];
    println!("String length : {}", st6.len());
    st5.clear();

    let st6 = String::from("just some");
    let st7 = String::from(" words");
    let st8 = st6 + &st7;

    println!("st7 : {}", st7);
    println!("st8 : {}", st8);

    for char in st8.bytes() {
        println!("{}", char);
    }

    // casting
    let int_u8: u8 = 5;
    let int2_u8:u8 = 4;
    let int3_u32: u32 = (int_u8 as u32) + (int2_u8 as u32);

    // enum
    enum Day {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday
    }

    impl Day {
        fn is_weekend(&self) -> bool {
            match self {
                Day::Saturday | Day::Sunday => true,
                _ => false
            }
        }
    }

    let today:Day = Day::Monday;

    match today {
        Day::Monday => println!("Everyone hates Monday."),
        Day::Tuesday => println!("Donut day."),
        Day::Wednesday => println!("Hump day."),
        Day::Thursday => println!("Laxmi day"),
        Day::Friday => println!("Pay day."),
        Day::Saturday => println!("Weekend"),
        Day::Sunday => println!("Weekend")
    }

    println!("Is today the weekend {}", today.is_weekend());

    // vectors
    let vec1: Vec<i32> = Vec::new();

    let mut vec2: Vec<i32> = vec![1,2,3,4];
    vec2.push(5);
    println!("1st : {}", vec2[0]);

    let second: &i32 = &vec2[1];

    match vec2.get(1) {
        Some(second) => println!("2nd : {}", second),
        None => println!("no 2nd value"),
    }

    for i in &mut vec2 {
        *i *= 2;
    }

    for i in &vec2 {
        println!("{}", i)
    }

    println!("vec len {}", vec2.len());
    println!("pop : {:?}", vec2.pop());

}
