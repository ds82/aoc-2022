use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

// fn readfile(filename: string) {
//     let cont

// }

pub fn get_file_buffered(filename: impl AsRef<Path>) -> BufReader<File> {
    let file = File::open(filename).expect("no such file");
    BufReader::new(file)
}

pub fn read_input_strings(filename: impl AsRef<Path>) -> Vec<String> {
    let buf = get_file_buffered(filename);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

pub fn read_input_numbers(filename: impl AsRef<Path>) -> Vec<i32> {
    // let file = File::open(filename).expect("no such file");
    // let buf = BufReader::new(file);
    let buf = get_file_buffered(filename);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .map(|l| l.parse::<i32>().unwrap())
        .collect()
}

pub fn read_input_to_groups(filename: impl AsRef<Path>) -> Vec<Vec<i32>> {
    let buf = get_file_buffered(filename);
    let start_with = vec![vec![]];

    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .fold(start_with, |mut accu, item| {
            let is_empty_line = item == "";
            if is_empty_line {
                let new_entry = Vec::new();
                accu.push(new_entry);
            } else {
                let parsed_item = item.parse::<i32>().unwrap();
                let last_accu_index = accu.len() - 1;
                accu[last_accu_index].push(parsed_item);
            }

            accu
        })
}

pub fn read_input_tuple_per_line(filename: impl AsRef<Path>) -> Vec<(char, char)> {
    let buf = get_file_buffered(filename);

    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .map(|l| {
            let chars_in_line: Vec<char> = l.chars().collect();
            (chars_in_line[0], chars_in_line[2])
        })
        .collect()
}

pub fn group_by<T>(group_size: usize, collection: Vec<T>) -> Vec<Vec<T>>
where
    T: Clone + std::fmt::Debug,
{
    let grouped_collection = collection.iter().step_by(group_size).enumerate().fold(
        Vec::new() as Vec<Vec<T>>,
        |mut accu, (index, item)| {
            let mut part = vec![];
            let ori_index = index * group_size;

            for n in ori_index..ori_index + group_size {
                part.push(collection[n].clone());
            }

            accu.push(part);

            accu
        },
    );

    // println!("grouped collection");
    // println!("{:?}", grouped_collection);

    grouped_collection
}

pub fn fold_sum_sublists(mut list: Vec<i32>, item: &Vec<i32>) -> Vec<i32> {
    list.push(item.iter().sum());
    list
}

#[test]
fn test_group_by() {
    let t1 = vec!["a", "b", "c", "d", "e", "f", "g", "h"];

    assert_eq!(
        group_by(2, t1),
        [["a", "b"], ["c", "d"], ["e", "f"], ["g", "h"]]
    )
}

pub fn str_split_half(s: &String) -> (String, String) {
    let len = s.len();
    assert!(
        len % 2 == 0,
        "str_split_half needs even number of chars in string"
    );

    ((s[..(len / 2)]).to_string(), (s[len / 2..len]).to_string())
}
#[test]
fn test_str_split_half() {
    assert_eq!(
        str_split_half(&"xxyy".to_string()),
        ("xx".to_string(), "yy".to_string())
    );
    assert_eq!(
        str_split_half(&"abcxyz".to_string()),
        ("abc".to_string(), "xyz".to_string())
    );
    assert_eq!(
        str_split_half(&"VpbpZZbvPLbZbbBhwqMHhsGMnJdVwV".to_string()),
        ("VpbpZZbvPLbZbbB".to_string(), "hwqMHhsGMnJdVwV".to_string())
    );
}

// need:
// a - z -> 1 .. 26
// A - Z -> 27 .. 52
// ascii:
// a: 97
// A: 65
pub fn map_char_to_prio(c: char) -> u32 {
    let num = c as u32;
    if num >= 65 && num < 97 {
        num - 38
    } else if num >= 97 && num < 123 {
        num - 96
    } else {
        0
    }
}

#[test]
fn test_map_char_to_prio() {
    assert_eq!(map_char_to_prio('a'), 1);
    assert_eq!(map_char_to_prio('b'), 2);
    assert_eq!(map_char_to_prio('s'), 19);
    assert_eq!(map_char_to_prio('z'), 26);

    assert_eq!(map_char_to_prio('A'), 27);
    assert_eq!(map_char_to_prio('B'), 28);
    assert_eq!(map_char_to_prio('S'), 45);
    assert_eq!(map_char_to_prio('Z'), 52);
}
