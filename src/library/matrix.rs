use rand::{Rng, thread_rng};

pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<f64>>,
}

impl Matrix {
    pub fn zero(rows: usize, cols: usize) -> Matrix {
        return Matrix {
            rows,
            cols,
            data: vec![vec![0.0; cols]; rows]
        }
    }

    pub fn random(rows: usize, cols: usize) -> Matrix {
        let mut rng = thread_rng();

        let mut result = Matrix::zero(rows, cols);

        for i in 0..rows {
            for j in 0..cols {
                result.data[i][j] = rng.gen::<f64>() * 2.0 - 1.0;
            }
        }
    return result;
    }

    pub fn add(&mut self, other: &Matrix) -> Matrix {
        if self.rows != other.rows || self.cols != other.cols {
            panic!("Attempting to add a matrix with a matrix of different dimension")
        }

        let mut result = Matrix::zero(self.rows, other.cols);

        for i in 0..self.rows {
            for j in 0..self.cols {
                result.data[i][j] = self.data[i][j] + other.data[i][j];
            }
        }
        return result;
    }

       pub fn multiply(&mut self, other: &Matrix) -> Matrix {
           if self.cols != other.rows {
               panic!("Attempting to multiply by matrix of incorrect dimension")
           }

           let mut result = Matrix::zero(self.rows, other.cols);

           for i in 0..self.rows {
               for j in 0..other.cols {
                   let mut sum = 0.0;
                   for k in 0..self.cols {
                       sum += self.data[i][k] * other.data[k][j];
                   }
                   result.data[i][j] = sum;
               }
           }
           return result;
    }

    pub fn transpose(&mut self) -> Matrix {
        let mut result = Matrix::zero(self.cols, self.rows);

        for i in 0..self.rows {
            for j in 0..self.cols {
                result.data[i][j] = self.data[j][i];
            }
        }
        return result;
    }

}