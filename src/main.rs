use std::f64::consts::E;
use crate::library::matrix::Matrix;

pub mod library;

fn main() {

let mut test;
    let mut test1 = Matrix {rows: 4, cols: 3, data: vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0], vec![7.0, 8.0, 9.0], vec![7.0, 8.0, 9.0]]};
    let mut test2 = Matrix {rows: 3, cols: 3, data: vec![vec![2.2, 2.3, 2.4], vec![2.5, 2.6, -2.7], vec![-2.8, 2.9, -2.0]]};

    test = test1.transpose();

    // println!("{:#?}", test.data);

    test = test2.map(&ReLU);

    println!("{:?}", test.data);

    test = softmax(test);

    println!("{:?}", test.data);

    // let result = softmax(&[2.0, 3.0, 1.0, -0.5]);
}

fn ReLU(value: f64) -> f64 {
    return if value > 0.0 {
        value
    } else {
        0.0
    }
}

fn softmax (mut m:  Matrix) -> Matrix {
    m = m.map(&|x| x.exp());

    let mut v = vec![0.0; m.cols];
    for i in 0..m.rows {
        let cm = m.column(i);
        let sum = cm.iter().fold(0.0, |sum, val| sum + val);
        v[i] = sum;
    }

    // apply the division to each value in the matrix, the sum must correspond to the col !
    let mut matrix = vec![vec![0.0; m.cols]; m.rows];
    for i in 0..m.rows {
        for j in 0..m.cols {
            m.data[i][j] = m.data[i][j] / v[j];
        }
/*            let row = m.row(i);
            // should give the sum for each column and not each rows :(
            let result = row.iter().map(|x| x / v[i]).collect();
            matrix[i] = result;*/
    }

    return m;
}