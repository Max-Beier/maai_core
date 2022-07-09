#[path = "./neuron.rs"]
mod neuron;

#[derive(Debug)]
pub struct Weight {
    pub value: f64,
    pub startNeuron: neuron::Neuron,
    pub endNeuron: neuron::Neuron,
    pub layerIndex: i32,
}

impl Weight {
    pub fn new(layerIndex: i32) -> Weight {
        return Weight {
            value: 0.0,
            startNeuron: neuron::Neuron::new(layerIndex - 1),
            endNeuron: neuron::Neuron::new(layerIndex + 1),
            layerIndex: layerIndex,
        };
    }
}
