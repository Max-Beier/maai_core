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
    pub fn new(
        #[allow(unused_variables)] layers: &Vec<Layer>,
        index: u32,
        layer_count: u32,
        neuron_count: u32,
        output_layer_neuron_count: u32,
        payload: &Vec<f64>,
    ) -> Layer {
        let mut layer: Layer = Layer {
            index: index,
            height: neuron_count,
            weights: Vec::new(),
            neurons: Vec::new(),
        };

        // CREATES INPUT LAYER
        if index == 0 {
            let payload: &Vec<f64> = payload;
            for input_layer_index in payload {
                let mut neuron = Neuron {
                    activation: 0.0,
                    index: *input_layer_index as u32,
                    layer_index: index,
                    weights_from: Vec::new(),
                    weights_values: Vec::new(),
                };
                connect_to_network(&mut neuron, layers);
                layer.neurons.push(neuron);
            }
            return layer;
        }

        // CREATES OUTPUT LAYER
        if index == layer_count - 1 {
            for output_layer_index in 0..output_layer_neuron_count {
                let mut neuron = Neuron {
                    activation: 0.0,
                    index: output_layer_index,
                    layer_index: index,
                    weights_from: Vec::new(),
                    weights_values: Vec::new(),
                };
                connect_to_network(&mut neuron, layers);
                layer.neurons.push(neuron);
            }
            return layer;
        }

        // CREATES HIDDEN LAYER
        for hidden_layer_index in 0..neuron_count {
            let mut neuron = Neuron {
                activation: 0.0,
                index: hidden_layer_index,
                layer_index: index,
                weights_from: Vec::new(),
                weights_values: Vec::new(),
            };
            connect_to_network(&mut neuron, layers);
            layer.neurons.push(neuron);
        }
        layer
    }
}

#[allow(dead_code)]
fn relu(v: f64) -> f64 {
    if v >= 0.0 {
        v
    } else {
        0.01 * v
    }
}

fn connect_to_network(neuron: &mut Neuron, layers: &Vec<Layer>) {
    if neuron.layer_index == 0 {
        return;
    }

    for n in layers[neuron.layer_index as usize - 1].neurons.iter() {
        neuron.weights_from.push(n.index);
        neuron.weights_values.push(n.activation);
    }
}
