use crate::library::matrix::Matrix;

pub mod library;

fn main() {

let test;
    let mut test1 = Matrix {rows: 4, cols: 3, data: vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0], vec![7.0, 8.0, 9.0], vec![7.0, 8.0, 9.0]]};
    let test2 = Matrix {rows: 3, cols: 3, data: vec![vec![2.2, 2.3, 2.4], vec![2.5, 2.6, 2.7], vec![2.8, 2.9, 2.0]]};

    test = test1.transpose();

    println!("{:#?}", test.data);

    // need to implement the Mul<_> trait
    // test = test1 * 2;



}
