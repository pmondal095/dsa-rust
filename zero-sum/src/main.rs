use std::{io, vec, collections::HashMap};

fn main() {
    // println!("Hello, world!");
    let mut vec = take_array_input();
    let is_true = check_zero_sum_subarray2(&mut vec);
    println!("sub array check :{}", is_true);
}

fn take_array_input() -> Vec<i32> {
    // take array input in string format
    let mut numbers = String::new();
    io::stdin()
        .read_line(&mut numbers)
        .ok()
        .expect("read error");

    // parse string array into integer array
    let numbers: Vec<i32> = numbers
        .split_whitespace()
        .map(|s| s.parse().expect("parse error"))
        .collect();

    numbers
}

// Time & Space complexity : O(n^2), O(1)
fn check_zero_sum_subarray1(arr: &mut Vec<i32>) -> bool {
    for i in 0..arr.len() - 1 as usize {
        let mut curr_sum = arr[i];
        for j in i + 1..arr.len() as usize {
            curr_sum += arr[j];
            if curr_sum == 0 {
                return true;
            }
        }
    }
    false
}

// Time & Space complexity : O(n), O(n)
fn check_zero_sum_subarray2(arr: &mut Vec<i32>) -> bool {

    let mut set: HashMap<i32,bool> = HashMap::with_capacity(arr.len() as usize);
    let mut prefix_arr: Vec<i32> = Vec::with_capacity(arr.len() as usize);

    prefix_arr.push(arr[0]);

    for i in 1..arr.len() as usize {
        prefix_arr.push(prefix_arr[i - 1] + arr[i]);
    }
    
    for sum in &prefix_arr {
        if set.contains_key(sum) {
            return true;
        } else {
            set.insert(*sum, true);
        }
    }

    if set.contains_key(&0) {
        return true;
    }
    false
}
