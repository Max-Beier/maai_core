#[path = "./neuron.rs"]
pub(crate) mod neuron;

#[derive(Debug)]
pub struct Weight {
    pub value: f64,
    pub startNeuron: neuron::Neuron,
    pub endNeuron: neuron::Neuron,
    pub layerIndex: i32,
}
