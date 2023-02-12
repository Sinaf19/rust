use std::f64::consts::E;
use crate::library::matrix::Matrix;

pub struct Activation<'a> {
    pub function: &'a dyn Fn(f64) -> f64,
    pub derivative: &'a dyn Fn(f64) -> f64,

}

pub const ReLU: Activation = Activation {
    function: &|x| return if x > 0.0 {x} else {0.0},
    derivative: &|x| return (x > 0.0) as i32 as f64,
};

/*pub const SOFTMAX: Activation<Matrix> = Activation {
    function: &|x| return x.data[1][1],
    derivative: &|x| return x.data[1][1],
};*/