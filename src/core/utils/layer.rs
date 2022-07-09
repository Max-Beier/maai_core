#[path = "./neuron.rs"]
mod neuron;

#[path = "./weight.rs"]
mod weight;

#[derive(Debug)]
pub struct Layer {
    pub height: i32,
    pub neurons: Vec<neuron::Neuron>,
    pub weights: Vec<weight::Weight>,
}

impl Layer {
    pub fn new() -> Layer {
        let mut layer = Layer {
            height: 10,
            neurons: Vec::new(),
            weights: Vec::new(),
        };

        for i in 0..layer.height {
            let neuron = neuron::Neuron::new();
            let weight = weight::Weight::new();

            layer.neurons.push(neuron);
            layer.weights.push(weight);
        }

        return layer;
    }
}
