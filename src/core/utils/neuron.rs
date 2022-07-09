#[derive(Debug)]
pub struct Neuron {
    pub activation: f64,
    pub sum: f64,
    layerIndex: i32,
}

impl Neuron {
    pub fn new(layerIndex: i32) -> Neuron {
        return Neuron {
            activation: 0.0,
            sum: 0.0,
            layerIndex: layerIndex,
        };
    }
}
