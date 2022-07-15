use std::fs;

use crate::core::inspect::inspect;
use crate::core::layer::Layer;

pub struct Maai {
    pub learning_rate: f64,
    layers: Vec<Layer>,
}

impl Maai {
    /// For the maai system you have to specify how much `Layers` and `Neurons [per Layer]` you would like to use.
    pub fn new(payload: Vec<f64>, layer_count: u32, neuron_count: u32) -> Maai {
        let mut maai = Maai {
            layers: Vec::new(),
            learning_rate: 2.0,
        };

        for index in 0..layer_count {
            let layer_instance = Layer::new(&maai.layers, index, neuron_count, &payload);
            maai.layers.push(layer_instance);
        }

        maai
    }

    // GET FUNCTIONS

    /// With `get_accuracy()` you will get a float which determines how accurate the network is by the training data reference.
    /// As lower the value is, as more accurate is the network.
    pub fn get_accuracy(&self) -> f64 {
        return self.cost();
    }

    // TOOLS

    /// [WORK IN PROGRESS] Currently `inpsect()` just outputs the `Layer` and `Neurons [per Layer]` count.
    pub fn inspect(&self) {
        inspect(&self.layers);
    }

    // PRIVATE FUNCTIONS

    fn cost(&self) -> f64 {
        let mut v: f64 = 0.0;
        let mut n: f64 = 0.0;
        let output_layer: &Layer = self.layers.last().unwrap();
        output_layer.weights.iter().for_each(|weight| {
            n += weight.index as f64;
            v += (weight.end_neuron.activation - 0.0).sqrt();
        });
        v / n
    }
}

#[allow(dead_code)]
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
