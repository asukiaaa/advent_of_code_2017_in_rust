extern crate time;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use time::PreciseTime;

extern crate approx;
extern crate nalgebra as na;
use na::{Matrix2, Matrix3, Matrix4,};

const ON_VALUE: i32 = 1;
const OFF_VALUE: i32 = 0;

type Square = Vec<Vec<i32>>;

#[derive(Debug)]
struct Pattern2_to_3 {
    from: Matrix2<i32>,
    to: Matrix3<i32>,
}

#[derive(Debug)]
struct Pattern3_to_4 {
    from: Matrix3<i32>,
    to: Matrix4<i32>,
}

#[derive(Debug)]
struct Canvas {
    square: Square,
    patterns2: Vec<Pattern2_to_3>,
    patterns3: Vec<Pattern3_to_4>,
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
            if pattern[0][0].len() == 2 {
                // Refactoring is needed
                let square = &pattern[0];
                let mut matrix2 = Matrix2::new(square[0][0], square[0][1],
                                               square[1][0], square[1][1]);
                let square = &pattern[1];
                let mut matrix3 = Matrix3::new(square[0][0], square[0][1], square[0][2],
                                               square[1][0], square[1][1], square[1][2],
                                               square[2][0], square[2][1], square[2][2]);
                self.patterns2.push(Pattern2_to_3 {
                    from: matrix2,
                    to: matrix3,
                });
            } else {
                // Refactoring is needed
                let square = &pattern[0];
                let mut matrix3 = Matrix3::new(square[0][0], square[0][1], square[0][2],
                                               square[1][0], square[1][1], square[1][2],
                                               square[2][0], square[2][1], square[2][2]);
                let square = &pattern[1];
                let mut matrix4 = Matrix4::new(square[0][0], square[0][1], square[0][2], square[0][3],
                                               square[1][0], square[1][1], square[1][2], square[1][3],
                                               square[2][0], square[2][1], square[2][2], square[2][3],
                                               square[3][0], square[3][1], square[3][2], square[3][3]);
                self.patterns3.push(Pattern3_to_4 {
                    from: matrix3,
                    to: matrix4,
                });
            }
        }
    }
    fn update(&mut self) -> bool {
        if self.square.len() % 2 == 0 {
            if self.expand_with_2() {
                return true;
            }
        }
        if self.square.len() % 3 == 0 {
            if self.expand_with_3() {
                return true;
            }
        }
        false
    }
    fn expand_with_2(&mut self) -> bool {
        let mut result_square: Vec<Vec<Matrix3<i32>>> = vec![];
        let step = self.square[0].len() / 2;
        let reverse_matrix = Matrix3::new(0,0,1,
                                          0,1,0,
                                          1,0,0);
        for y in 0..step {
            let mut result_line: Vec<Matrix3<i32>> = vec![];
            for x in 0..step {
                let compare_matrix2 = Matrix2::new(self.square[2*y + 0][2*x + 0], self.square[2*y + 0][2*x + 1],
                                                   self.square[2*y + 1][2*x + 0], self.square[2*y + 1][2*x + 1]);
                let mut result_matrix3 = Matrix3::zeros();
                for pattern2_to_3 in self.patterns2.iter() {
                    let match_result = match_type2(&pattern2_to_3.from, &compare_matrix2);
                    if match_result == None {
                        continue;
                    }
                    result_matrix3 = pattern2_to_3.to.clone();
                    // for command in match_result.unwrap().chars() {
                    //     if command == 'r' {
                    //         result_matrix3 *= reverse_matrix;
                    //     } else {
                    //         result_matrix3 = result_matrix3.transpose();
                    //     }
                    // }
                    break;
                }
                if result_matrix3 == Matrix3::zeros() {
                    return false;
                }
                result_line.push(result_matrix3);
            }
            result_square.push(result_line);
        }
        let mut next_square: Square = vec![];
        for matrix_line in result_square.iter() {
            for i in 0..3 {
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
    fn expand_with_3(&mut self) -> bool {
        let mut result_square: Vec<Vec<Matrix4<i32>>> = vec![];
        let step = self.square[0].len() / 3;
        let reverse_matrix = Matrix4::new(0,0,0,1,
                                          0,0,1,0,
                                          0,1,0,0,
                                          1,0,0,0);
        for y in 0..step {
            let mut result_line: Vec<Matrix4<i32>> = vec![];
            for x in 0..step {
                let compare_matrix3 = Matrix3::new(self.square[3*y + 0][3*x + 0], self.square[3*y + 0][3*x + 1], self.square[3*y + 0][3*x + 2],
                                                   self.square[3*y + 1][3*x + 0], self.square[3*y + 1][3*x + 1], self.square[3*y + 1][3*x + 2],
                                                   self.square[3*y + 2][3*x + 0], self.square[3*y + 2][3*x + 1], self.square[3*y + 2][3*x + 2]);
                let mut result_matrix4 = Matrix4::zeros();
                for pattern3_to_4 in self.patterns3.iter() {
                    let match_result = match_type3(&pattern3_to_4.from, &compare_matrix3);
                    if match_result == None {
                        continue;
                    }
                    result_matrix4 = pattern3_to_4.to.clone();
                    // for command in match_result.unwrap().chars() {
                    //     if command == 'r' {
                    //         result_matrix4 *= reverse_matrix;
                    //     } else {
                    //         result_matrix4 = result_matrix4.transpose();
                    //     }
                    // }
                }
                if result_matrix4 == Matrix4::zeros() {
                    return false;
                }
                result_line.push(result_matrix4);
            }
            result_square.push(result_line);
        }
        let mut next_square: Square = vec![];
        for matrix_line in result_square.iter() {
            for i in 0..4 {
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

fn match_type2(matrix1: &Matrix2<i32>, matrix2: &Matrix2<i32>) -> Option<String> {
    let mut action:String = String::new();
    if *matrix1 == *matrix2 {
        return Some(action);
    }
    let mut matrix1 = matrix1.clone();
    let matrix_to_revert = Matrix2::new(0, 1,
                                        1, 0);
    for i in 0..8 {
        if i % 2 == 0 {
            matrix1 *= matrix_to_revert;
            action.push('r');
        } else {
            matrix1 = matrix1.transpose();
            action.push('t');
        }
        // println!("matrix: {} {}", matrix1, action);
        if matrix1 == *matrix2 {
            return Some(action);
        }
    }
    None
}

fn match_type3(matrix1: &Matrix3<i32>, matrix2: &Matrix3<i32>) -> Option<String> {
    let mut action:String = String::new();
    if *matrix1 == *matrix2 {
        return Some(action);
    }
    let mut matrix1 = matrix1.clone();
    let matrix_to_revert = Matrix3::new(0, 0, 1,
                                        0, 1, 0,
                                        1, 0, 0);
    for i in 0..8 {
        if i % 2 == 0 {
            matrix1 *= matrix_to_revert;
            action.push('r');
        } else {
            matrix1 = matrix1.transpose();
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
    //let file_name = "./data/example.txt";
    canvas.load_patterns(file_name);

    for i in 0..18 {
        canvas.update();
        canvas.print_canvas();
        println!("loop in {}", i);
    }

    let end = PreciseTime::now();
    println!("{:?}", canvas);
    canvas.print_canvas();
    println!("true count: {}", canvas.true_count());
    println!("{} seconds", start.to(end));
}
