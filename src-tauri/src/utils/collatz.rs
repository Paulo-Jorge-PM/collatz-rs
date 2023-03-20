use serde::{Serialize};


//#[derive(Debug, Deserialize, Serialize)]
#[derive(serde::Serialize)]
pub struct Number {
    x:usize,
    y:usize
 }
 
 /*impl Number {
    pub fn new(x:usize, y:usize) -> Self {
        Self {
            x,
            y,
        }
    }
}*/

pub fn calculate_collatz(start_number: usize) -> Vec<Number> {

    println!("> Starting number: {}", start_number);

    let mut result: Vec<Number> = Vec::new();
    let mut current_number: usize = start_number;
    let mut counter: usize = 1;

    //result.push("{"+counter.to_string()+":"+start_number.to_string()+"}");
    //result.push(start_number.to_string());

    //result.push(format!("[{{x:{}, y:{}}}", counter, start_number));
    result.push(Number{x: counter, y: start_number});

    while current_number != 1 {
        if current_number % 2 == 0 {
            current_number = current_number / 2;
        }
        else {
            current_number = current_number * 3 + 1;
        }
        println!("> Step {}: {}", counter, current_number);
        counter += 1;
        //result.push(format!("{{x:{}, y:{}}}", counter, current_number));
        result.push(Number{x: counter, y: current_number})
    }

    let n = result.len() - 1;
    let last = &mut result[n];

    return result;
}