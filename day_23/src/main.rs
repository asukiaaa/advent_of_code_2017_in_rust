extern crate time;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use time::PreciseTime;


#[derive(Debug)]
struct Registors {
    a: i32,
    b: i32,
    c: i32,
    d: i32,
    e: i32,
    f: i32,
    g: i32,
    h: i32,
    mul_count: i32,
}

impl Registors {
    fn new() -> Registors {
        Registors {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            g: 0,
            h: 0,
            mul_count: 0,
        }
    }
    fn execute(&mut self, commands: &Vec<Vec<&str>>, command_index: &mut usize) {
        // println!("commands: {:?}", commands);
        let command = &commands[*command_index];
        // println!("index: {:?}", command_index);
        // println!("command: {:?}", command);
        match command[0] {
            "sub" => self.sub(command_index, command[1], command[2]),
            "set" => self.set(command_index, command[1], command[2]),
            "mul" => self.mul(command_index, command[1], command[2]),
            "jnz" => self.jnz(commands, command_index, command[1], command[2]),
            _ => println!("{:?}", command[0])
            //_ => panic!("unexpected command: {:?}", command[0])
        }
    }
    fn jnz(&mut self, commands: &Vec<Vec<&str>>, command_index: &mut usize, target: &str, value: &str) {
        // println!("in jnz");
        let target = self.value(target);
        if target == 0 {
            *command_index += 1;
            //println!("return");
            return;
        }
        // println!("value: {}", value);
        let value = self.value(value);
        // println!("value: {}", value);
        let mut new_command_index = *command_index as i32;
        new_command_index += value;
        // println!("new_command_index: {}", new_command_index);
        if new_command_index < 0 {
            panic!("index become minus");
        }
        *command_index = new_command_index as usize;
        if *command_index < commands.len() {
            self.execute(commands, command_index);
        }
    }
    fn registor(&mut self, target: &str) -> Option<&mut i32> {
        match target {
            "a" => Some(&mut self.a),
            "b" => Some(&mut self.b),
            "c" => Some(&mut self.c),
            "d" => Some(&mut self.d),
            "e" => Some(&mut self.e),
            "f" => Some(&mut self.f),
            "g" => Some(&mut self.g),
            "h" => Some(&mut self.h),
            _ => None
        }
    }
    fn value(&mut self, value: &str) -> i32 {
        let registor = self.registor(value);
        if registor == None {
            return value.parse::<i32>().unwrap();
        }
        *registor.unwrap()
    }
    fn sub(&mut self, command_index: &mut usize, target: &str, value: &str) {
        let value = self.value(value);
        let registor = self.registor(target).unwrap();
        *registor -= value;
        *command_index += 1;
    }
    fn set(&mut self, command_index: &mut usize, target: &str, value: &str) {
        let value = self.value(value);
        let registor = self.registor(target).unwrap();
        *registor = value;
        *command_index += 1;
    }
    fn mul(&mut self, command_index: &mut usize, target: &str, value: &str) {
        self.mul_count += 1;
        let value = self.value(value);
        let registor = self.registor(target).unwrap();
        *registor *= value;
        *command_index += 1;
    }
}

fn main() {
    let start = PreciseTime::now();

    let file_name = "./data/input.txt";
    let file = BufReader::new(File::open(file_name).unwrap());
    // let mut commands: Vec<Command> = Vec::new();
    // for line in file.lines() {
    //     let line = line.unwrap();
    //     let data: Vec<&str> = line.split_whitespace().collect();
    //     println!("{:?}", data);
    //     commands.push(Command {key: data[0].to_string(), target: data[1].to_string(), value: data[2].parse::<i32>().unwrap()})
    // }
    let commands: Vec<String> = file.lines().map(|l| l.unwrap()).collect();
    let commands: Vec<Vec<&str>> = commands.iter().map(|c| c.split_whitespace().collect()).collect();
    let mut registors = Registors::new();

    let mut current_index: usize = 0;
    let commands_len = commands.len();
    while current_index < commands_len {
        // println!("{}", current_index);
        registors.execute(&commands, &mut current_index);
    }

    println!("{:?}", registors);
    let end = PreciseTime::now();
    println!("{} seconds", start.to(end));
}
