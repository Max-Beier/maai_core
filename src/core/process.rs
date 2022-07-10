use crate::core::utils::layer::*;

pub struct Max {
    pub input_payload: Vec<Vec<f64>>,
    pub layerCount: i32,
    pub layers: Vec<Layer>,
}

impl Max {
    pub fn new(input: Vec<Vec<f64>>) -> Max {
        let mut max = Max {
            input_payload: input,
            layerCount: 4,
            layers: Vec::new(),
        };

        for i in 0..max.layerCount {
            let layer = Layer::new(i);
            max.layers.push(layer);
        }

        return max;
    }

    pub fn run(&self) {
        //println!("{:?}", self.layers);
    }
}

fn sigmoid(x: f64) -> f64 {
    return 1.0 / (1.0 + f64::exp(-x));
}
