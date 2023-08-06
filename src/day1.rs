use crate::utils::*;

pub fn run() {
    day_1_1();
    day_1_2();
}

fn day_1_1() {
    println!("Day 01, Task 01");
    let calories_per_elf = read_input_to_groups("_inputs/input-01-01.txt");

    let max = calories_per_elf
        .iter()
        .fold(Vec::new(), fold_sum_sublists)
        .iter()
        .fold(0, |accu, item| accu.max(*item));

    println!("{:?}", max)
}

fn day_1_2() {
    println!("Day 01, Task 02");

    let calories_per_elf = read_input_to_groups("_inputs/input-01-01.txt");
    let mut summed_calories = calories_per_elf.iter().fold(Vec::new(), fold_sum_sublists);

    // really dont like how this functions change the original vector
    summed_calories.sort();
    summed_calories.reverse();

    let top_3: Vec<i32> = summed_calories[0..3].iter().cloned().collect();
    let top_3_sum: i32 = top_3.iter().sum();

    println!("{:?}\n\n{:?} -> {:?}", summed_calories, top_3, top_3_sum)
}
