use crate::core::neuron::Neuron;

#[derive(Debug)]
pub struct Weight {
    pub index: u8,
    pub value: f64,
    pub bias: f64,
    pub start_neuron: Neuron,
    pub end_neuron: Neuron,
    pub layer_index: i32,
}
