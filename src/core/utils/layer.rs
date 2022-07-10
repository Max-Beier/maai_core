use crate::core::utils::neuron::*;
use crate::core::utils::weight::*;

#[derive(Debug)]
pub struct Layer {
    pub index: i32,
    pub height: i32,
    pub neurons: Vec<Neuron>,
    pub weights: Vec<Weight>,
}

impl Layer {
    pub fn new(index: i32) -> Layer {
        return Layer {
            index: index,
            height: 16,
            neurons: Vec::new(),
            weights: Vec::new(),
        };
    }
}
