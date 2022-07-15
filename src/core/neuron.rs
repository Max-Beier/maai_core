#[derive(Debug)]
pub struct Neuron {
    pub activation: f64,
    pub index: u32,
    pub layer_index: u32,
    pub weights_from: Vec<u32>,
    pub weights_values: Vec<f64>,
}
