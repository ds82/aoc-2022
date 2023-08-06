use crate::utils;
use std::collections::HashMap;

pub fn run() {
    day_3_1();
    day_3_2();
}

fn day_3_1() {
    println!("Day 03, Task 01");

    let lines = utils::read_input_strings("_inputs/input-03-01.txt");

    let prio_per_line: Vec<u32> = lines
        .iter()
        .map(|line| {
            let (s1, s2) = utils::str_split_half(line);
            let mut char_map: HashMap<char, (u32, u32)> = HashMap::new();

            for c in s1.chars() {
                char_map.entry(c).or_insert((1, 0));
            }
            for c in s2.chars() {
                char_map
                    .entry(c)
                    .and_modify(|(a, b)| *b = 1)
                    .or_insert((0, 1));
            }

            let char_in_both = char_map.iter().find(|(key, &(a, b))| a == 1 && b == 1);
            let unwrap_char = match char_in_both {
                Some((char, _)) => *char,
                None => ' ',
            };

            unwrap_char
        })
        .map(utils::map_char_to_prio)
        .collect();

    let sum: u32 = prio_per_line.iter().sum();

    println!("{:?}", sum);
}

fn day_3_2() {
    println!("Day 03, Task 01");

    let lines = utils::read_input_strings("_inputs/input-03-01.txt");
    let lines_grouped = utils::group_by(3, lines);

    let char_map: HashMap<char, (u8, u8, u8)> = HashMap::new();

    let prio_per_group: Vec<u32> = lines_grouped
        .iter()
        .map(|group: &Vec<String>| {
            let s1 = &group[0];
            let s2 = &group[1];
            let s3 = &group[2];

            let mut char_map: HashMap<char, (u32, u32, u32)> = HashMap::new();

            for c in s1.chars() {
                char_map.entry(c).or_insert((1, 0, 0));
            }
            for c in s2.chars() {
                char_map
                    .entry(c)
                    .and_modify(|(a, b, c)| *b = 1)
                    .or_insert((0, 1, 0));
            }
            for c in s3.chars() {
                char_map
                    .entry(c)
                    .and_modify(|(a, b, c)| *c = 1)
                    .or_insert((0, 0, 1));
            }

            let char_in_all = char_map
                .iter()
                .find(|(key, &(a, b, c))| a == 1 && b == 1 && c == 1);
            let unwrap_char = match char_in_all {
                Some((char, _)) => *char,
                None => {
                    println!("Found no char in all three in {}, {}, {}", s1, s2, s3);
                    ' '
                }
            };

            unwrap_char
        })
        .map(utils::map_char_to_prio)
        .collect();

    let sum: u32 = prio_per_group.iter().sum();

    println!("{:?} \n\n {:?}", prio_per_group, sum)
}
