use crate::core::neuron::Neuron;
use crate::core::weight::Weight;

#[derive(Debug)]
pub struct Layer {
    pub index: u32,
    pub height: u32,
    pub weights: Vec<Weight>,
    pub neurons: Vec<Neuron>,
}

impl Layer {
    pub fn new(layers: &Vec<Layer>, index: u32, neuron_count: u32, payload: &Vec<f64>) -> Layer {
        let mut layer: Layer = Layer {
            index: index,
            height: neuron_count,
            weights: Vec::new(),
            neurons: Vec::new(),
        };

        if index == 0 {
            let payload: &Vec<f64> = payload;
            for input_layer_index in payload {
                let neuron = Neuron {
                    activation: 0.0,
                    index: *input_layer_index as u32,
                    layer_index: index,
                };

                layer.neurons.push(neuron);

                println!("{:?}", layer.neurons);
            }
        }

        layer
    }
}

fn relu(v: f64) -> f64 {
    if v >= 0.0 {
        v
    } else {
        0.01 * v
    }
}
