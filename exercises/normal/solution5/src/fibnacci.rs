use std::ops::{Div, Mul};

pub fn odd_fibnacci_sum(threshold: u32) -> u32 {
    let mut sum = 0;
    for i in 0..=threshold {
        let fib = fib(i);
        if fib % 2 == 1 {
            sum += fib;
        }
    }
    sum
}

pub fn fib(n: u32) -> u32 {
    1.0_f32.div(5.0_f32.sqrt()).mul((1.0_f32 + 5.0_f32.sqrt()).div(2.0_f32).powf(n as f32) - (1.0_f32 - 5.0_f32.sqrt()).div(2.0_f32).powf(n as f32))
        .round() as u32
}