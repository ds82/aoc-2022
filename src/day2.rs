use crate::utils;
use std::collections::HashMap;

pub fn run() {
    day_2_1();
    day_2_2();
}

fn calculate_rock_paper_scissor_result((p1, p2): &(char, char)) -> u32 {
    /*
    *
    A = Rock
    B = Paper
    C = Scissors

    X = Rock (1)
    Y = Paper (2)
    Z =  Scissors (3)

    *
    */
    match (p1, p2) {
        ('A', 'X') => 3 + 1,
        ('A', 'Y') => 6 + 2,
        ('A', 'Z') => 0 + 3,

        ('B', 'X') => 0 + 1,
        ('B', 'Y') => 3 + 2,
        ('B', 'Z') => 6 + 3,

        ('C', 'X') => 6 + 1,
        ('C', 'Y') => 0 + 2,
        ('C', 'Z') => 3 + 3,
        (_, _) => 0,
    }
}
fn calculate_rock_paper_scissor_secret_decrpt_result((p1, p2): &(char, char)) -> u32 {
    /*
    *
    A = Rock (1)
    B = Paper (2)
    C = Scissors (3)

    X = Loose
    Y = Draw
    Z = Win

    *
    */
    match (p1, p2) {
        ('A', 'X') => 0 + 3,
        ('A', 'Y') => 3 + 1,
        ('A', 'Z') => 6 + 2,

        ('B', 'X') => 0 + 1,
        ('B', 'Y') => 3 + 2,
        ('B', 'Z') => 6 + 3,

        ('C', 'X') => 0 + 2,
        ('C', 'Y') => 3 + 3,
        ('C', 'Z') => 6 + 1,
        (_, _) => 0,
    }
}

fn day_2_1() {
    println!("Day 01, Task 01");

    let strategy = utils::read_input_tuple_per_line("_inputs/input-02-01.txt");
    let input: Vec<u32> = strategy
        .iter()
        .map(calculate_rock_paper_scissor_result)
        .collect();

    let score: u32 = input.iter().sum();

    println!("{:?} \n\n {:?}", input, score)
}

fn day_2_2() {
    println!("Day 02, Task 02");

    let strategy = utils::read_input_tuple_per_line("_inputs/input-02-01.txt");
    let input: Vec<u32> = strategy
        .iter()
        .map(calculate_rock_paper_scissor_secret_decrpt_result)
        .collect();

    let score: u32 = input.iter().sum();

    println!("{:?} \n\n {:?}", input, score)
}
