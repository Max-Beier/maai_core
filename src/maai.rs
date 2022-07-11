use std::fs::File;
use std::io::prelude::*;

use rand::Rng;

use crate::core::layer::Layer;
use crate::core::neuron::Neuron;
use crate::core::weight::Weight;

pub struct Maai {
    pub input_payload: Vec<Vec<f64>>,
    pub layer_count: i32,
    pub layers: Vec<Layer>,
}

impl Maai {
    pub fn get_cost(&self) -> f64 {
        let mut v: f64 = 0.0;
        let mut n: f64 = 0.0;
        for layer_index in 0..self.layers.len() {
            for height_index in 0..self.layers[layer_index].weights.len() {
                n += height_index as f64;
                v += self.layers[layer_index].weights[height_index].value;
            }
        }
        v / n
    }

    pub fn new(input: Vec<Vec<f64>>) -> Maai {
        let mut rng = rand::thread_rng();
        let mut maai = Maai {
            input_payload: input,
            layer_count: 4,
            layers: Vec::new(),
        };

        for i in 0..maai.layer_count {
            let layer = Layer::new(i);
            maai.layers.push(layer);
        }

        for layer_index in 0..maai.layers.len() {
            for neuron_index in 0..maai.layers[layer_index].height {
                let neuron = Neuron {
                    activation: rng.gen::<f64>(),
                    index: neuron_index,
                    layer_index: maai.layers[layer_index].index,
                };

                let next_neuron = Neuron {
                    activation: rng.gen::<f64>(),
                    index: neuron_index,
                    layer_index: maai.layers[layer_index].index + 1,
                };

                let mut weight = Weight {
                    index: maai.layers[layer_index].height as u8,
                    value: rng.gen::<f64>(),
                    bias: rng.gen::<f64>(),
                    start_neuron: neuron,
                    end_neuron: next_neuron,
                    layer_index: maai.layers[layer_index].index + 1,
                };

                let mut sum: f64 = 0.0;
                for weight in &maai.layers[layer_index].weights {
                    sum += weight.value * weight.start_neuron.activation;
                }

                weight.end_neuron.activation += relu(sum - weight.bias);
                maai.layers[layer_index].weights.push(weight);

                let mut file =
                    File::open("./data/data.maai").expect("Can't open './data/data.maai'");

                _ = file.write(b"maai.layers").unwrap();
            }
        }
        maai
    }

    pub fn run(&self) {
        println!("{:?}", self.get_cost());
    }
}

fn relu(v: f64) -> f64 {
    if v >= 0.0 {
        v
    } else {
        0.01 * v
    }
}
