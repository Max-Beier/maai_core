#[path = "./neuron.rs"]
mod neuron;

#[derive(Debug)]
pub struct Weight {
    pub value: f64,
    pub startNeuron: neuron::Neuron,
    pub endNeuron: neuron::Neuron,
}

impl Weight {
    pub fn new() -> Weight {
        return Weight {
            value: 0.0,
            startNeuron: neuron::Neuron::new(),
            endNeuron: neuron::Neuron::new(),
        };
    }
}
