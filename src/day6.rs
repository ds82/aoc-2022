use crate::utils;
use itertools::Itertools;
use std::error::Error;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

const UNIQUE_CHAR_MARKER: usize = 14;

pub fn run() {
    day_6_1();
    day_6_2();
}

#[derive(Debug)]
struct Marker {
    need_unique: usize,
    received: usize,
    marker_value: Vec<char>,
    marker_found_after: usize,
    message_content: Vec<char>,
}

impl Marker {
    fn new(need_unique: usize) -> Marker {
        Marker {
            need_unique,
            marker_value: vec![],
            marker_found_after: 0,
            message_content: vec![],
            received: 0,
        }
    }

    fn push_symbol(&mut self, symbol: char) -> () {
        if !self.is_valid_marker() {
            if self.marker_value.len() >= self.need_unique {
                self.marker_value = self.marker_value.split_off(1);
            }
            self.marker_value.push(symbol);

            if self.is_valid_marker() {
                self.marker_found_after = self.received + 1;
            }
        } else {
            self.message_content.push(symbol);
        }
        self.received += 1;
    }

    fn is_valid_marker(&self) -> bool {
        self.marker_value.iter().unique().count() == self.need_unique
    }

    fn get_received(&self) -> usize {
        self.received
    }
}

fn day_6_1() {
    println!("== Day 6, Task 1 ==");
    let complete_message = utils::read_input_strings("_inputs/input-06-01.txt").join("");
    let mut char_iter = complete_message.chars();

    let mut m = Marker::new(4);

    while !m.is_valid_marker() {
        let c = char_iter.next().unwrap();
        m.push_symbol(c);
    }

    println!("{:?}", m);
}
fn day_6_2() {
    println!("== Day 6, Task 2 ==");
    let complete_message = utils::read_input_strings("_inputs/input-06-01.txt").join("");
    let mut char_iter = complete_message.chars();

    let mut m = Marker::new(14);

    while !m.is_valid_marker() {
        let c = char_iter.next().unwrap();
        m.push_symbol(c);
    }

    println!("{:?}", m);
}

#[cfg(test)]
mod test {
    use super::Marker;

    #[test]
    fn test_received_chars_no_marker_1() {
        let mut m = Marker::new(4);
        m.push_symbol('a');
        m.push_symbol('b');
        m.push_symbol('c');

        assert_eq!(false, m.is_valid_marker());
        assert_eq!(3, m.get_received());
    }
    #[test]
    fn test_received_chars_no_marker_2() {
        let mut m = Marker::new(4);
        m.push_symbol('a');
        m.push_symbol('b');
        m.push_symbol('b');
        m.push_symbol('c');

        assert_eq!(false, m.is_valid_marker());
    }

    #[test]
    fn test_received_chars_is_marker() {
        let mut m = Marker::new(4);
        m.push_symbol('a');
        m.push_symbol('b');
        m.push_symbol('c');
        m.push_symbol('d');

        assert_eq!(true, m.is_valid_marker());
    }
}
