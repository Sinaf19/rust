use crate::library::matrix::Matrix;
use crate::library::activation::Activation;

pub struct Network<'a> {
    layers: Vec<usize>,
    weights: Vec<Matrix>,
    biases: Vec<Matrix>,
    data: Vec<Matrix>,
    activation: Activation<'a>,

}

impl Network<'_> {
    pub fn new<'a>(layers: Vec<usize>, activation: Activation<'a>) -> Network {
        let mut weights = vec![];
        let mut biases = vec![];

        for i in 0..layers.len() - 1 {
            weights.push(Matrix::random(layers[i + 1], layers[i]));
            biases.push(Matrix::random(layers[i + 1], 1));
        }

        Network {
            layers,
            weights,
            biases,
            data: vec![],
            activation,
        }
    }

    pub fn feed_forward(&mut self, input: Vec<f64>) -> Vec<f64> {
        if input.len() != self.layers[0] {
            panic!("Non-valid number of inputs");
        }

        let mut current = Matrix::from(vec![input]).transpose();

        for i in 0..self.layers.len() - 1 {
            current = self.weights[i]
                .multiply(&current)
                .add(&self.biases[i])
                .map(self.activation.function)
        }

        return vec![];
    }

}