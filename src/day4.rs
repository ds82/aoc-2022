use crate::utils;
use itertools::Itertools;

pub fn run() {
    day_4_1();
    day_4_2();
}

struct Task(usize, usize);

struct ElvePair {
    left: Task,
    right: Task,
}

impl Task {
    pub fn parse(line: String) -> Task {
        let (low, high): (usize, usize) = line
            .split("-")
            .take(2)
            .map(|x| x.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();

        Task(low, high)
    }

    pub fn contains(&self, t: &Task) -> bool {
        let Task(a, b) = *self;
        let Task(x, y) = *t;

        a <= x && b >= y
    }

    pub fn overlaps(&self, t: &Task) -> bool {
        let Task(a, b) = *self;
        let Task(x, y) = *t;

        (x >= a && x <= b) || (y >= a && y <= b)
    }
}

impl ElvePair {
    pub fn parse(line: String) -> ElvePair {
        let (left, right): (Task, Task) = line
            .split(",")
            .take(2)
            .map(|x| Task::parse(x.into()))
            .collect_tuple()
            .unwrap();

        ElvePair { left, right }
    }

    pub fn one_contains_the_other(&self) -> bool {
        self.left.contains(&self.right) || self.right.contains(&self.left)
    }

    pub fn overlaps(&self) -> bool {
        self.left.overlaps(&self.right) || self.right.overlaps(&self.left)
    }
}

fn day_4_1() {
    let lines = utils::read_input_strings("_inputs/input-04-01.txt");

    let fully_contains_the_other = lines
        .iter()
        .map(|x| ElvePair::parse(x.into()))
        .filter(|elve_pair| elve_pair.one_contains_the_other())
        .count();

    println!(
        "Pairs where one fully contains the other: {}",
        fully_contains_the_other
    );
}

fn day_4_2() {
    let lines = utils::read_input_strings("_inputs/input-04-01.txt");

    let overlaps = lines
        .iter()
        .map(|x| ElvePair::parse(x.into()))
        .filter(|elve_pair| elve_pair.overlaps())
        .count();

    println!("Paris that overlap each oterh: {}", overlaps)
}
