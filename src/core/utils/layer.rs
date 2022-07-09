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
            height: 16,
            neurons: Vec::new(),
            weights: Vec::new(),
            index: index,
        };

        for i in 0..layer.height {
            let neuron = neuron::Neuron::new(index);
            let weight = weight::Weight::new(index);

            layer.neurons.push(neuron);
            layer.weights.push(weight);
        }

        return layer;
    }
}
