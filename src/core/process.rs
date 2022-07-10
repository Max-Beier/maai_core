use crate::core::utils::layer::*;
use crate::core::utils::neuron::*;
use crate::core::utils::weight::*;

use rand::Rng;

pub struct Max {
    pub input_payload: Vec<Vec<f64>>,
    pub layerCount: i32,
    pub layers: Vec<Layer>,
}

impl Max {
    pub fn new(input: Vec<Vec<f64>>) -> Max {
        let mut rng = rand::thread_rng();
        let mut max = Max {
            input_payload: input,
            layerCount: 4,
            layers: Vec::new(),
        };

        for i in 0..max.layerCount {
            let layer = Layer::new(i);
            max.layers.push(layer);
        }

        for layerIndex in 0..max.layers.len() {
            for layerHeight in 0..max.layers[layerIndex].height {
                let neuron = Neuron {
                    activation: rng.gen::<f64>(),
                    layerIndex: max.layers[layerIndex].index,
                };

                // NEXT LAYER FOR WEIGHT BRIDGE
                if layerIndex + 1 >= max.layers.len() {
                    break;
                }

                let nextNeuron = Neuron {
                    activation: rng.gen::<f64>(),
                    layerIndex: max.layers[layerIndex + 1].index,
                };

                let mut weight = Weight {
                    index: layerHeight as u8,
                    value: rng.gen::<f64>(),
                    bias: rng.gen::<f64>(),
                    startNeuron: neuron,
                    endNeuron: nextNeuron,
                    layerIndex: max.layers[layerIndex + 1].index,
                };

                let mut sum: f64 = 0.0;
                for weightIndex in 0..max.layers[layerIndex].weights.len() {
                    sum += max.layers[layerIndex].weights[weightIndex].value
                        * max.layers[layerIndex].weights[weightIndex]
                            .startNeuron
                            .activation;
                }

                weight.endNeuron.activation += relu(sum - weight.bias);

                max.layers[layerIndex].weights.push(weight);
            }
        }

        return max;
    }

    pub fn run(&self) {
        println!("{:?}", self.get_cost());
    }

    pub fn get_cost(&self) -> f64 {
        let mut v: f64 = 0.0;
        let mut n: f64 = 0.0;
        for layerIndex in 0..self.layers.len() {
            for heightIndex in 0..self.layers[layerIndex].weights.len() {
                n += heightIndex as f64;
                v += self.layers[layerIndex].weights[heightIndex].value;
            }
        }
        return v / n;
    }
}

fn relu(v: f64) -> f64 {
    if v >= 0.0 {
        v
    } else {
        0.01 * v
    }
}
