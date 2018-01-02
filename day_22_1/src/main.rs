extern crate time;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use time::PreciseTime;

#[derive(Debug)]
struct InfectionMap {
    map: Vec<Vec<bool>>,
    x: usize,
    y: usize,
    direction: char, // t b l r
    turn_to_infect_count: usize,
}

impl InfectionMap {
    fn new() -> InfectionMap {
        InfectionMap {
            map: vec![vec![]],
            x: 0,
            y: 0,
            direction: 't',
            turn_to_infect_count: 0,
        }
    }
    fn load(&mut self, file_name: &str) {
        let file = BufReader::new(File::open(file_name).unwrap());
        for (line_index, line) in file.lines().enumerate() {
            let line = line.unwrap();
            let line = line.as_str();
            //println!("{:?}", line);
            if self.map.len() <= line_index {
                self.map.push(vec![]);
            }
            for c in line.chars() {
                self.map[line_index].push(c == '#');
            }
        }
        self.y = self.map.len() / 2;
        self.x = self.map[0].len() / 2;
    }
    fn infected_count(&mut self) -> usize {
        let mut count: usize = 0;
        for row in self.map.iter() {
            // count += row.drain_filter(|x| x).collect().count();
            for &x in row {
                if x {
                    count += 1;
                }
            }
        }
        count
    }
    fn action(&mut self) {
        if self.get_current_state() {
            // infected
            self.set_current_state(false);
            self.turn_right();
        } else {
            self.set_current_state(true);
            self.turn_to_infect_count += 1;
            self.turn_left();
        }
        self.go_forward();
    }
    fn turn_left(&mut self) {
        match self.direction {
            't' => self.direction = 'l',
            'l' => self.direction = 'b',
            'b' => self.direction = 'r',
            'r' => self.direction = 't',
            _ => panic!("unexpected direction {}", self.direction),
        }
    }
    fn turn_right(&mut self) {
        match self.direction {
            't' => self.direction = 'r',
            'r' => self.direction = 'b',
            'b' => self.direction = 'l',
            'l' => self.direction = 't',
            _ => panic!("unexpected direction {}", self.direction),
        }
    }
    fn get_current_state(&mut self) -> bool {
        self.map[self.y][self.x]
    }
    fn set_current_state(&mut self, infected: bool) {
        self.map[self.y][self.x] = infected;
    }
    fn go_forward(&mut self) {
        match self.direction {
            't' => self.move_to_top(),
            'l' => self.move_to_left(),
            'b' => self.move_to_bottom(),
            'r' => self.move_to_right(),
            _ => panic!("unexpected direction {}", self.direction),
        }
    }
    fn move_to_top(&mut self) {
        if self.y != 0 {
            self.y -= 1;
            return;
        }
        self.y = self.map.len() - 1;
        let mut empty_map: Vec<Vec<bool>> = vec![vec![false; self.map[0].len()]; self.map.len()];
        empty_map.extend(self.map.clone());
        self.map = empty_map;
    }
    fn move_to_left(&mut self) {
        if self.x != 0 {
            self.x -= 1;
            return;
        }
        self.x = self.map[0].len() - 1;
        for row in self.map.iter_mut() {
            let before_row = row.clone();
            for e in row.iter_mut() {
                *e = false;
            }
            row.extend(before_row);
        }
    }
    fn move_to_bottom(&mut self) {
        self.y += 1;
        if self.y == self.map.len() {
            let empty_map: Vec<Vec<bool>> = vec![vec![false; self.map[0].len()]; self.map.len()];
            self.map.extend(empty_map);
        }
    }
    fn move_to_right(&mut self) {
        self.x += 1;
        if self.x == self.map[0].len() {
            for row in self.map.iter_mut() {
                let row_len = row.len();
                row.extend(vec![false; row_len]);
            }
        }
    }
    fn print_map(&mut self) {
        for row in self.map.iter() {
            let mut row_string = String::new();
            for e in row.iter() {
                if *e {row_string.push('#')}
                else {row_string.push('.')}
            }
            println!("{:?}", row_string);
        }
    }
}

fn main() {
    let start = PreciseTime::now();

    let mut infection_map = InfectionMap::new();
    let file_name = "./data/input.txt";
    //let file_name = "./data/example.txt";

    infection_map.load(file_name);
    for i in 0..10000 {
        infection_map.action();
    }

    let end = PreciseTime::now();
    infection_map.print_map();
    // println!("result: {:?}", infection_map);
    println!("x: {}, y: {}, direction: {}, turn_to_infect_count: {}", infection_map.x, infection_map.y, infection_map.direction, infection_map.turn_to_infect_count);
    println!("infected count: {}", infection_map.infected_count());
    println!("{} seconds", start.to(end));
}
