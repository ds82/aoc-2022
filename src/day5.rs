use crate::utils;
use itertools::Itertools;
use std::error::Error;
/*
*/

#[derive(Debug)]
struct Crane {
    load: Vec<char>,
}

impl Crane {
    fn new() -> Crane {
        Crane { load: vec![] }
    }

    fn push_cargo(&mut self, cargo: char) -> Result<(), Box<dyn Error>> {
        self.load.push(cargo);
        Ok(())
    }

    fn view_last_cargo(&self) -> char {
        *self.load.last().unwrap_or(&' ')
    }

    fn pop_cargo(&mut self) -> char {
        self.load.pop().unwrap_or(' ')
    }
}

#[derive(Debug)]
struct State {
    cranes: usize,
    crane: Vec<Crane>,
}

impl State {
    fn new(number_of_cranes: usize) -> State {
        let crane_vec: Vec<Crane> = (0..number_of_cranes).map(|_| Crane::new()).collect();

        State {
            cranes: number_of_cranes,
            crane: crane_vec,
        }
    }

    // fn get_crane(&self, crane_number: usize) -> &mut Crane {
    //     self.crane
    //         .get_mut(crane_number - 1)
    //         .expect("Tried to access Crane out of bounds")
    // }

    fn load(&mut self, crane_number: usize, cargo: char) -> Result<(), Box<dyn Error>> {
        if self.cranes >= crane_number {
            self.crane[crane_number - 1].push_cargo(cargo);
            return Ok(());
        }

        todo!();
    }

    fn topline(&self) -> Vec<char> {
        self.crane.iter().map(|c| c.view_last_cargo()).collect()
    }

    fn do_op(&mut self, op: Op) -> Result<(), Box<dyn Error>> {
        for _ in 0..op.number_of_cargos {
            let cargo = self.crane.get_mut(op.from - 1).unwrap().pop_cargo();
            self.crane.get_mut(op.to - 1).unwrap().push_cargo(cargo);
        }

        Ok(())
    }

    fn do_op_multiple(&mut self, op: Op) -> Result<(), Box<dyn Error>> {
        let mut lifted_cargos: Vec<char> = vec![];

        for _ in 0..op.number_of_cargos {
            let cargo = self.crane.get_mut(op.from - 1).unwrap().pop_cargo();
            lifted_cargos.push(cargo);
        }

        lifted_cargos.into_iter().rev().for_each(|c| {
            self.crane
                .get_mut(op.to - 1)
                .unwrap()
                .push_cargo(c)
                .unwrap()
        });

        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
struct Op {
    number_of_cargos: usize,
    from: usize,
    to: usize,
}

impl Op {
    fn parse(line: String) -> Op {
        let (number_of_cargos, from, to): (usize, usize, usize) = line
            .split(" ")
            .map(|value| value.parse::<usize>().unwrap_or(0))
            .filter(|x| x > &0)
            .collect_tuple()
            .unwrap();

        Op {
            number_of_cargos,
            from,
            to,
        }
    }
}

const START_STATE: &str = r#"
[M]                     [N] [Z]
[F]             [R] [Z] [C] [C]
[C]     [V]     [L] [N] [G] [V]
[W]     [L]     [T] [H] [V] [F] [H]
[T]     [T] [W] [F] [B] [P] [J] [L]
[D] [L] [H] [J] [C] [G] [S] [R] [M]
[L] [B] [C] [P] [S] [D] [M] [Q] [P]
[B] [N] [J] [S] [Z] [W] [F] [W] [R]
 1   2   3   4   5   6   7   8   9
"#;

fn get_initial_state() -> State {
    let mut state = State::new(9);

    //
    // init start situation
    //
    START_STATE.trim().lines().rev().skip(1).for_each(|line| {
        line.chars().enumerate().for_each(|(idx, c)| {
            let current_crane = (idx / 4) + 1;

            match c {
                'A'..='Z' => {
                    state.load(current_crane, c);
                    // println!("load [{}] to crane #{}", c, current_crane);
                }
                _ => {}
            }
        })
    });

    state
}

fn day_5_1() {
    println!("== Day 5, Task 1 ==");

    let mut state = get_initial_state();

    let topline = state.topline();
    println!("topline after initial load: {:?}", topline);

    let operations: Vec<Op> = utils::read_input_strings("_inputs/input-05-01.txt")
        .iter()
        .map(|line| Op::parse(line.clone()))
        .collect();

    operations.iter().for_each(|op| state.do_op(*op).unwrap());

    let topline = state.topline().into_iter().collect::<String>();
    println!("topline after operations: {:?}", topline);
}

fn day_5_2() {
    println!("== Day 5, Task 2 ==");

    let mut state = get_initial_state();

    let topline = state.topline();
    println!("topline after initial load: {:?}", topline);

    let operations: Vec<Op> = utils::read_input_strings("_inputs/input-05-01.txt")
        .iter()
        .map(|line| Op::parse(line.clone()))
        .collect();

    operations
        .iter()
        .for_each(|op| state.do_op_multiple(*op).unwrap());

    let topline = state.topline().into_iter().collect::<String>();
    println!("topline after operations: {:?}", topline);
}

pub fn run() {
    day_5_1();
    day_5_2();
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_cargo_transfer_1() {
        let mut state = get_initial_state();
        state.do_op(Op {
            number_of_cargos: 3,
            from: 1,
            to: 2,
        });
        state.do_op(Op {
            number_of_cargos: 1,
            from: 7,
            to: 9,
        });
        state.do_op(Op {
            number_of_cargos: 2,
            from: 8,
            to: 4,
        });

        assert_eq!("WCVCRZCVN", state.topline().into_iter().collect::<String>());
    }

    #[test]
    fn test_op_parse() {
        let op = Op::parse("move 5 from 3 to 6".into());

        assert_eq!(5, op.number_of_cargos);
        assert_eq!(3, op.from);
        assert_eq!(6, op.to);
    }
}
