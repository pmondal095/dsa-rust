use std::{collections::HashMap, io};

fn main() {
    let mut vec = take_array_input();
    let target_sum = take_sum_input();
    pair_sum(&mut vec, target_sum);
}

fn take_sum_input() -> i32 {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let vector_size: i32 = n.trim().parse().expect("invalid input");
    vector_size
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

    return numbers;
}

// Time & Space: O(n^2), O(1)
fn pair_sum2(vec: &mut Vec<i32>, target_sum: i32) {
    let mut pair_count = 0;

    for i in 0..vec.len() - 1 as usize {
        for j in (i + 1)..vec.len() as usize {
            let sum = vec[i] + vec[j];
            if sum == target_sum {
                println!("pair : ({},{})", vec[i], vec[j]);
                pair_count += 1;
                break;
            }
        }
    }
    if pair_count == 0 {
        println!("No pairs found...")
    }
}

// Time & Space: O(n), O(n)
fn pair_sum1(vec: &mut Vec<i32>, target_sum: i32) {
    let mut pair_map: HashMap<i32, bool> = HashMap::new();
    for i in 0..vec.len() - 1 as usize {
        let other_pair = target_sum - vec[i];
        if pair_map.get(&other_pair).is_none() {
            pair_map.insert(vec[i], true);
        } else {
            println!("pair : ({},{})", vec[i], other_pair);
        }
    }
}

// Time & Space: O(nlogn), O(1)
fn pair_sum(vec: &mut Vec<i32>, target_sum: i32) {

    vec.sort_unstable(); // faster & in place
    let mut left: usize = 0;
    let mut right = vec.len() - 1;

    while left < right {
        let curr_sum = vec[left] + vec[right];
        if curr_sum == target_sum {
            println!("pair : ({},{})", vec[left], vec[right]);
            left += 1;
            right -= 1;
        } else if curr_sum > target_sum {
            right -= 1;
        } else {
            left += 1;
        }
    }
}