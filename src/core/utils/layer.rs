use crate::core::utils::weight::*;

#[derive(Debug)]
pub struct Layer {
    pub index: i32,
    pub height: i32,
    pub weights: Vec<Weight>,
}

impl Layer {
    pub fn new(index: i32) -> Layer {
        return Layer {
            index: index,
            height: 16,
            weights: Vec::new(),
        };
    }
}
