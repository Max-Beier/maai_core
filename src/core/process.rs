#[path = "./utils/mod.rs"]
mod utils;

pub struct Max {
    pub input_payload: Vec<Vec<f64>>,
    pub layerCount: i32,
    pub layers: Vec<utils::layer::Layer>,
}

impl Max {
    pub fn new(input: Vec<Vec<f64>>) -> Max {
        let mut max = Max {
            input_payload: input,
            layerCount: 4,
            layers: Vec::new(),
        };

        for i in 0..max.layerCount {
            let layer = utils::layer::Layer::new(i);
            max.layers.push(layer);
        }

        for layer in 0..max.layers.len() {
            if max.layers[layer].index == 0 || max.layers[layer].index == max.layerCount - 1 {
                continue;
            }
            for neuron in 0..max.layers[layer].height {
                for cWeight in 0..max.layers[layer + 1].weights.len() {
                    let mut weight = utils::weight::Weight::new(layer as i32 + 1);
                    max.layers[layer + 1].weights[cWeight] = weight;
                }
            }
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
