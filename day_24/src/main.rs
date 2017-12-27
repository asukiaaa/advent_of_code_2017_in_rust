use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

#[derive(Debug)]
struct PortChain {
    ports: Vec<Vec<u32>>,
    best_chain: Vec<usize>,
    best_point: u32,
    searched_chain: Vec<usize>,
    current_chain: Vec<usize>,
    current_point: u32,
    current_port_number: u32,
}

fn find_port(ports: Vec<Vec<u32>>, start_index: usize, target_port_num: u32, reserved_index: Vec<usize>) -> Option<usize> {
    let mut sorted_reserved_index = reserved_index.clone();
    sorted_reserved_index.sort();
    for (i, port) in ports.iter().enumerate() {
        // Want to start iter from start_index
        if i < start_index || sorted_reserved_index.binary_search(&i).is_ok() {
            continue;
        }
        if port.iter().position(|&n| n == target_port_num) != None {
            return Some(i);
        }
    }
    None
}

fn calc_chain_points(ports: Vec<Vec<u32>>, chain: Vec<usize>) -> u32 {
    let mut chain_points: u32 = 0;
    for index in chain.iter() {
        let port = ports.get(*index).unwrap();
        let port_points: u32 = port.iter().sum();
        chain_points += port_points;
        //point = point + port.get(0).unwrap() + port.get(1).unwrap();
    }
    chain_points
}

impl PortChain {
    fn new(ports: Vec<Vec<u32>>) -> PortChain {
        let initial_index = find_port(ports.clone(), 0, 0, vec![]);
        if initial_index == None {
            panic!("No initial port");
        }
        let initial_index = initial_index.unwrap();
        let initial_chain = vec![initial_index];
        let initial_point = calc_chain_points(ports.clone(), initial_chain.clone());
        let cloned_port = ports.clone();
        let initial_port = cloned_port.get(initial_index).unwrap();
        let mut initial_number = *initial_port.get(0).unwrap();
        if initial_number == 0 {
            initial_number = *initial_port.get(1).unwrap();
        }
        let port_chain = PortChain {
            ports: ports,
            best_chain: initial_chain.clone(),
            best_point: initial_point,
            searched_chain: initial_chain.clone(),
            current_chain: initial_chain.clone(),
            current_point: initial_point.clone(),
            current_port_number: initial_number,
        };
        port_chain
    }
    fn next_chain(&mut self) -> Option<Vec<usize>> {
        let next_index = find_port(self.ports.clone(), 0, self.current_port_number, self.current_chain.clone());
        let cloned_ports = self.ports.clone();
        if next_index != None {
            let next_index = next_index.unwrap();
            let mut next_chain = self.current_chain.clone();
            next_chain.push(next_index);
            return Some(next_chain);
        } else {
            let mut prev_chain = self.current_chain.clone();
            let prev_last_index = prev_chain.pop();
            if prev_last_index == None {
                return None;
            }
            let prev_last_index = prev_last_index.unwrap();
            let current_index = *self.current_chain.clone().last().unwrap();
            let last_port = cloned_ports.get(current_index).unwrap();
            let mut prev_port_number = *last_port.get(0).unwrap();
            if prev_port_number == self.current_port_number {
                prev_port_number = *last_port.get(1).unwrap();
            }
            let next_index = find_port(self.ports.clone(), prev_last_index, prev_port_number, prev_chain);
        }
        Some(self.current_chain.clone())
    }
}

fn main() {
    let file_name = "./data/ports.txt";
    //let file_name = "./data/example.txt";
    let file = BufReader::new(File::open(file_name).unwrap());

    //let ports: Vec<Vec<&str>> = file.lines().map(|l| l.unwrap().split("/").collect::<Vec<&str>>()).collect();
    let ports: Vec<String> = file.lines().map(|l| l.unwrap()).collect();
    let ports: Vec<Vec<&str>> = ports.iter().map(|p| p.split("/").collect()).collect();
    let ports: Vec<Vec<u32>> = ports.iter().map(|p| p.iter().map(|s| s.parse::<u32>().unwrap()).collect()).collect();

    let mut port_chain = PortChain::new(ports);
    println!("{:?}", port_chain);
    let next_chain = port_chain.next_chain().unwrap();
    println!("{:?}", next_chain);
    let next_chain_last_index = *next_chain.last().unwrap();
    println!("next chain last index {:?}", next_chain_last_index);
    println!("ports {:?}", port_chain.ports);
    println!("ports.len {:?}", port_chain.ports.len());
    let ports = port_chain.ports.clone();
    let next_port = ports.get(next_chain_last_index);
    println!("{:?}", next_port);
}

