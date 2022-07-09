#[path = "./neuron.rs"]
mod neuron;

#[path = "./weight.rs"]
mod weight;

#[derive(Debug)]
pub struct Layer {
    pub index: i32,
    pub height: i32,
    pub neurons: Vec<neuron::Neuron>,
    pub weights: Vec<weight::Weight>,
}

impl Layer {
    pub fn new(index: i32) -> Layer {
        let mut layer = Layer {
            index: index,
            height: 16,
            neurons: Vec::new(),
            weights: Vec::new(),
        };

        for i in 0..layer.height {
            let neuron = neuron::Neuron::new(index);
            layer.neurons.push(neuron);
        }

        return layer;
    }
}
