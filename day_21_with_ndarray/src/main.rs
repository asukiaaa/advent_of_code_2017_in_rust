extern crate time;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use time::PreciseTime;

extern crate ndarray;
use ndarray::prelude::*;
use ndarray::{arr2, Array2, Data, Array};

const ON_VALUE: i32 = 1;
const OFF_VALUE: i32 = 0;

type Square = Vec<Vec<i32>>;

#[derive(Debug)]
struct Canvas {
    square: Square,
    patterns2: Vec<Vec<Array2<i32>>>,
    patterns3: Vec<Vec<Array2<i32>>>,
}

impl Canvas {
    fn new() -> Canvas {
        Canvas {
            square: vec![
                vec![OFF_VALUE, ON_VALUE, OFF_VALUE],
                vec![OFF_VALUE, OFF_VALUE, ON_VALUE],
                vec![ON_VALUE, ON_VALUE, ON_VALUE]],
            patterns2: vec![],
            patterns3: vec![],
        }
    }
    fn load_patterns(&mut self, file_name: &str) {
        let file = BufReader::new(File::open(file_name).unwrap());
        for line in file.lines() {
            let line = line.unwrap();
            let pattern: Vec<Square> = line.split(" => ").map(
                |str| str.split('/').map(
                    |s| s.chars().map(
                        |c| if c == '#' { ON_VALUE } else { OFF_VALUE }
                    ).collect()
                ).collect()
            ).collect();
            let pattern_vec = vec![square_to_arr2(&pattern[0]), square_to_arr2(&pattern[1])];
            if pattern_vec[0].shape()[0] == 2 {
                self.patterns2.push(pattern_vec);
            } else {
                self.patterns3.push(pattern_vec);
            }
        }
    }
    fn update(&mut self) -> bool {
        if self.square.len() % 2 == 0 {
            //println!("expand with 2");
            if self.expand(2) {
                return true;
            }
        }
        if self.square.len() % 3 == 0 {
            //println!("expand with 3");
            if self.expand(3) {
                return true;
            }
        }
        false
    }
    fn expand(&mut self, size: usize) -> bool {
        let mut result_square: Vec<Vec<Array2<i32>>> = vec![];
        let step = self.square[0].len() / size;
        // let reverse_matrix = arr2(&[[0,0,1],
        //                             [0,1,0],
        //                             [1,0,0]]);
        for y in 0..step {
            let mut result_line: Vec<Array2<i32>> = vec![];
            for x in 0..step {
                let mut compare_matrix: Array2<i32>;
                if size == 2 {
                    compare_matrix = arr2(&[[self.square[size*y + 0][size*x + 0], self.square[size*y + 0][size*x + 1]],
                                            [self.square[size*y + 1][size*x + 0], self.square[size*y + 1][size*x + 1]]]);
                } else {
                    compare_matrix = arr2(&[[self.square[size*y + 0][size*x + 0], self.square[size*y + 0][size*x + 1], self.square[size*y + 0][size*x + 2]],
                                            [self.square[size*y + 1][size*x + 0], self.square[size*y + 1][size*x + 1], self.square[size*y + 1][size*x + 2]],
                                            [self.square[size*y + 2][size*x + 0], self.square[size*y + 2][size*x + 1], self.square[size*y + 2][size*x + 2]]]);
                }
                let mut result_matrix = Array2::<i32>::zeros((1,1));
                // println!("compare_matrix: {:?}", compare_matrix);
                let mut patterns: &Vec<Vec<Array2<i32>>>;
                if size == 2 {
                    patterns = &self.patterns2;
                } else {
                    patterns = &self.patterns3;
                }
                for pattern in patterns.iter() {
                    let match_result = match_type(&pattern[0], &compare_matrix);
                    // println!("match result: {:?}", match_result);
                    if match_result == None {
                        continue;
                    }
                    result_matrix = pattern[1].clone();
                    // for command in match_result.unwrap().chars() {
                    //     if command == 'r' {
                    //         result_matrix3 *= reverse_matrix;
                    //     } else {
                    //         result_matrix3 = result_matrix3.transpose();
                    //     }
                    // }
                    break;
                }
                // println!("result_matrix: {:?}", result_matrix);
                if result_matrix == Array2::<i32>::zeros((1,1)) {
                    return false;
                }
                result_line.push(result_matrix);
            }
            result_square.push(result_line);
        }
        let mut next_square: Square = vec![];
        for matrix_line in result_square.iter() {
            for i in 0..size+1 {
                let mut square_line: Vec<i32> = vec![];
                for matrix in matrix_line.iter() {
                    // println!("{}", matrix.row(i));
                    square_line.extend(matrix.row(i).iter());
                }
                next_square.push(square_line);
            }
        }
        self.square = next_square;
        true
    }
    fn print_canvas(&mut self) {
        for row in self.square.iter() {
            println!("{:?}", row);
        }
    }
    fn true_count(&mut self) -> i32 {
        let mut sum_number = 0;
        for line in self.square.iter() {
            for element in line.iter() {
                if *element == 1 {
                    sum_number += 1;
                }
            }
        }
        sum_number
    }
}

fn square_to_arr2(square: &Square) -> Array2<i32> {
    let size = square.len();
    let mut vec: Vec<i32> = vec![];
    for line in square.iter() {
        vec.extend(line);
    }
    //println!("{:?}", vec);
    // println!("{}", size);
    let result = Array::from_shape_vec((size, size), vec);
    // println!("{:?}", result);
    result.unwrap()
}

// fn match_type<A, S, D>(matrix1: &ArrayBase<S, D>, matrix2: &ArrayBase<S, D>) -> Option<String>
//     where S: Data<Elem=A>,
//           D: Dimension,
fn match_type(matrix1: &Array2<i32>, matrix2: &Array2<i32>) -> Option<String>
{
    // println!("matrix1:\n{}", *matrix1);
    // println!("matrix1:\n{:?}", *matrix1.t());
    // println!("matrix2:\n{}", *matrix2);
    let mut action:String = String::new();
    if *matrix1 == *matrix2 {
        return Some(action);
    }
    let mut matrix1 = matrix1.clone();
    // println!("shape:\n{:?}", matrix2.shape());
    let size = matrix2.shape()[0];
    let mut revert_vec: Vec<Vec<i32>> = vec![];
    // for i in 1..size+1 {
    //     let mut line = vec![OFF_VALUE; size];
    //     line[size - i] = ON_VALUE;
    //     revert_vec.push(line);
    // }
    // let matrix_to_revert = Array::from_vec(revert_vec);
    let matrix_to_revert: Array2<i32>;
    if size == 2 {
        matrix_to_revert = arr2(&[[0,1],
                                  [1,0]]);
    } else {
        matrix_to_revert = arr2(&[[0,0,1],
                                  [0,1,0],
                                  [1,0,0]]);
    }
    // println!("matrix to revert: {:?}", matrix_to_revert);
    for i in 0..8 {
        if i % 2 == 0 {
            matrix1 = matrix1.dot(&matrix_to_revert);
            action.push('r');
        } else {
            matrix1 = matrix1.reversed_axes();
            action.push('t');
        }
        // println!("matrix: {} {}", matrix1, action);
        if matrix1 == *matrix2 {
            return Some(action);
        }
    }
    None
}

fn main() {
    let start = PreciseTime::now();

    let mut canvas = Canvas::new();
    let file_name = "./data/input.txt";
    // let file_name = "./data/example.txt";
    canvas.load_patterns(file_name);
    // println!("{:?}", canvas.patterns3[0]);
    for i in 0..18 {
        canvas.update();
        // canvas.print_canvas();
        println!("loop in {}", i);
    }

    let end = PreciseTime::now();
    // println!("{:?}", canvas);
    canvas.print_canvas();
    println!("true count: {}", canvas.true_count());
    println!("{} seconds", start.to(end));
}
