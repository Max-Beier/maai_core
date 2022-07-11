use rand::Rng;
use std::fs;

use crate::core::layer::Layer;
use crate::core::neuron::Neuron;
use crate::core::weight::Weight;

pub struct Maai {
    pub layer_count: i32,
    layers: Vec<Layer>,
}

impl Maai {
    pub fn get_cost(&self) -> f64 {
        let mut v: f64 = 0.0;
        let mut n: f64 = 0.0;
        for layer in &self.layers {
            for height_index in 0..layer.weights.len() {
                n += height_index as f64;
                v += (layer.weights[height_index].end_neuron.activation - 0.0).sqrt();
            }
        }
        v / n
    }

    pub fn new(input: Vec<f64>) -> Maai {
        let mut rng = rand::thread_rng();
        let mut maai = Maai {
            layer_count: 4,
            layers: Vec::new(),
        };

        for i in 0..maai.layer_count {
            let layer = Layer::new(i);
            maai.layers.push(layer);
        }

        for layer_index in 0..maai.layers.len() {
            for neuron_index in 0..maai.layers[layer_index].height {
                let start_neuron: Neuron;
                let end_neuron: Neuron;
                let mut weight: Weight;

                if layer_index == 0 {
                    if neuron_index < input.len() as i32 {
                        start_neuron = Neuron {
                            activation: input[(neuron_index as usize)],
                            index: neuron_index,
                            layer_index: maai.layers[layer_index].index,
                        }
                    } else {
                        continue;
                    }
                } else {
                    start_neuron = Neuron {
                        activation: rng.gen::<f64>(),
                        index: neuron_index,
                        layer_index: maai.layers[layer_index].index,
                    };
                }

                end_neuron = Neuron {
                    activation: rng.gen::<f64>(),
                    index: neuron_index,
                    layer_index: maai.layers[layer_index].index + 1,
                };

                weight = Weight {
                    index: maai.layers[layer_index].height as u8,
                    value: rng.gen::<f64>(),
                    shift_value: rng.gen::<f64>(),
                    bias: rng.gen::<f64>(),
                    start_neuron: start_neuron,
                    end_neuron: end_neuron,
                    layer_index: maai.layers[layer_index].index,
                };

                let mut sum: f64 = 0.0;
                for weight in &maai.layers[layer_index].weights {
                    sum += weight.value * weight.start_neuron.activation;
                }

                weight.end_neuron.activation += relu(sum - weight.bias);
                maai.layers[layer_index].weights.push(weight);
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

fn get_cache() -> fs::File {
    let file: fs::File;
    if fs::metadata("../cache/").is_err() {
        fs::create_dir("../cache").unwrap();
        file = fs::File::create("../cache/cache.maai").unwrap();
    } else if fs::metadata("../cache/cache.maai").is_err() {
        file = fs::File::create("../cache/cache.maai").unwrap();
    } else {
        file = fs::File::open("../cache/cache.maai").unwrap();
    }
    file
}
