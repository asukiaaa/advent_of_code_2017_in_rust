extern crate time;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use time::PreciseTime;

extern crate approx;
extern crate alga;
extern crate num_traits;
extern crate nalgebra as na;
use na::{Dim, DimName, Matrix, MatrixN, Matrix2, Matrix3, Matrix4, Scalar, DefaultAllocator, PermutationSequence};
use na::storage::Storage;
use na::allocator::Allocator;
use num_traits::identities::Zero;
use alga::general::{ClosedAdd, ClosedMul, Inverse};

const ON_VALUE: f32 = 1.;
const OFF_VALUE: f32 = 0.;

type Square = Vec<Vec<f32>>;

#[derive(Debug)]
struct Pattern2_to_3 {
    from: Matrix2<f32>,
    to: Matrix3<f32>,
}

#[derive(Debug)]
struct Pattern3_to_4 {
    from: Matrix3<f32>,
    to: Matrix4<f32>,
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
        let mut result_square: Vec<Vec<Matrix3<f32>>> = vec![];
        let step = self.square[0].len() / 2;
        let reverse_matrix = Matrix3::new(0,0,1,
                                          0,1,0,
                                          1,0,0);
        for y in 0..step {
            let mut result_line: Vec<Matrix3<f32>> = vec![];
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
                let mut square_line: Vec<f32> = vec![];
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
        let mut result_square: Vec<Vec<Matrix4<f32>>> = vec![];
        let step = self.square[0].len() / 3;
        let reverse_matrix = Matrix4::new(0,0,0,1,
                                          0,0,1,0,
                                          0,1,0,0,
                                          1,0,0,0);
        for y in 0..step {
            let mut result_line: Vec<Matrix4<f32>> = vec![];
            for x in 0..step {
                let compare_matrix3 = Matrix3::new(self.square[3*y + 0][3*x + 0], self.square[3*y + 0][3*x + 1], self.square[3*y + 0][3*x + 2],
                                                   self.square[3*y + 1][3*x + 0], self.square[3*y + 1][3*x + 1], self.square[3*y + 1][3*x + 2],
                                                   self.square[3*y + 2][3*x + 0], self.square[3*y + 2][3*x + 1], self.square[3*y + 2][3*x + 2]);
                let mut result_matrix4 = Matrix4::zeros();
                for pattern3_to_4 in self.patterns3.iter() {
                    let match_result = match_type(&pattern3_to_4.from, &compare_matrix3);
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
                let mut square_line: Vec<f32> = vec![];
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
                if *element == 1. {
                    sum_number += 1;
                }
            }
        }
        sum_number
    }
}

// fn create_matrix_to_revert<N, D: Dim>(size: usize) -> MatrixN<N, D>
//     where N: Scalar + Zero + ClosedAdd + ClosedMul,
//           DefaultAllocator: Allocator<N, D, D>,
// {
//     match size {
//         2 => { return Matrix2::new(0,1,
//                                    1,0); },
//         3 => { return Matrix3::new(0,0,1,
//                                    0,1,0,
//                                    1,0,0); },
//         _ => panic!("unexpected size: {}", size)
//     }
// }

fn permute_row<N, D: Dim>(matrix: &mut MatrixN<N, D>)
    where N: Scalar + Zero + ClosedAdd + ClosedMul,
          DefaultAllocator: Allocator<N, D, D>,
{
    let mut cloned_m = matrix.clone();
    let size = matrix.nrows();
    for y in 0..size {
        for (x, elem) in matrix.row_mut(y).iter().enumerate() {
            println!("elem: {:?}", elem);
            //println!("is f32: {:?}", N::is::<f32>());
            //*elem = *elem.inc();
            //let mut zero;
            //let mut one;
            let val = cloned_m.row(size - 1 - y)[x];
            *elem = val;
        }
    }
}

// fn match_type<N, R: Dim, C: Dim, S>(matrix1: &Matrix<N, R, C, S>, matrix2: &Matrix<N, R, C, S>) -> Option<String>
//     where N: Scalar + Zero + ClosedAdd + ClosedMul,
//           S: Storage<N, R, C>,
fn match_type<N, D: Dim>(matrix1: &MatrixN<N, D>, matrix2: &MatrixN<N, D>) -> Option<String>
    where N: Scalar + Zero + ClosedAdd + ClosedMul,
          DefaultAllocator: Allocator<N, D, D>,
{
    let mut action:String = String::new();
    if *matrix1 == *matrix2 {
        return Some(action);
    }
    let mut matrix1 = matrix1.clone();
    // matrix_to_revert = create_matrix_to_revert(matrix1.nrows());
    // let matrix_to_revert = MatrixN<N, D>::from(Matrix2::new(0,1,
    //                                                         1,0));
    // create matrix like
    // 0 0 .. 0 1
    // 0 0 .. 1 0
    // ..
    // 0 1 .. 0 0
    // 1 0 .. 0 0
    let mut matrix_to_revert = matrix1.clone();
    let size = matrix_to_revert.nrows();
    // match size {
    //     2 => { matrix_to_revert = Matrix2::new(0,1,
    //                                            1,0); },
    //     3 => { matrix_to_revert = Matrix3::new(0,0,1,
    //                                            0,1,0,
    //                                            1,0,0); },
    //     _ => panic!("unexpected size: {}", size)
    // }
    // for y in 0..size {
    //     for (x, elem) in matrix_to_revert.row_mut(y).iter().enumerate() {
    //         println!("elem: {:?}", elem);
    //         //println!("is f32: {:?}", N::is::<f32>());
    //         //*elem = *elem.inc();
    //         //let mut zero;
    //         //let mut one;
    //         if x + y == size - 1 {
    //             // set 1
    //             *elem = ON_VALUE;
    //             // *elem = *elem.clone();
    //         } else {
    //             // set 0
    //             *elem = OFF_VALUE;
    //         }
    //     }
    // }

    //let lu = matrix1.full_piv_lu();
    //println!("{:?}", lu);
    //let (p, l, u, q) = lu.unpack();

    // let matrix_to_revert = MatrixN<f32, matrix1.dimension()>::zero();
    // let matrix_to_revert = Matrix2::zero();
    // let matrix_to_revert = Matrix2::new(0, 1,
    //                                     1, 0);
    for i in 0..8 {
        if i % 2 == 0 {
            //matrix1 = matrix1.refrect();
            //matrix1 = matrix1.dot(&matrix_to_revert);
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

fn match_type2(matrix1: &Matrix2<f32>, matrix2: &Matrix2<f32>) -> Option<String> {
    let mut action:String = String::new();
    if *matrix1 == *matrix2 {
        return Some(action);
    }
    let mut matrix1 = matrix1.clone();
    let matrix_to_revert = Matrix2::new(0., 1.,
                                        1., 0.);
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

fn match_type3(matrix1: &Matrix3<f32>, matrix2: &Matrix3<f32>) -> Option<String> {
    let mut action:String = String::new();
    if *matrix1 == *matrix2 {
        return Some(action);
    }
    let mut matrix1 = matrix1.clone();
    let matrix_to_revert = Matrix3::new(0., 0., 1.,
                                        0., 1., 0.,
                                        1., 0., 0.);
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

    for i in 0..1 {
    // for i in 0..18 {
        canvas.update();
        canvas.print_canvas();
        println!("loop in {}", i);
    }

    let mut matrix = Matrix2::new(1.,2.,
                                  3.,4.);
    let mut matrix_to_revert = Matrix2::new(0.,1.,
                                            1.,0.);
    println!("{:?}", matrix);
    println!("{:?}", matrix.transpose());
    println!("{:?}", matrix.inverse());
    println!("m x m_to_revert\n{:?}", matrix * matrix_to_revert);
    //println!("{:?}", PermutationSequence::permute_rows(&matrix));

    let lu = matrix.full_piv_lu();
    //println!("{:?}", lu);
    let (p, l, u, q) = lu.unpack();
    let mut lu = l*u;
    // println!("{:?}", lu);
    p.permute_rows(&mut matrix);
    println!("{:?}", matrix);
    p.permute_rows(&mut matrix);
    println!("{:?}", matrix);
    p.permute_columns(&mut matrix);
    println!("{:?}", matrix);
    p.permute_columns(&mut matrix);
    println!("{:?}", matrix);
    println!("{:?}", p);

    let end = PreciseTime::now();
    // println!("{:?}", canvas);
    // canvas.print_canvas();
    println!("true count: {}", canvas.true_count());
    println!("{} seconds", start.to(end));
}
