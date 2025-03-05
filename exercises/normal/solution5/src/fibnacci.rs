use std::ops::{Div, Mul};

pub fn odd_fibnacci_sum(threshold: u32) -> u32 {
    let mut sum = 0;
    let mut start = 0;
    loop {
        let fib = fib(start);
        if fib > threshold {
            break;
        }
        if fib % 2 == 1 {
            sum += fib;
        }
        start += 1;
    }
    sum
}

pub fn fib(n: u32) -> u32 {
    1.0_f32.div(5.0_f32.sqrt()).mul((1.0_f32 + 5.0_f32.sqrt()).div(2.0_f32).powf(n as f32) - (1.0_f32 - 5.0_f32.sqrt()).div(2.0_f32).powf(n as f32))
        .round() as u32
}