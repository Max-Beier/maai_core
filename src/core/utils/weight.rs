use crate::core::utils::neuron::*;

#[derive(Debug)]
pub struct Weight {
    pub index: u8,
    pub value: f64,
    pub bias: f64,
    pub startNeuron: Neuron,
    pub endNeuron: Neuron,
    pub layerIndex: i32,
}
