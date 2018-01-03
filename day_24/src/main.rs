extern crate time;
use time::PreciseTime;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

#[derive(Debug)]
struct PortChain {
    ports: Vec<Vec<u32>>,
    best_chain: Vec<usize>,
    best_point: u32,
    longest_best_chain: Vec<usize>,
    longest_best_point: u32,
    current_chain: Vec<usize>,
    current_point: u32,
    current_port_number: u32,
}

fn find_port(ports: &Vec<Vec<u32>>, start_index: usize, target_port_num: u32, reserved_index: &Vec<usize>) -> Option<usize> {
    for (i, port) in ports.iter().enumerate() {
        // Want to start iter from start_index
        // println!("{:?} {:?}", i, reserved_index);
        if i < start_index || reserved_index.iter().find(|&&ri| ri == i) != None {
            continue;
        }
        if port.iter().position(|&n| n == target_port_num) != None {
            // println!("return {:?}", i);
            return Some(i);
        }
    }
    // println!("return none");
    None
}

fn get_last_port_number(ports: &Vec<Vec<u32>>, chain: &Vec<usize>) -> u32 {
    if ports.len() == 0 {
        panic!("zero length chain was put to get_last_port_number");
    }
    let last_port = ports.last().unwrap();
    if ports.len() == 1 {
        let number = last_port[0];
        if number == 0 {
            return last_port[1];
        } else {
            return number;
        }
    }
    // // search with using last 2 ports
    // let before_last_port = ports.get(ports.len() - 2);
    // let last_port_number = None;
    // for before_number in &before_last_port {
    // }
    // if before_last_port.iter().find(|&&number| number != before_last_port)
    let mut last_port_number = 0;
    for port_index in chain.iter() {
        let port = &ports[*port_index];
        let number = port.iter().find(|&&number| number != last_port_number);
        if number != None {
            last_port_number = *number.unwrap();
        }
    }
    // println!("last port number {:?}", last_port_number);
    last_port_number
}

fn calc_chain_points(ports: &Vec<Vec<u32>>, chain: &Vec<usize>) -> u32 {
    let mut chain_points: u32 = 0;
    for index in chain.iter() {
        let port = &ports[*index];
        chain_points = chain_points + &port[0] + &port[1];
    }
    chain_points
}

fn get_next_child_chain(ports: &Vec<Vec<u32>>, current_chain: &Vec<usize>, last_port_number: u32) -> Option<Vec<usize>> {
    let mut next_chain = current_chain.clone();
    let mut next_last_port_number = last_port_number;
    loop {
        // println!("next chain {:?}", next_chain);
        // println!("next last port num {:?}", next_last_port_number);
        let next_port_index = find_port(&ports, 0, next_last_port_number, &next_chain);
        if next_port_index == None {
            if next_chain.len() == current_chain.len() {
                return None;
            } else {
                return Some(next_chain);
            }
        }
        let next_port_index = next_port_index.unwrap();
        let next_port = &ports[next_port_index];
        next_chain.push(next_port_index);
        let number = next_port[0];
        if number == next_last_port_number {
            next_last_port_number = next_port[1];
        } else {
            next_last_port_number = number;
        }
    }
}

fn get_next_parent_chain(ports: &Vec<Vec<u32>>, current_chain: &Vec<usize>, last_port_number: u32) -> Option<Vec<usize>> {
    let mut next_chain = current_chain.clone();
    let last_port_index = next_chain.pop();
    if last_port_index == None {
        return None;
    }
    let last_port_index = last_port_index.unwrap();
    let last_port = &ports[last_port_index];
    let mut next_last_port_number = last_port[0];
    if next_last_port_number == last_port_number {
        next_last_port_number = last_port[1];
    }
    let next_port_index = find_port(&ports, last_port_index + 1, next_last_port_number, &next_chain);
    if next_port_index == None {
        return get_next_parent_chain(&ports, &next_chain, next_last_port_number);
    } else {
        next_chain.push(next_port_index.unwrap());
        return Some(next_chain);
    }
}

fn get_next_chain(ports: &Vec<Vec<u32>>, current_chain: &Vec<usize>, last_port_number: u32) -> Option<Vec<usize>> {
    let next_child_chain = get_next_child_chain(&ports, &current_chain, last_port_number);
    if next_child_chain != None {
        return next_child_chain;
    }
    return get_next_parent_chain(&ports, &current_chain, last_port_number);
}

fn create_port_chain(ports: &Vec<Vec<u32>>, current_chain: &Vec<usize>) -> Vec<Vec<u32>> {
    let mut port_chain: Vec<Vec<u32>> = Vec::new();
    for i in current_chain.iter() {
        let port = &ports[*i];
        port_chain.push(port.clone());
    }
    port_chain
}

impl PortChain {
    fn new(ports: Vec<Vec<u32>>) -> PortChain {
        let initial_index = find_port(&ports, 0, 0, &vec![]);
        if initial_index == None {
            panic!("No initial port");
        }
        let initial_index = initial_index.unwrap();
        let initial_port = &ports.clone()[initial_index];
        let mut initial_number = initial_port[0];
        if initial_number == 0 {
            initial_number = initial_port[1];
        }
        let initial_chain = get_next_chain(&ports, &vec![initial_index], initial_number).unwrap();
        let initial_point = calc_chain_points(&ports, &initial_chain);
        let port_chain = PortChain {
            ports: ports,
            best_chain: initial_chain.clone(),
            best_point: initial_point,
            longest_best_chain: initial_chain.clone(),
            longest_best_point: initial_point,
            current_chain: initial_chain,
            current_point: initial_point,
            current_port_number: initial_number,
        };
        port_chain
    }
    fn search_next_chain(&mut self) -> bool {
        let next_chain =  get_next_chain(&self.ports, &self.current_chain, self.current_port_number);
        if next_chain == None {
            return false;
        }
        self.current_chain = next_chain.unwrap();
        self.current_point = calc_chain_points(&self.ports, &self.current_chain);
        self.current_port_number = get_last_port_number(&self.ports, &self.current_chain);
        // println!("current chain {:?}", self.current_chain);
        if self.current_point > self.best_point {
            self.best_chain = self.current_chain.clone();
            self.best_point = self.current_point;
        }
        if self.current_chain.len() >= self.longest_best_chain.len() && self.current_point > self.longest_best_point {
            self.longest_best_chain = self.current_chain.clone();
            self.longest_best_point = self.current_point;
        }
        true
    }
}

fn main() {
    let start = PreciseTime::now();
    let file_name = "./data/ports.txt";
    //let file_name = "./data/example.txt";
    let file = BufReader::new(File::open(file_name).unwrap());

    //let ports: Vec<Vec<&str>> = file.lines().map(|l| l.unwrap().split("/").collect::<Vec<&str>>()).collect();
    // let ports: Vec<String> = file.lines().map(|l| l.unwrap()).collect();
    // let ports: Vec<Vec<&str>> = ports.iter().map(|p| p.split("/").collect()).collect();
    // let ports: Vec<Vec<u32>> = ports.iter().map(|p| p.iter().map(|s| s.parse::<u32>().unwrap()).collect()).collect();
    let ports: Vec<Vec<u32>> = file.lines().map(
        |l| l.unwrap().split("/").map(
            |s| s.parse::<u32>().unwrap()
        ).collect()
    ).collect();

    let mut port_chain = PortChain::new(ports);
    while port_chain.search_next_chain() {}
    println!("{:?}", port_chain);
    println!("best_chain: {:?}", &create_port_chain(&port_chain.ports, &port_chain.best_chain));
    println!("longest_best_chain: {:?}", &create_port_chain(&port_chain.ports, &port_chain.longest_best_chain));
    println!("best point {:?}", port_chain.best_point);
    println!("logest best point {:?}", port_chain.longest_best_point);
    let end = PreciseTime::now();
    println!("{} seconds", start.to(end));
}
