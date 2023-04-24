use std::{
    collections::{hash_map::Entry, HashMap},
    io, vec,
};

fn main() {
    let mut vec = take_array_input();
    check_zero_sum_subarray2(&mut vec);
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


// Time & Space complexity : O(n*n), O(n)
fn check_zero_sum_subarray2(arr: &mut Vec<i32>) {
    let mut set: HashMap<i32, Vec<usize>> = HashMap::with_capacity(arr.len() as usize);
    let mut prefix_arr: Vec<i32> = Vec::with_capacity(arr.len() as usize);

    prefix_arr.push(arr[0]);

    for i in 1..arr.len() {
        prefix_arr.push(prefix_arr[i - 1] + arr[i]);
    }

    for index in 0..prefix_arr.len() {
        match set.entry(prefix_arr[index]) {
            Entry::Vacant(e) => {
                e.insert(vec![index]);
            }
            Entry::Occupied(mut e) => {
                e.get_mut().push(index);
            }
        }
    }

    for (key, value) in &set {
        if *key == 0 {
            for val in value {
                println!("pair : ({},{})", 0, *val);
            }
        }

        if value.len() > 1 {
            for i in 0..value.len() - 1 {
                for j in i + 1..value.len() {
                    println!("pair : ({},{})", value[i] + 1, value[j]);
                }
            }
        }
    }

    println!("{:?}", set);

}

