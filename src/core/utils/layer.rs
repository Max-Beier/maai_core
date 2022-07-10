use crate::core::utils::weight::*;

#[derive(Debug)]
pub struct Layer {
    pub index: i32,
    pub height: i32,
    pub weights: Vec<Weight>,
}

impl Layer {
    pub fn new(index_: i32) -> Layer {
        Layer {
            index: index_,
            height: 16,
            weights: Vec::new(),
        }
    }
}
