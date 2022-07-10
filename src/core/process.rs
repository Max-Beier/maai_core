use crate::core::utils::layer::*;
use crate::core::utils::neuron::*;
use crate::core::utils::weight::*;

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

        for layerIndex in 0..max.layers.len() {
            for layerHeight in 0..max.layers[layerIndex].height {
                let neuron = Neuron {
                    activation: 0.0,
                    sum: 0.0,
                    layerIndex: max.layers[layerIndex].index,
                };

                // NEXT LAYER FOR WEIGHT BRIDGE
                if layerIndex + 1 >= max.layers.len() {
                    break;
                }

                let nextNeuron = Neuron {
                    activation: 0.0,
                    sum: 0.0,
                    layerIndex: max.layers[layerIndex + 1].index,
                };

                let weight = Weight {
                    value: 0.0,
                    startNeuron: neuron,
                    endNeuron: nextNeuron,
                    layerIndex: max.layers[layerIndex + 1].index,
                };

                max.layers[layerIndex].weights.push(weight);
            }
        }

        return max;
    }

    pub fn run(&self) {
        println!("{:?}", self.layers);
    }
}

fn sigmoid(x: f64) -> f64 {
    return 1.0 / (1.0 + f64::exp(-x));
}
