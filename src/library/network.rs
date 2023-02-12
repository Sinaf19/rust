use crate::library::matrix::Matrix;
use crate::library::activation::Activation;
use crate::softmax;

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
        self.data = vec![current.clone()];
        for i in 0..self.layers.len() - 2 {
            current = self.weights[i]
                .multiply(&current)
                .add(&self.biases[i])
                .map(self.activation.function);
            self.data.push(current.clone());
        }



        // activation functions are used until the last layer where we need the softmax function
        // to be applied to the matrix to have a vector of output based on probability to
        // each class the input could correspond to
        current = softmax(current.clone());

        return current.data[0].to_owned();
    }

}